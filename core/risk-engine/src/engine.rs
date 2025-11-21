use crate::{
    config::RiskEngineConfig,
    error::Result,
    guards::{
        intent::IntentGuard, route::RouteGuard, subscription::SubscriptionGuard,
        toolchain::ToolchainGuard, GuardTrait,
    },
    models::{GuardResult, RiskAssessment, RiskScore, ScoreBreakdown, TransactionRequest},
    policy::PolicyEngine,
};
use std::sync::Arc;
use tracing::{info, instrument};

pub struct RiskEngine {
    config: RiskEngineConfig,
    intent_guard: IntentGuard,
    route_guard: RouteGuard,
    subscription_guard: SubscriptionGuard,
    toolchain_guard: ToolchainGuard,
    policy_engine: Arc<PolicyEngine>,
}

impl RiskEngine {
    pub fn new(config: RiskEngineConfig) -> Result<Self> {
        Ok(Self {
            intent_guard: IntentGuard::new(),
            route_guard: RouteGuard::new(),
            subscription_guard: SubscriptionGuard::new(),
            toolchain_guard: ToolchainGuard::new(),
            policy_engine: Arc::new(PolicyEngine::new()),
            config,
        })
    }

    #[instrument(skip(self, request))]
    pub async fn assess(&self, request: &TransactionRequest) -> Result<RiskAssessment> {
        info!("Starting risk assessment for transaction");

        let mut guard_results = Vec::new();

        if self.config.guards.intent_guard_enabled {
            let result = self.intent_guard.evaluate(request).await?;
            guard_results.push(result);
        }

        if self.config.guards.route_guard_enabled {
            let result = self.route_guard.evaluate(request).await?;
            guard_results.push(result);
        }

        if self.config.guards.subscription_guard_enabled {
            let result = self.subscription_guard.evaluate(request).await?;
            guard_results.push(result);
        }

        if self.config.guards.toolchain_guard_enabled {
            let result = self.toolchain_guard.evaluate(request).await?;
            guard_results.push(result);
        }

        let risk_score = self.calculate_risk_score(&guard_results, request);
        let approved = risk_score.overall >= self.config.approval_threshold;

        let evidence = guard_results
            .iter()
            .flat_map(|r| r.evidence.clone())
            .collect();

        let reason = if !approved {
            Some(self.generate_rejection_reason(&guard_results))
        } else {
            None
        };

        Ok(RiskAssessment {
            approved,
            risk_score,
            guard_results,
            reason,
            timestamp: chrono::Utc::now().timestamp(),
            evidence,
        })
    }

    fn calculate_risk_score(
        &self,
        guard_results: &[GuardResult],
        request: &TransactionRequest,
    ) -> RiskScore {
        let weights = &self.config.guards.guard_weights;

        let mut intent_score = 100;
        let mut route_score = 100;
        let mut subscription_score = 100;
        let mut toolchain_score = 100;
        let behavioral_score = self.calculate_behavioral_score(request);

        for result in guard_results {
            match result.guard_name.as_str() {
                "intent" => intent_score = result.score,
                "route" => route_score = result.score,
                "subscription" => subscription_score = result.score,
                "toolchain" => toolchain_score = result.score,
                _ => {}
            }
        }

        let overall = (
            (intent_score as f32 * weights.intent)
                + (route_score as f32 * weights.route)
                + (subscription_score as f32 * weights.subscription)
                + (toolchain_score as f32 * weights.toolchain)
                + (behavioral_score as f32 * weights.behavioral)
        ) as u8;

        RiskScore {
            overall,
            breakdown: ScoreBreakdown {
                intent_score,
                route_score,
                subscription_score,
                toolchain_score,
                behavioral_score,
            },
        }
    }

    fn calculate_behavioral_score(&self, request: &TransactionRequest) -> u8 {
        if request.amount > 500.0 {
            (75 + (rand::random::<u8>() % 10))
        } else if request.amount > 100.0 {
            (85 + (rand::random::<u8>() % 10))
        } else {
            (95 + (rand::random::<u8>() % 5))
        }
    }

    fn generate_rejection_reason(&self, guard_results: &[GuardResult]) -> String {
        let failed_guards: Vec<&str> = guard_results
            .iter()
            .filter(|r| !r.passed)
            .map(|r| r.guard_name.as_str())
            .collect();

        if failed_guards.is_empty() {
            "Risk score below approval threshold".to_string()
        } else {
            format!(
                "Failed guard checks: {}. See evidence for details.",
                failed_guards.join(", ")
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{AgentContext, ExecutionTrace};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_risk_assessment() {
        let config = RiskEngineConfig::default();
        let engine = RiskEngine::new(config).unwrap();

        let request = TransactionRequest {
            amount: 100.0,
            recipient: "test_recipient".to_string(),
            agent_context: AgentContext {
                intent: "Test transaction".to_string(),
                execution_trace: vec![],
                reasoning_chain: vec![],
                tool_calls: vec![],
                environment: HashMap::new(),
            },
            metadata: HashMap::new(),
        };

        let result = engine.assess(&request).await;
        assert!(result.is_ok());

        let assessment = result.unwrap();
        assert!(assessment.risk_score.overall > 0);
    }
}
