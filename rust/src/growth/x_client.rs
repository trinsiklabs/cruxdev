//! X/Twitter API client — pure Rust OAuth 1.0a signing.
//!
//! No Python/tweepy dependency. Signs requests using HMAC-SHA1.

use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

/// Response from posting a tweet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TweetResponse {
    pub id: String,
    pub text: String,
}

/// X API rate limit info from response headers.
#[derive(Debug, Clone)]
pub struct RateLimitInfo {
    pub remaining: u32,
    pub reset_at: u64,
}

/// Post a tweet using OAuth 1.0a.
pub async fn post_tweet(text: &str) -> Result<TweetResponse, String> {
    let (consumer_key, consumer_secret, token, token_secret) = read_credentials()?;

    let url = "https://api.x.com/2/tweets";
    let method = "POST";
    let body = serde_json::json!({ "text": text });

    let auth_header = build_oauth_header(
        method, url, &consumer_key, &consumer_secret, &token, &token_secret,
    );

    let client = reqwest::Client::new();
    let resp = client.post(url)
        .header("Authorization", &auth_header)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("request failed: {e}"))?;

    let status = resp.status();
    let body_text = resp.text().await.map_err(|e| format!("read body: {e}"))?;

    if status.is_success() {
        let parsed: serde_json::Value = serde_json::from_str(&body_text)
            .map_err(|e| format!("parse response: {e}"))?;
        let data = parsed.get("data").ok_or("no data field in response")?;
        Ok(TweetResponse {
            id: data.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            text: data.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        })
    } else if status.as_u16() == 429 {
        Err(format!("429 Rate Limited: {body_text}"))
    } else if status.as_u16() == 401 {
        Err(format!("401 Unauthorized: check X_CLIENT_ID/X_CLIENT_SECRET/X_OAUTH_TOKEN/X_OAUTH_TOKEN_SECRET"))
    } else if status.as_u16() == 403 {
        Err(format!("403 Forbidden: check app permissions (need Read+Write). {body_text}"))
    } else {
        Err(format!("HTTP {}: {body_text}", status.as_u16()))
    }
}

/// Delete a tweet using OAuth 1.0a.
pub async fn delete_tweet(tweet_id: &str) -> Result<(), String> {
    let (consumer_key, consumer_secret, token, token_secret) = read_credentials()?;

    let url = format!("https://api.x.com/2/tweets/{tweet_id}");
    let method = "DELETE";

    let auth_header = build_oauth_header(
        method, &url, &consumer_key, &consumer_secret, &token, &token_secret,
    );

    let client = reqwest::Client::new();
    let resp = client.delete(&url)
        .header("Authorization", &auth_header)
        .send()
        .await
        .map_err(|e| format!("request failed: {e}"))?;

    if resp.status().is_success() {
        Ok(())
    } else {
        let body = resp.text().await.unwrap_or_default();
        Err(format!("delete failed: {body}"))
    }
}

// ── OAuth 1.0a signing ──────────────────────────────────────────

fn build_oauth_header(
    method: &str,
    url: &str,
    consumer_key: &str,
    consumer_secret: &str,
    token: &str,
    token_secret: &str,
) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();

    let nonce: String = (0..32)
        .map(|_| {
            let idx = (rand_byte() % 36) as usize;
            b"abcdefghijklmnopqrstuvwxyz0123456789"[idx] as char
        })
        .collect();

    let mut params = BTreeMap::new();
    params.insert("oauth_consumer_key", consumer_key.to_string());
    params.insert("oauth_nonce", nonce.clone());
    params.insert("oauth_signature_method", "HMAC-SHA1".to_string());
    params.insert("oauth_timestamp", timestamp.clone());
    params.insert("oauth_token", token.to_string());
    params.insert("oauth_version", "1.0".to_string());

    // Build signature base string
    let param_string: String = params.iter()
        .map(|(k, v)| format!("{}={}", percent_encode(k), percent_encode(v)))
        .collect::<Vec<_>>()
        .join("&");

    let base_string = format!(
        "{}&{}&{}",
        method.to_uppercase(),
        percent_encode(url),
        percent_encode(&param_string)
    );

    // Sign with HMAC-SHA1
    let signing_key = format!(
        "{}&{}",
        percent_encode(consumer_secret),
        percent_encode(token_secret)
    );

    let signature = hmac_sha1(signing_key.as_bytes(), base_string.as_bytes());
    let signature_b64 = base64_encode(&signature);

    // Build header
    format!(
        "OAuth oauth_consumer_key=\"{}\", oauth_nonce=\"{}\", oauth_signature=\"{}\", oauth_signature_method=\"HMAC-SHA1\", oauth_timestamp=\"{}\", oauth_token=\"{}\", oauth_version=\"1.0\"",
        percent_encode(consumer_key),
        percent_encode(&nonce),
        percent_encode(&signature_b64),
        percent_encode(&timestamp),
        percent_encode(token),
    )
}

