///! ZPrivacy402 zk-STARK Prover
///!
///! Privacy-preserving transaction verification using Winterfell STARK proofs.
///! Integrates with Bonsol for Solana blockchain verification.

pub mod air;
pub mod bonsol;
pub mod error;
pub mod prover;
pub mod trace;
pub mod verifier;

pub use error::{Error, Result};
pub use prover::{ProofOptions, ZKProver};
pub use trace::{RiskAssessmentTrace, TraceBuilder};
pub use verifier::Verifier;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub stark_proof: Vec<u8>,
    pub public_inputs: PublicInputs,
    pub proof_metadata: ProofMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicInputs {
    pub risk_score: u8,
    pub approval_status: bool,
    pub timestamp: i64,
    pub transaction_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofMetadata {
    pub security_level: u32,
    pub proof_size_bytes: usize,
    pub generation_time_ms: u64,
    pub soundness_error: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_loads() {
        assert!(true, "zk-prover library loaded successfully");
    }
}
