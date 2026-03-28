//! Ground Truth Verification (GTV) — automated claim extraction and verification.
//!
//! If CruxDev says it on a public page, it must be TRUE RIGHT NOW.

pub mod cache;
pub mod scanner;
pub mod verifier;

use serde::{Deserialize, Serialize};

/// A claim extracted from content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub claim_type: ClaimType,
    pub value: String,
    pub line_number: usize,
    pub source_text: String,
}

/// Types of verifiable claims.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClaimType {
    Numeric,  // "485 tests", "61 tools"
    Feature,  // "supports X", "integrates with Y"
    Status,   // "active", "live", "deployed"
    Path,     // file/directory references
    Url,      // https:// links
}

/// Result of verifying a single claim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub claim: Claim,
    pub verified: bool,
    pub actual_value: Option<String>,
    pub message: String,
    pub confidence: Confidence,
}

/// How confident are we in the verification result?
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Confidence {
    /// Verified by code — file exists, command output matches.
    High,
    /// Verified by grep/heuristic — feature keyword found but implementation not proven.
    Medium,
    /// Could not verify — needs manual review.
    Low,
}

/// Summary of a GTV scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub file_path: String,
    pub total_claims: usize,
    pub verified: usize,
    pub failed: usize,
    pub uncertain: usize,
    pub results: Vec<VerificationResult>,
}

impl ScanResult {
    pub fn passed(&self) -> bool {
        self.failed == 0
    }
}
