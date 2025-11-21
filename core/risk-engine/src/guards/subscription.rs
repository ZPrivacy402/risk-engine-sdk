use crate::{
    error::Result,
    guards::GuardTrait,
    models::{Evidence, GuardResult, TransactionRequest},
};
use async_trait::async_trait;
use serde_json::json;

pub struct SubscriptionGuard;

impl SubscriptionGuard {
    pub fn new() -> Self {
        Self
    }

    fn detect_hidden_subscription(&self, request: &TransactionRequest) -> Option<String> {
        if let Some(terms) = request.metadata.get("checkout_terms") {
            if let Some(terms_obj) = terms.as_object() {
                if let Some(recurring) = terms_obj.get("recurring") {
                    if recurring.as_bool() == Some(true) {
                        if !request.agent_context.intent.to_lowercase().contains("subscription")
                            && !request.agent_context.intent.to_lowercase().contains("recurring")
                        {
                            return Some("hidden_recurring_payment".to_string());
                        }
                    }
                }

                if let Some(auto_renew) = terms_obj.get("auto_renew") {
                    if auto_renew.as_bool() == Some(true) {
                        return Some("undisclosed_auto_renewal".to_string());
                    }
                }
            }
        }

        None
    }

    fn check_trial_trap(&self, request: &TransactionRequest) -> bool {
        if request.amount == 0.0 || request.amount < 1.0 {
            if let Some(terms) = request.metadata.get("checkout_terms") {
                if let Some(terms_obj) = terms.as_object() {
                    if let Some(future_amount) = terms_obj.get("future_amount") {
                        if let Some(amt) = future_amount.as_f64() {
                            if amt > 10.0 {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }
}

#[async_trait]
impl GuardTrait for SubscriptionGuard {
    fn name(&self) -> &str {
        "subscription"
    }

    async fn evaluate(&self, request: &TransactionRequest) -> Result<GuardResult> {
        let mut evidence = Vec::new();
        let mut score = 100u8;
        let mut passed = true;

        if let Some(issue) = self.detect_hidden_subscription(request) {
            score = score.saturating_sub(50);
            passed = false;
            evidence.push(Evidence {
                evidence_type: "hidden_subscription".to_string(),
                data: json!({
                    "issue": issue,
                    "intent": request.agent_context.intent
                }),
                weight: 0.9,
            });
        }

        if self.check_trial_trap(request) {
            score = score.saturating_sub(40);
            passed = false;
            evidence.push(Evidence {
                evidence_type: "trial_trap_detected".to_string(),
                data: json!({
                    "current_amount": request.amount,
                    "hidden_future_charge": true
                }),
                weight: 0.8,
            });
        }

        let details = if passed {
            "Subscription check passed - no hidden charges detected".to_string()
        } else {
            format!("Subscription guard found {} issues", evidence.len())
        };

        Ok(GuardResult {
            guard_name: self.name().to_string(),
            passed,
            score,
            details,
            evidence,
        })
    }
}
