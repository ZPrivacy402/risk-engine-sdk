use crate::{
    air::RiskAssessmentAir,
    error::Result,
    trace::RiskAssessmentTrace,
    {Proof, ProofMetadata, PublicInputs},
};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use winterfell::{
    crypto::{hashers::Blake3_256, DefaultRandomCoin},
    math::{fields::f128::BaseElement, FieldElement},
    ProofOptions as WinterfellProofOptions, Prover as WinterfellProver, StarkProof, Trace,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOptions {
    pub security_level: u32,
    pub num_queries: u32,
    pub blowup_factor: u32,
    pub grinding_factor: u32,
}

impl Default for ProofOptions {
    fn default() -> Self {
        Self {
            security_level: 128,
            num_queries: 27,
            blowup_factor: 8,
            grinding_factor: 20,
        }
    }
}

pub struct ZKProver {
    options: ProofOptions,
}

impl ZKProver {
    pub fn new(options: ProofOptions) -> Self {
        Self { options }
    }

    pub async fn generate_proof(
        &self,
        trace: &RiskAssessmentTrace,
        public_inputs: &PublicInputs,
    ) -> Result<Proof> {
        let start_time = Instant::now();

        let winterfell_options = WinterfellProofOptions::new(
            self.options.num_queries as usize,
            self.options.blowup_factor as usize,
            self.options.grinding_factor as u32,
            winterfell::FieldExtension::None,
            4,
            31,
        );

        let air = RiskAssessmentAir::new(trace.clone(), public_inputs.clone());

        let trace_table = trace.build_trace_table();

        let stark_proof = winterfell::prove::<RiskAssessmentAir, Blake3_256<BaseElement>, DefaultRandomCoin<Blake3_256<BaseElement>>>(
            trace_table,
            air,
            winterfell_options,
        )
        .map_err(|e| crate::Error::ProofGeneration(e.to_string()))?;

        let proof_bytes = bincode::serialize(&stark_proof)
            .map_err(|e| crate::Error::Serialization(e.to_string()))?;

        let generation_time = start_time.elapsed();

        let metadata = ProofMetadata {
            security_level: self.options.security_level,
            proof_size_bytes: proof_bytes.len(),
            generation_time_ms: generation_time.as_millis() as u64,
            soundness_error: format!("2^-{}", self.options.security_level),
        };

        Ok(Proof {
            stark_proof: proof_bytes,
            public_inputs: public_inputs.clone(),
            proof_metadata: metadata,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_proof_generation() {
        let options = ProofOptions::default();
        let prover = ZKProver::new(options);

        let trace = RiskAssessmentTrace::new(vec![95, 92, 88, 85]);

        let public_inputs = PublicInputs {
            risk_score: 95,
            approval_status: true,
            timestamp: 1234567890,
            transaction_hash: "test_hash".to_string(),
        };

        let result = prover.generate_proof(&trace, &public_inputs).await;
        assert!(result.is_ok());

        let proof = result.unwrap();
        assert!(proof.proof_metadata.proof_size_bytes > 0);
        assert!(proof.proof_metadata.generation_time_ms > 0);
    }
}
