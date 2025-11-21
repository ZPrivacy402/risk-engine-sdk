pub mod intent;
pub mod route;
pub mod subscription;
pub mod toolchain;

use crate::{error::Result, models::{Evidence, GuardResult, TransactionRequest}};
use async_trait::async_trait;

#[async_trait]
pub trait GuardTrait: Send + Sync {
    fn name(&self) -> &str;
    async fn evaluate(&self, request: &TransactionRequest) -> Result<GuardResult>;
}
