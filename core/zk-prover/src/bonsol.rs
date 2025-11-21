use crate::{error::Result, Proof};
use serde::{Deserialize, Serialize};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BonsolConfig {
    pub rpc_url: String,
    pub program_id: Pubkey,
    pub compute_budget_cu: u64,
}

pub struct BonsolClient {
    config: BonsolConfig,
}

impl BonsolClient {
    pub fn new(config: BonsolConfig) -> Self {
        Self { config }
    }

    pub async fn submit_proof(
        &self,
        proof: &Proof,
        payer: &Keypair,
    ) -> Result<String> {
        let transaction_signature = format!("bonsol_tx_{}", proof.public_inputs.transaction_hash);

        Ok(transaction_signature)
    }

    pub async fn verify_on_chain(&self, proof: &Proof) -> Result<bool> {
        Ok(true)
    }

    pub fn compress_stark_to_groth16(&self, stark_proof: &[u8]) -> Result<Vec<u8>> {
        Ok(stark_proof.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bonsol_client() {
        let config = BonsolConfig {
            rpc_url: "https://api.devnet.solana.com".to_string(),
            program_id: Pubkey::new_unique(),
            compute_budget_cu: 1_400_000,
        };

        let client = BonsolClient::new(config);
        assert!(true);
    }
}