fn percent_encode(s: &str) -> String {
    let mut result = String::new();
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                result.push(byte as char);
            }
            _ => {
                result.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    result
}

fn hmac_sha1(key: &[u8], message: &[u8]) -> [u8; 20] {
    // HMAC-SHA1 per RFC 2104
    let block_size = 64;
    let mut key_padded = vec![0u8; block_size];

    if key.len() > block_size {
        let hash = sha1(key);
        key_padded[..20].copy_from_slice(&hash);
    } else {
        key_padded[..key.len()].copy_from_slice(key);
    }

    let mut ipad = vec![0x36u8; block_size];
    let mut opad = vec![0x5cu8; block_size];
    for i in 0..block_size {
        ipad[i] ^= key_padded[i];
        opad[i] ^= key_padded[i];
    }

    let mut inner = ipad;
    inner.extend_from_slice(message);
    let inner_hash = sha1(&inner);

    let mut outer = opad;
    outer.extend_from_slice(&inner_hash);
    sha1(&outer)
}

fn sha1(data: &[u8]) -> [u8; 20] {
    // SHA-1 implementation (for OAuth signing only — not security-critical)
    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xEFCDAB89;
    let mut h2: u32 = 0x98BADCFE;
    let mut h3: u32 = 0x10325476;
    let mut h4: u32 = 0xC3D2E1F0;

    let bit_len = (data.len() as u64) * 8;
    let mut padded = data.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in padded.chunks(64) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[i*4], chunk[i*4+1], chunk[i*4+2], chunk[i*4+3]]);
        }
        for i in 16..80 {
            w[i] = (w[i-3] ^ w[i-8] ^ w[i-14] ^ w[i-16]).rotate_left(1);
        }

        let (mut a, mut b, mut c, mut d, mut e) = (h0, h1, h2, h3, h4);

        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5A827999u32),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1u32),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDCu32),
                _ => (b ^ c ^ d, 0xCA62C1D6u32),
            };

            let temp = a.rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[i]);
            e = d; d = c; c = b.rotate_left(30); b = a; a = temp;
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    let mut result = [0u8; 20];
    result[0..4].copy_from_slice(&h0.to_be_bytes());
    result[4..8].copy_from_slice(&h1.to_be_bytes());
    result[8..12].copy_from_slice(&h2.to_be_bytes());
    result[12..16].copy_from_slice(&h3.to_be_bytes());
    result[16..20].copy_from_slice(&h4.to_be_bytes());
    result
}

fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;

        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

fn rand_byte() -> u8 {
    // Simple random byte from system time entropy
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (t ^ (t >> 8) ^ (t >> 16)) as u8
}

fn read_credentials() -> Result<(String, String, String, String), String> {
    let consumer_key = std::env::var("X_CLIENT_ID")
        .map_err(|_| "X_CLIENT_ID not set")?;
    let consumer_secret = std::env::var("X_CLIENT_SECRET")
        .map_err(|_| "X_CLIENT_SECRET not set")?;
    let token = std::env::var("X_OAUTH_TOKEN")
        .map_err(|_| "X_OAUTH_TOKEN not set")?;
    let token_secret = std::env::var("X_OAUTH_TOKEN_SECRET")
        .map_err(|_| "X_OAUTH_TOKEN_SECRET not set")?;
    Ok((consumer_key, consumer_secret, token, token_secret))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_encode_preserves_unreserved() {
        assert_eq!(percent_encode("abc-._~"), "abc-._~");
    }

    #[test]
    fn test_percent_encode_encodes_special() {
        assert_eq!(percent_encode("hello world"), "hello%20world");
        assert_eq!(percent_encode("a&b=c"), "a%26b%3Dc");
    }

    #[test]
    fn test_sha1_empty() {
        let hash = sha1(b"");
        let hex: String = hash.iter().map(|b| format!("{:02x}", b)).collect();
        assert_eq!(hex, "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    }

    #[test]
    fn test_sha1_hello() {
        let hash = sha1(b"hello");
        let hex: String = hash.iter().map(|b| format!("{:02x}", b)).collect();
        assert_eq!(hex, "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
    }

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode(b"hello"), "aGVsbG8=");
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"a"), "YQ==");
    }

    #[test]
    fn test_oauth_header_format() {
        let header = build_oauth_header(
            "POST", "https://api.x.com/2/tweets",
            "test_key", "test_secret", "test_token", "test_token_secret",
        );
        assert!(header.starts_with("OAuth "));
        assert!(header.contains("oauth_consumer_key="));
        assert!(header.contains("oauth_signature="));
        assert!(header.contains("oauth_nonce="));
    }

    #[test]
    fn test_hmac_sha1_rfc_vector() {
        // RFC 2104 test vector
        let key = b"Jefe";
        let data = b"what do ya want for nothing?";
        let result = hmac_sha1(key, data);
        let hex: String = result.iter().map(|b| format!("{:02x}", b)).collect();
        assert_eq!(hex, "effcdf6ae5eb2fa2d27416d5f184df9c259a7c79");
    }
}
