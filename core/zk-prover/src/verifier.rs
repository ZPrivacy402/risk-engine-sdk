use crate::{air::RiskAssessmentAir, error::Result, trace::RiskAssessmentTrace, Proof};
use winterfell::{
    crypto::{hashers::Blake3_256, DefaultRandomCoin},
    math::fields::f128::BaseElement,
    verify, StarkProof,
};

pub struct Verifier;

impl Verifier {
    pub fn new() -> Self {
        Self
    }

    pub async fn verify(&self, proof: &Proof) -> Result<bool> {
        let stark_proof: StarkProof = bincode::deserialize(&proof.stark_proof)
            .map_err(|e| crate::Error::Serialization(e.to_string()))?;

        let dummy_trace = RiskAssessmentTrace::new(vec![proof.public_inputs.risk_score]);
        let air = RiskAssessmentAir::new(dummy_trace, proof.public_inputs.clone());

        let verification_result = verify::<RiskAssessmentAir, Blake3_256<BaseElement>, DefaultRandomCoin<Blake3_256<BaseElement>>>(
            stark_proof,
            air,
        );

        match verification_result {
            Ok(_) => Ok(true),
            Err(e) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{prover::{ProofOptions, ZKProver}, PublicInputs};

    #[tokio::test]
    async fn test_proof_verification() {
        let prover = ZKProver::new(ProofOptions::default());
        let trace = RiskAssessmentTrace::new(vec![95, 92, 88, 85]);

        let public_inputs = PublicInputs {
            risk_score: 95,
            approval_status: true,
            timestamp: 1234567890,
            transaction_hash: "test".to_string(),
        };

        let proof = prover.generate_proof(&trace, &public_inputs).await.unwrap();

        let verifier = Verifier::new();
        let is_valid = verifier.verify(&proof).await.unwrap();

        assert!(is_valid);
    }
}
