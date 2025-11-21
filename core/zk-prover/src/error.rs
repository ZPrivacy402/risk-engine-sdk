use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Proof generation failed: {0}")]
    ProofGeneration(String),

    #[error("Proof verification failed: {0}")]
    ProofVerification(String),

    #[error("Invalid trace: {0}")]
    InvalidTrace(String),

    #[error("Invalid public inputs: {0}")]
    InvalidPublicInputs(String),

    #[error("Bonsol client error: {0}")]
    BonsolClient(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
