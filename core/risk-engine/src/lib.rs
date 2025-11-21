///! ZPrivacy402 Risk Engine
///!
///! Production-grade risk assessment framework for autonomous AI agent transactions.
///! Implements the TRUSTLINE protection framework with multi-layered guards.

pub mod config;
pub mod engine;
pub mod error;
pub mod guards;
pub mod models;
pub mod policy;
pub mod telemetry;

pub use config::RiskEngineConfig;
pub use engine::RiskEngine;
pub use error::{Error, Result};
pub use guards::{
    intent::IntentGuard,
    route::RouteGuard,
    subscription::SubscriptionGuard,
    toolchain::ToolchainGuard,
    GuardResult, GuardTrait,
};
pub use models::{
    AgentContext, ExecutionTrace, RiskAssessment, RiskScore, TransactionRequest,
};
pub use policy::{Policy, PolicyEngine, RuleSet};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_loads() {
        assert!(true, "Library loaded successfully");
    }
}
