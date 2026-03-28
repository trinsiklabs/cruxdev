//! GTV verification cache with TTL per check type.
//!
//! Don't re-verify what was just verified. Cache results with type-specific TTLs.

use std::collections::HashMap;
use std::fs;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::ClaimType;

/// TTL in seconds by claim type.
pub fn ttl_for_type(claim_type: &ClaimType) -> i64 {
    match claim_type {
        ClaimType::Path => 5 * 60,        // 5 minutes — files change during convergence
        ClaimType::Numeric => 10 * 60,     // 10 minutes — stats change when code changes
        ClaimType::Url => 60 * 60,         // 1 hour — websites don't go down frequently
        ClaimType::Feature => 30 * 60,     // 30 minutes — code changes during convergence
        ClaimType::Status => 30 * 60,      // 30 minutes — services are stable short-term
    }
}

/// A cached GTV result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub claim_key: String,
    pub verified: bool,
    pub actual_value: Option<String>,
    pub message: String,
    pub verified_at: String,
    pub ttl_secs: i64,
}

impl CacheEntry {
    pub fn is_expired(&self) -> bool {
        let verified_at = DateTime::parse_from_rfc3339(&self.verified_at)
            .map(|t| t.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        let elapsed = Utc::now().signed_duration_since(verified_at);
        elapsed.num_seconds() > self.ttl_secs
    }
}

/// GTV cache backed by a JSON file.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GtvCache {
    pub entries: HashMap<String, CacheEntry>,
}

impl GtvCache {
    /// Load cache from file, or return empty cache.
    pub fn load(path: &str) -> Self {
        fs::read_to_string(path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    }

    /// Save cache to file (atomic write).
    pub fn save(&self, path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self).map_err(|e| format!("serialize: {e}"))?;
        let tmp = format!("{path}.tmp");
        fs::write(&tmp, &json).map_err(|e| format!("write: {e}"))?;
        fs::rename(&tmp, path).map_err(|e| format!("rename: {e}"))?;
        Ok(())
    }

    /// Generate a cache key for a claim.
    pub fn key(claim_type: &ClaimType, value: &str) -> String {
        format!("{:?}:{}", claim_type, value)
    }

    /// Get a cached result if it exists and is not expired.
    pub fn get(&self, claim_type: &ClaimType, value: &str) -> Option<&CacheEntry> {
        let key = Self::key(claim_type, value);
        self.entries.get(&key).filter(|e| !e.is_expired())
    }

    /// Store a verification result in the cache.
    pub fn put(&mut self, claim_type: &ClaimType, value: &str, verified: bool, actual_value: Option<String>, message: String) {
        let key = Self::key(claim_type, value);
        self.entries.insert(key.clone(), CacheEntry {
            claim_key: key,
            verified,
            actual_value,
            message,
            verified_at: Utc::now().to_rfc3339(),
            ttl_secs: ttl_for_type(claim_type),
        });
    }

    /// Remove expired entries.
    pub fn prune(&mut self) -> usize {
        let before = self.entries.len();
        self.entries.retain(|_, e| !e.is_expired());
        before - self.entries.len()
    }

    /// Invalidate all entries (force re-verification).
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Invalidate entries matching a claim type.
    pub fn invalidate_type(&mut self, claim_type: &ClaimType) {
        let prefix = format!("{:?}:", claim_type);
        self.entries.retain(|k, _| !k.starts_with(&prefix));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gtv_cache_stores_and_retrieves() {
        let mut cache = GtvCache::default();
        cache.put(&ClaimType::Path, "src/main.rs", true, None, "exists".into());

        let entry = cache.get(&ClaimType::Path, "src/main.rs");
        assert!(entry.is_some());
        assert!(entry.unwrap().verified);
    }

    #[test]
    fn test_gtv_cache_ttl_expiry() {
        let mut cache = GtvCache::default();
        let key = GtvCache::key(&ClaimType::Path, "old.rs");
        cache.entries.insert(key.clone(), CacheEntry {
            claim_key: key,
            verified: true,
            actual_value: None,
            message: "old".into(),
            verified_at: "2020-01-01T00:00:00Z".into(), // ancient
            ttl_secs: 300,
        });

        // Should return None — expired
        assert!(cache.get(&ClaimType::Path, "old.rs").is_none());
    }

    #[test]
    fn test_gtv_cache_prune() {
        let mut cache = GtvCache::default();
        let key = GtvCache::key(&ClaimType::Url, "https://old.com");
        cache.entries.insert(key.clone(), CacheEntry {
            claim_key: key,
            verified: true,
            actual_value: None,
            message: "old".into(),
            verified_at: "2020-01-01T00:00:00Z".into(),
            ttl_secs: 3600,
        });
        cache.put(&ClaimType::Path, "fresh.rs", true, None, "fresh".into());

        let pruned = cache.prune();
        assert_eq!(pruned, 1);
        assert_eq!(cache.entries.len(), 1);
    }

    #[test]
    fn test_gtv_cache_persistence_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("gtv_cache.json");
        let path_str = path.to_str().unwrap();

        let mut cache = GtvCache::default();
        cache.put(&ClaimType::Numeric, "485 tests", true, Some("485".into()), "verified".into());
        cache.save(path_str).unwrap();

        let loaded = GtvCache::load(path_str);
        assert_eq!(loaded.entries.len(), 1);
        let entry = loaded.get(&ClaimType::Numeric, "485 tests").unwrap();
        assert!(entry.verified);
    }

    #[test]
    fn test_gtv_cache_invalidate_type() {
        let mut cache = GtvCache::default();
        cache.put(&ClaimType::Path, "a.rs", true, None, "ok".into());
        cache.put(&ClaimType::Path, "b.rs", true, None, "ok".into());
        cache.put(&ClaimType::Url, "https://x.com", true, None, "ok".into());

        cache.invalidate_type(&ClaimType::Path);
        assert_eq!(cache.entries.len(), 1);
        assert!(cache.get(&ClaimType::Url, "https://x.com").is_some());
    }

    #[test]
    fn test_gtv_force_bypasses_cache() {
        let mut cache = GtvCache::default();
        cache.put(&ClaimType::Path, "file.rs", true, None, "cached".into());

        // clear simulates --force-gtv
        cache.clear();
        assert!(cache.entries.is_empty());
    }
}
