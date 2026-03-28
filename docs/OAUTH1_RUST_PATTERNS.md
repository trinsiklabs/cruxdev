# OAuth 1.0a Signing Patterns for Rust HTTP Clients

**Purpose:** Implementable reference for building a pure-Rust X/Twitter API client with OAuth 1.0a signing. No Python/tweepy dependency.

**Target:** X API v2 (`POST /2/tweets`, etc.) authenticated via OAuth 1.0a User Context.

---

## Table of Contents

1. [Crate Dependencies](#1-crate-dependencies)
2. [Percent-Encoding (RFC 5849)](#2-percent-encoding-rfc-5849)
3. [Nonce Generation](#3-nonce-generation)
4. [Timestamp Handling](#4-timestamp-handling)
5. [Signature Base String Construction](#5-signature-base-string-construction)
6. [HMAC-SHA1 Signing](#6-hmac-sha1-signing)
7. [Authorization Header Construction](#7-authorization-header-construction)
8. [Full Request Signing with reqwest](#8-full-request-signing-with-reqwest)
9. [Error Handling (401, 403, 429)](#9-error-handling-401-403-429)
10. [Token Refresh Considerations](#10-token-refresh-considerations)
11. [Testing OAuth Signatures](#11-testing-oauth-signatures)
12. [Complete Working Module](#12-complete-working-module)

---

## 1. Crate Dependencies

```toml
[dependencies]
reqwest = { version = "0.12", features = ["rustls-tls", "json"], default-features = false }
hmac = "0.12"
sha1 = "0.10"
base64 = "0.22"
percent-encoding = "2.3"
rand = "0.8"
chrono = "0.4"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
tokio = { version = "1", features = ["full"] }
```

**Why these crates (not `reqwest-oauth1`):** The `reqwest-oauth1` crate wraps `oauth1-request` and adds a layer of abstraction. For a focused X API client, rolling the signing logic directly gives full control over parameter handling, debugging, and the ability to sign arbitrary requests without fighting the wrapper's API surface. The signing logic is ~100 lines of code.

---

## 2. Percent-Encoding (RFC 5849)

OAuth 1.0a requires RFC 5849 Section 3.6 percent-encoding: encode everything except unreserved characters (ALPHA, DIGIT, `-`, `.`, `_`, `~`).

```rust
use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};

/// RFC 5849 Section 3.6 — encode everything except unreserved characters.
/// Unreserved = ALPHA / DIGIT / "-" / "." / "_" / "~"
const OAUTH_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'!')
    .add(b'"')
    .add(b'#')
    .add(b'$')
    .add(b'%')
    .add(b'&')
    .add(b'\'')
    .add(b'(')
    .add(b')')
    .add(b'*')
    .add(b'+')
    .add(b',')
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'<')
    .add(b'=')
    .add(b'>')
    .add(b'?')
    .add(b'@')
    .add(b'[')
    .add(b'\\')
    .add(b']')
    .add(b'^')
    .add(b'`')
    .add(b'{')
    .add(b'|')
    .add(b'}');

fn percent_encode(input: &str) -> String {
    utf8_percent_encode(input, OAUTH_ENCODE_SET).to_string()
}
```

**Critical detail:** The `percent-encoding` crate's built-in sets (like `NON_ALPHANUMERIC`) encode too aggressively — they encode `-`, `.`, `_`, and `~` which OAuth requires to remain unencoded. You must define the set explicitly.

**Verification:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_encode_unreserved_passthrough() {
        // Unreserved characters MUST NOT be encoded
        assert_eq!(percent_encode("abcXYZ019"), "abcXYZ019");
        assert_eq!(percent_encode("-._~"), "-._~");
    }

    #[test]
    fn test_percent_encode_reserved() {
        assert_eq!(percent_encode("Hello Ladies + Gentlemen"),
                   "Hello%20Ladies%20%2B%20Gentlemen");
        assert_eq!(percent_encode("hello@example.com"),
                   "hello%40example.com");
        assert_eq!(percent_encode("/status/1"), "%2Fstatus%2F1");
    }
}
```

---

## 3. Nonce Generation

The nonce is a unique random string per request. X API requires it to be unique across requests but does not enforce a specific format. A 32-character alphanumeric string is standard.

```rust
use rand::Rng;

fn generate_nonce() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
```

**Alternative (faster, no `rand` dependency):** Use `uuid::Uuid::new_v4().to_string().replace("-", "")` if the project already depends on `uuid`. The nonce just needs to be unique; cryptographic randomness is not required by the spec.

---

## 4. Timestamp Handling

OAuth timestamp is seconds since Unix epoch (UTC). Must be reasonably current — X API rejects requests with timestamps more than ~5 minutes off.

```rust
fn generate_timestamp() -> String {
    chrono::Utc::now().timestamp().to_string()
}
```

**Without chrono dependency:**

```rust
fn generate_timestamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock before epoch")
        .as_secs()
        .to_string()
}
```

---

## 5. Signature Base String Construction

The signature base string has exactly this structure:

```
HTTP_METHOD&percent_encode(base_url)&percent_encode(normalized_params)
```

Three components joined by literal `&`.

### 5.1 Collecting Parameters

Collect ALL parameters from these sources:
- OAuth parameters (excluding `oauth_signature`)
- URL query parameters
- Request body parameters (only for `application/x-www-form-urlencoded`)

**Do NOT include:** the request body for `application/json` POST bodies (this is the X API v2 pattern — JSON bodies are not included in the signature base string).

### 5.2 Parameter Normalization

1. Percent-encode each key and value
2. Sort by encoded key (ascending byte order)
3. If keys are identical, sort by encoded value
4. Join each pair with `=`
5. Join all pairs with `&`

```rust
use std::collections::BTreeMap;

struct OAuthParams {
    consumer_key: String,
    nonce: String,
    signature_method: String, // "HMAC-SHA1"
    timestamp: String,
    token: String,
    version: String, // "1.0"
}

fn build_signature_base_string(
    method: &str,
    base_url: &str,
    oauth_params: &OAuthParams,
    query_params: &[(String, String)],
) -> String {
    // Collect all parameters into a BTreeMap for sorted iteration.
    // BTreeMap sorts by key automatically.
    let mut params: BTreeMap<String, String> = BTreeMap::new();

    // OAuth parameters
    params.insert(
        "oauth_consumer_key".to_string(),
        oauth_params.consumer_key.clone(),
    );
    params.insert("oauth_nonce".to_string(), oauth_params.nonce.clone());
    params.insert(
        "oauth_signature_method".to_string(),
        oauth_params.signature_method.clone(),
    );
    params.insert(
        "oauth_timestamp".to_string(),
        oauth_params.timestamp.clone(),
    );
    params.insert("oauth_token".to_string(), oauth_params.token.clone());
    params.insert("oauth_version".to_string(), oauth_params.version.clone());

    // Query string parameters (already decoded — will be re-encoded below)
    for (k, v) in query_params {
        params.insert(k.clone(), v.clone());
    }

    // Normalize: percent-encode keys and values, sort, join
    let mut param_pairs: Vec<(String, String)> = params
        .into_iter()
        .map(|(k, v)| (percent_encode(&k), percent_encode(&v)))
        .collect();
    param_pairs.sort();

    let normalized_params: String = param_pairs
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&");

    // Assemble: METHOD&url&params
    format!(
        "{}&{}&{}",
        method.to_uppercase(),
        percent_encode(base_url),
        percent_encode(&normalized_params)
    )
}
```

### 5.3 Base URL Rules

- Include scheme and host (lowercase)
- Include path
- Exclude query string (those go into parameters)
- Exclude fragment
- Exclude default ports (80 for HTTP, 443 for HTTPS)

For X API, the base URL is always something like `https://api.x.com/2/tweets` — no query string stripping needed for POST endpoints.

---

## 6. HMAC-SHA1 Signing

### 6.1 Signing Key Construction

The signing key is two percent-encoded secrets joined by `&`:

```
percent_encode(consumer_secret) & percent_encode(token_secret)
```

Even if `token_secret` is empty (during request_token phase), the trailing `&` is required.

### 6.2 HMAC-SHA1 Computation

```rust
use hmac::{Hmac, Mac};
use sha1::Sha1;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;

type HmacSha1 = Hmac<Sha1>;

fn sign(
    base_string: &str,
    consumer_secret: &str,
    token_secret: &str,
) -> String {
    // Construct signing key: percent_encode(consumer_secret)&percent_encode(token_secret)
    let signing_key = format!(
        "{}&{}",
        percent_encode(consumer_secret),
        percent_encode(token_secret)
    );

    // HMAC-SHA1
    let mut mac = HmacSha1::new_from_slice(signing_key.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(base_string.as_bytes());
    let result = mac.finalize().into_bytes();

    // Base64 encode the raw bytes
    BASE64.encode(result)
}
```

**Why not `ring`?** The `hmac` + `sha1` crates from RustCrypto are pure Rust, lighter weight, and sufficient for OAuth signing. `ring` is a good choice if the project already uses it for TLS or other crypto, but it pulls in more code. The RustCrypto crates are the idiomatic choice for standalone HMAC.

---

## 7. Authorization Header Construction

The Authorization header uses the `OAuth` scheme with comma-separated key="value" pairs. All values are percent-encoded.

```rust
fn build_auth_header(
    oauth_params: &OAuthParams,
    signature: &str,
) -> String {
    // All values in the header are percent-encoded and double-quoted
    format!(
        "OAuth \
         oauth_consumer_key=\"{}\", \
         oauth_nonce=\"{}\", \
         oauth_signature=\"{}\", \
         oauth_signature_method=\"{}\", \
         oauth_timestamp=\"{}\", \
         oauth_token=\"{}\", \
         oauth_version=\"{}\"",
        percent_encode(&oauth_params.consumer_key),
        percent_encode(&oauth_params.nonce),
        percent_encode(signature),
        percent_encode(&oauth_params.signature_method),
        percent_encode(&oauth_params.timestamp),
        percent_encode(&oauth_params.token),
        percent_encode(&oauth_params.version),
    )
}
```

**Key detail:** The `oauth_signature` value is the base64-encoded HMAC output, which contains `+`, `/`, and `=` characters. These MUST be percent-encoded in the header (e.g., `+` becomes `%2B`).

---

## 8. Full Request Signing with reqwest

### 8.1 Credentials Struct

```rust
#[derive(Clone)]
pub struct OAuthCredentials {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_token: String,
    pub access_token_secret: String,
}

impl OAuthCredentials {
    pub fn from_env() -> Result<Self, OAuthError> {
        Ok(Self {
            consumer_key: std::env::var("X_CONSUMER_KEY")
                .map_err(|_| OAuthError::MissingCredential("X_CONSUMER_KEY"))?,
            consumer_secret: std::env::var("X_CONSUMER_SECRET")
                .map_err(|_| OAuthError::MissingCredential("X_CONSUMER_SECRET"))?,
            access_token: std::env::var("X_ACCESS_TOKEN")
                .map_err(|_| OAuthError::MissingCredential("X_ACCESS_TOKEN"))?,
            access_token_secret: std::env::var("X_ACCESS_TOKEN_SECRET")
                .map_err(|_| OAuthError::MissingCredential("X_ACCESS_TOKEN_SECRET"))?,
        })
    }
}
```

### 8.2 Signed Request Builder

```rust
use reqwest::{Client, Response};

pub struct OAuthClient {
    credentials: OAuthCredentials,
    http: Client,
}

impl OAuthClient {
    pub fn new(credentials: OAuthCredentials) -> Self {
        Self {
            credentials,
            http: Client::new(),
        }
    }

    /// Sign and send a POST request with a JSON body.
    /// JSON body is NOT included in the signature base string per OAuth spec.
    pub async fn post_json(
        &self,
        url: &str,
        body: &serde_json::Value,
    ) -> Result<Response, OAuthError> {
        let auth_header = self.sign("POST", url, &[])?;

        let response = self
            .http
            .post(url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .map_err(OAuthError::Http)?;

        Self::check_response(response).await
    }

    /// Sign and send a GET request with query parameters.
    /// Query parameters ARE included in the signature base string.
    pub async fn get(
        &self,
        url: &str,
        query_params: &[(String, String)],
    ) -> Result<Response, OAuthError> {
        let auth_header = self.sign("GET", url, query_params)?;

        let mut request = self.http.get(url).header("Authorization", auth_header);

        if !query_params.is_empty() {
            request = request.query(query_params);
        }

        let response = request.send().await.map_err(OAuthError::Http)?;

        Self::check_response(response).await
    }

    /// Build the OAuth Authorization header for a request.
    fn sign(
        &self,
        method: &str,
        url: &str,
        query_params: &[(String, String)],
    ) -> Result<String, OAuthError> {
        let oauth_params = OAuthParams {
            consumer_key: self.credentials.consumer_key.clone(),
            nonce: generate_nonce(),
            signature_method: "HMAC-SHA1".to_string(),
            timestamp: generate_timestamp(),
            token: self.credentials.access_token.clone(),
            version: "1.0".to_string(),
        };

        let base_string =
            build_signature_base_string(method, url, &oauth_params, query_params);

        let signature = sign(
            &base_string,
            &self.credentials.consumer_secret,
            &self.credentials.access_token_secret,
        );

        Ok(build_auth_header(&oauth_params, &signature))
    }

    async fn check_response(response: Response) -> Result<Response, OAuthError> {
        let status = response.status();
        if status.is_success() {
            return Ok(response);
        }
        match status.as_u16() {
            401 => Err(OAuthError::Unauthorized),
            403 => Err(OAuthError::Forbidden),
            429 => {
                let reset = response
                    .headers()
                    .get("x-rate-limit-reset")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<i64>().ok());
                Err(OAuthError::RateLimited { reset_at: reset })
            }
            _ => {
                let body = response.text().await.unwrap_or_default();
                Err(OAuthError::ApiError {
                    status: status.as_u16(),
                    body,
                })
            }
        }
    }
}
```

### 8.3 Usage: Post a Tweet

```rust
pub async fn post_tweet(client: &OAuthClient, text: &str) -> Result<Response, OAuthError> {
    let body = serde_json::json!({ "text": text });
    client.post_json("https://api.x.com/2/tweets", &body).await
}
```

---

## 9. Error Handling (401, 403, 429)

```rust
#[derive(Debug, thiserror::Error)]
pub enum OAuthError {
    #[error("missing credential: {0}")]
    MissingCredential(&'static str),

    #[error("HTTP error: {0}")]
    Http(reqwest::Error),

    #[error("401 Unauthorized — check consumer key/secret and access token/secret")]
    Unauthorized,

    #[error("403 Forbidden — app may lack write permissions or user revoked access")]
    Forbidden,

    #[error("429 Rate Limited — reset at {reset_at:?}")]
    RateLimited { reset_at: Option<i64> },

    #[error("API error {status}: {body}")]
    ApiError { status: u16, body: String },
}
```

### 9.1 Rate Limit Strategy

X API returns three headers for rate limit tracking:

| Header | Meaning |
|--------|---------|
| `x-rate-limit-limit` | Max requests per 15-min window |
| `x-rate-limit-remaining` | Requests left in current window |
| `x-rate-limit-reset` | UTC epoch seconds when window resets |

**Recommended approach: respect the reset time, not exponential backoff.**

```rust
impl OAuthClient {
    /// Send a request with automatic rate-limit retry (single retry).
    pub async fn post_json_with_retry(
        &self,
        url: &str,
        body: &serde_json::Value,
    ) -> Result<Response, OAuthError> {
        match self.post_json(url, body).await {
            Err(OAuthError::RateLimited { reset_at: Some(reset) }) => {
                let now = chrono::Utc::now().timestamp();
                let wait_secs = (reset - now).max(1).min(900); // cap at 15 min
                tokio::time::sleep(std::time::Duration::from_secs(wait_secs as u64)).await;
                // Single retry — if still rate-limited, return the error
                self.post_json(url, body).await
            }
            other => other,
        }
    }
}
```

### 9.2 Common 401 Causes

- Clock skew: timestamp too far from server time (sync NTP)
- Wrong signing key: consumer_secret and token_secret swapped
- Signature base string mismatch: wrong URL, missing parameters, wrong encoding
- Tokens revoked or expired

### 9.3 Common 403 Causes

- App does not have "Read and Write" permissions (check X Developer Portal)
- User suspended or restricted
- Trying to post duplicate content
- Free tier API plan may not include write access (requires Basic or higher)

---

## 10. Token Refresh Considerations

OAuth 1.0a tokens (used by X API) do **not expire** under normal circumstances. They persist until:

- The user explicitly revokes app access
- The app owner regenerates keys in the Developer Portal
- The user's account is suspended

**There is no refresh token flow in OAuth 1.0a.** This is different from OAuth 2.0 with PKCE (which X API also supports but requires refresh handling).

If you receive a 401 after previously working requests:
1. Verify system clock is accurate
2. Check if user revoked access (requires re-authorization via 3-legged OAuth flow)
3. Check if app keys were regenerated

### 10.1 Three-Legged OAuth Flow (for obtaining tokens)

If you need to obtain access tokens programmatically (not just use pre-generated ones from the Developer Portal), the 3-legged flow is:

1. `POST https://api.x.com/oauth/request_token` (signed with consumer key only, empty token secret)
2. Redirect user to `https://api.x.com/oauth/authorize?oauth_token={request_token}`
3. User approves, X redirects to callback URL with `oauth_token` and `oauth_verifier`
4. `POST https://api.x.com/oauth/access_token` with the verifier to get permanent access token + secret

This is out of scope for a headless bot (use Developer Portal tokens), but the signing code handles it — just pass an empty string for `token_secret` in step 1.

---

## 11. Testing OAuth Signatures

### 11.1 Known Test Vector from X API Documentation

This is the canonical test case from [X's official documentation](https://developer.x.com/en/docs/authentication/oauth-1-0a/creating-a-signature):

```rust
#[cfg(test)]
mod tests {
    use super::*;

    /// Test vector from X API documentation:
    /// https://developer.x.com/en/docs/authentication/oauth-1-0a/creating-a-signature
    #[test]
    fn test_x_api_signature_vector() {
        let oauth_params = OAuthParams {
            consumer_key: "xvz1evFS4wEEPTGEFPHBog".to_string(),
            nonce: "kYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg".to_string(),
            signature_method: "HMAC-SHA1".to_string(),
            timestamp: "1318622958".to_string(),
            token: "370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb".to_string(),
            version: "1.0".to_string(),
        };

        let query_params = vec![
            ("include_entities".to_string(), "true".to_string()),
            (
                "status".to_string(),
                "Hello Ladies + Gentlemen, a signed OAuth request!".to_string(),
            ),
        ];

        let base_string = build_signature_base_string(
            "POST",
            "https://api.twitter.com/1.1/statuses/update.json",
            &oauth_params,
            &query_params,
        );

        // Expected base string from X docs
        let expected_base = "POST&https%3A%2F%2Fapi.twitter.com%2F1.1%2Fstatuses%2Fupdate.json&\
            include_entities%3Dtrue%26\
            oauth_consumer_key%3Dxvz1evFS4wEEPTGEFPHBog%26\
            oauth_nonce%3DkYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg%26\
            oauth_signature_method%3DHMAC-SHA1%26\
            oauth_timestamp%3D1318622958%26\
            oauth_token%3D370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb%26\
            oauth_version%3D1.0%26\
            status%3DHello%2520Ladies%2520%252B%2520Gentlemen%252C%2520a%2520signed%2520OAuth%2520request%2521";

        assert_eq!(base_string, expected_base);

        // Sign it
        let consumer_secret = "kAcSOqF21Fu85e7zjz7ZN2U4ZRhfV3WpwPAoE3Z7kBw";
        let token_secret = "LswwdoUaIvS8ltyTt5jkRh4J50vUPVVHtR2YPi5kE";

        let signature = sign(&base_string, consumer_secret, token_secret);
        let expected_signature = "hCtSmYh+iHYCEqBWrE7C7hYmtUk=";
        assert_eq!(signature, expected_signature);
    }

    /// Verify that the Authorization header is well-formed
    #[test]
    fn test_auth_header_format() {
        let oauth_params = OAuthParams {
            consumer_key: "test_key".to_string(),
            nonce: "testnonce123".to_string(),
            signature_method: "HMAC-SHA1".to_string(),
            timestamp: "1234567890".to_string(),
            token: "test_token".to_string(),
            version: "1.0".to_string(),
        };

        let header = build_auth_header(&oauth_params, "sig+nature/test=");
        assert!(header.starts_with("OAuth "));
        assert!(header.contains("oauth_consumer_key=\"test_key\""));
        assert!(header.contains("oauth_signature=\"sig%2Bnature%2Ftest%3D\""));
        assert!(header.contains("oauth_nonce=\"testnonce123\""));
    }

    /// Verify signing key construction with empty token secret
    #[test]
    fn test_sign_empty_token_secret() {
        // During request_token phase, token_secret is empty
        // The trailing & is still required
        let base = "GET&http%3A%2F%2Fexample.com&oauth_test%3Dvalue";
        let sig = sign(base, "consumer_secret", "");
        // Just verify it produces a non-empty base64 string
        assert!(!sig.is_empty());
        assert!(sig.ends_with('=') || sig.chars().all(|c| c.is_alphanumeric() || c == '+' || c == '/'));
    }
}
```

### 11.2 RFC 5849 Section 1.2 Test Vector

```rust
#[test]
fn test_rfc5849_example() {
    // From RFC 5849 Section 1.2
    // Consumer key:     dpf43f3p2l4k3l03
    // Consumer secret:  kd94hf93k423kf44
    // Token:            nnch734d00sl2jdk
    // Token secret:     pfkkdhi9sl3r4s00

    let oauth_params = OAuthParams {
        consumer_key: "dpf43f3p2l4k3l03".to_string(),
        nonce: "kllo9940pd9333jh".to_string(),
        signature_method: "HMAC-SHA1".to_string(),
        timestamp: "1191242096".to_string(),
        token: "nnch734d00sl2jdk".to_string(),
        version: "1.0".to_string(),
    };

    let query_params = vec![
        ("file".to_string(), "vacation.jpg".to_string()),
        ("size".to_string(), "original".to_string()),
    ];

    let base_string = build_signature_base_string(
        "GET",
        "http://photos.example.net/photos",
        &oauth_params,
        &query_params,
    );

    let signature = sign(&base_string, "kd94hf93k423kf44", "pfkkdhi9sl3r4s00");
    assert_eq!(signature, "tR3+Ty81lMeYAr/Fid0kMTYa/WM=");
}
```

### 11.3 Testing Strategy

1. **Unit test signing** with known vectors (deterministic nonce + timestamp)
2. **Unit test percent-encoding** with edge cases (spaces, unicode, empty strings)
3. **Integration test** with a real X API call (behind a feature flag)

```rust
/// Integration test — only runs with `cargo test --features integration`
#[cfg(feature = "integration")]
#[tokio::test]
async fn test_real_tweet() {
    let creds = OAuthCredentials::from_env().expect("X API credentials in env");
    let client = OAuthClient::new(creds);

    // Use the users/me endpoint (read-only, low rate limit impact)
    let response = client
        .get("https://api.x.com/2/users/me", &[])
        .await
        .expect("API call should succeed");

    assert_eq!(response.status(), 200);
}
```

---

## 12. Complete Working Module

Below is the full, copy-pasteable module. All functions from sections 2-9 assembled into a single coherent unit.

```rust
//! oauth.rs — OAuth 1.0a signing for X API v2
//!
//! Usage:
//!   let creds = OAuthCredentials::from_env()?;
//!   let client = OAuthClient::new(creds);
//!   let resp = client.post_json("https://api.x.com/2/tweets", &json!({"text": "hi"})).await?;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use hmac::{Hmac, Mac};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use rand::Rng;
use reqwest::{Client, Response};
use sha1::Sha1;
use std::collections::BTreeMap;

type HmacSha1 = Hmac<Sha1>;

// ---------------------------------------------------------------------------
// Percent-encoding (RFC 5849 Section 3.6)
// ---------------------------------------------------------------------------

const OAUTH_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ').add(b'!').add(b'"').add(b'#').add(b'$').add(b'%')
    .add(b'&').add(b'\'').add(b'(').add(b')').add(b'*').add(b'+')
    .add(b',').add(b'/').add(b':').add(b';').add(b'<').add(b'=')
    .add(b'>').add(b'?').add(b'@').add(b'[').add(b'\\').add(b']')
    .add(b'^').add(b'`').add(b'{').add(b'|').add(b'}');

fn percent_encode(input: &str) -> String {
    utf8_percent_encode(input, OAUTH_ENCODE_SET).to_string()
}

// ---------------------------------------------------------------------------
// Nonce + Timestamp
// ---------------------------------------------------------------------------

fn generate_nonce() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    (0..32)
        .map(|_| CHARSET[rng.gen_range(0..CHARSET.len())] as char)
        .collect()
}

fn generate_timestamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock before epoch")
        .as_secs()
        .to_string()
}

// ---------------------------------------------------------------------------
// Signature base string (RFC 5849 Section 3.4.1)
// ---------------------------------------------------------------------------

struct OAuthParams {
    consumer_key: String,
    nonce: String,
    signature_method: String,
    timestamp: String,
    token: String,
    version: String,
}

fn build_signature_base_string(
    method: &str,
    base_url: &str,
    oauth_params: &OAuthParams,
    query_params: &[(String, String)],
) -> String {
    let mut params: BTreeMap<String, String> = BTreeMap::new();

    params.insert("oauth_consumer_key".into(), oauth_params.consumer_key.clone());
    params.insert("oauth_nonce".into(), oauth_params.nonce.clone());
    params.insert("oauth_signature_method".into(), oauth_params.signature_method.clone());
    params.insert("oauth_timestamp".into(), oauth_params.timestamp.clone());
    params.insert("oauth_token".into(), oauth_params.token.clone());
    params.insert("oauth_version".into(), oauth_params.version.clone());

    for (k, v) in query_params {
        params.insert(k.clone(), v.clone());
    }

    let mut pairs: Vec<(String, String)> = params
        .into_iter()
        .map(|(k, v)| (percent_encode(&k), percent_encode(&v)))
        .collect();
    pairs.sort();

    let normalized: String = pairs
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("&");

    format!(
        "{}&{}&{}",
        method.to_uppercase(),
        percent_encode(base_url),
        percent_encode(&normalized)
    )
}

// ---------------------------------------------------------------------------
// HMAC-SHA1 signing (RFC 5849 Section 3.4.2)
// ---------------------------------------------------------------------------

fn sign(base_string: &str, consumer_secret: &str, token_secret: &str) -> String {
    let signing_key = format!(
        "{}&{}",
        percent_encode(consumer_secret),
        percent_encode(token_secret)
    );
    let mut mac =
        HmacSha1::new_from_slice(signing_key.as_bytes()).expect("HMAC accepts any key size");
    mac.update(base_string.as_bytes());
    BASE64.encode(mac.finalize().into_bytes())
}

// ---------------------------------------------------------------------------
// Authorization header (RFC 5849 Section 3.5.1)
// ---------------------------------------------------------------------------

fn build_auth_header(oauth_params: &OAuthParams, signature: &str) -> String {
    format!(
        "OAuth \
         oauth_consumer_key=\"{}\", \
         oauth_nonce=\"{}\", \
         oauth_signature=\"{}\", \
         oauth_signature_method=\"{}\", \
         oauth_timestamp=\"{}\", \
         oauth_token=\"{}\", \
         oauth_version=\"{}\"",
        percent_encode(&oauth_params.consumer_key),
        percent_encode(&oauth_params.nonce),
        percent_encode(signature),
        percent_encode(&oauth_params.signature_method),
        percent_encode(&oauth_params.timestamp),
        percent_encode(&oauth_params.token),
        percent_encode(&oauth_params.version),
    )
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum OAuthError {
    #[error("missing credential: {0}")]
    MissingCredential(&'static str),

    #[error("HTTP error: {0}")]
    Http(reqwest::Error),

    #[error("401 Unauthorized — check consumer key/secret and access token/secret")]
    Unauthorized,

    #[error("403 Forbidden — app may lack write permissions or user revoked access")]
    Forbidden,

    #[error("429 Rate Limited — reset at {reset_at:?}")]
    RateLimited { reset_at: Option<i64> },

    #[error("API error {status}: {body}")]
    ApiError { status: u16, body: String },
}

// ---------------------------------------------------------------------------
// Credentials
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct OAuthCredentials {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_token: String,
    pub access_token_secret: String,
}

impl OAuthCredentials {
    pub fn from_env() -> Result<Self, OAuthError> {
        Ok(Self {
            consumer_key: std::env::var("X_CONSUMER_KEY")
                .map_err(|_| OAuthError::MissingCredential("X_CONSUMER_KEY"))?,
            consumer_secret: std::env::var("X_CONSUMER_SECRET")
                .map_err(|_| OAuthError::MissingCredential("X_CONSUMER_SECRET"))?,
            access_token: std::env::var("X_ACCESS_TOKEN")
                .map_err(|_| OAuthError::MissingCredential("X_ACCESS_TOKEN"))?,
            access_token_secret: std::env::var("X_ACCESS_TOKEN_SECRET")
                .map_err(|_| OAuthError::MissingCredential("X_ACCESS_TOKEN_SECRET"))?,
        })
    }
}

// ---------------------------------------------------------------------------
// Client
// ---------------------------------------------------------------------------

pub struct OAuthClient {
    credentials: OAuthCredentials,
    http: Client,
}

impl OAuthClient {
    pub fn new(credentials: OAuthCredentials) -> Self {
        Self {
            credentials,
            http: Client::new(),
        }
    }

    /// POST with JSON body. JSON body is NOT included in signature base string.
    pub async fn post_json(
        &self,
        url: &str,
        body: &serde_json::Value,
    ) -> Result<Response, OAuthError> {
        let auth_header = self.build_authorization("POST", url, &[])?;
        let response = self
            .http
            .post(url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .map_err(OAuthError::Http)?;
        Self::check_response(response).await
    }

    /// GET with query parameters. Query params ARE included in signature.
    pub async fn get(
        &self,
        url: &str,
        query_params: &[(String, String)],
    ) -> Result<Response, OAuthError> {
        let auth_header = self.build_authorization("GET", url, query_params)?;
        let mut req = self.http.get(url).header("Authorization", auth_header);
        if !query_params.is_empty() {
            req = req.query(query_params);
        }
        let response = req.send().await.map_err(OAuthError::Http)?;
        Self::check_response(response).await
    }

    /// DELETE request (e.g., DELETE /2/tweets/:id).
    pub async fn delete(&self, url: &str) -> Result<Response, OAuthError> {
        let auth_header = self.build_authorization("DELETE", url, &[])?;
        let response = self
            .http
            .delete(url)
            .header("Authorization", auth_header)
            .send()
            .await
            .map_err(OAuthError::Http)?;
        Self::check_response(response).await
    }

    fn build_authorization(
        &self,
        method: &str,
        url: &str,
        query_params: &[(String, String)],
    ) -> Result<String, OAuthError> {
        let oauth_params = OAuthParams {
            consumer_key: self.credentials.consumer_key.clone(),
            nonce: generate_nonce(),
            signature_method: "HMAC-SHA1".to_string(),
            timestamp: generate_timestamp(),
            token: self.credentials.access_token.clone(),
            version: "1.0".to_string(),
        };
        let base_string =
            build_signature_base_string(method, url, &oauth_params, query_params);
        let signature = sign(
            &base_string,
            &self.credentials.consumer_secret,
            &self.credentials.access_token_secret,
        );
        Ok(build_auth_header(&oauth_params, &signature))
    }

    async fn check_response(response: Response) -> Result<Response, OAuthError> {
        let status = response.status();
        if status.is_success() {
            return Ok(response);
        }
        match status.as_u16() {
            401 => Err(OAuthError::Unauthorized),
            403 => Err(OAuthError::Forbidden),
            429 => {
                let reset = response
                    .headers()
                    .get("x-rate-limit-reset")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<i64>().ok());
                Err(OAuthError::RateLimited { reset_at: reset })
            }
            _ => {
                let body = response.text().await.unwrap_or_default();
                Err(OAuthError::ApiError {
                    status: status.as_u16(),
                    body,
                })
            }
        }
    }
}
```

---

## References

- [RFC 5849: The OAuth 1.0 Protocol](https://www.rfc-editor.org/rfc/rfc5849) — Canonical specification
- [X API: Creating a Signature](https://developer.x.com/en/docs/authentication/oauth-1-0a/creating-a-signature) — Official test vector and walkthrough
- [X API Rate Limits](https://docs.x.com/x-api/fundamentals/rate-limits) — Rate limit windows and headers
- [X API v2 Authentication Mapping](https://docs.x.com/fundamentals/authentication/guides/v2-authentication-mapping) — Which endpoints support OAuth 1.0a
- [hmac crate](https://docs.rs/hmac) — RustCrypto HMAC implementation
- [sha1 crate](https://docs.rs/sha1) — RustCrypto SHA-1
- [percent-encoding crate](https://docs.rs/percent-encoding) — Servo's percent-encoding library
- [reqwest-oauth1 crate](https://docs.rs/reqwest-oauth1/latest/reqwest_oauth1/) — Alternative: wrapper crate (not used here, but available)
