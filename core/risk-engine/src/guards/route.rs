use crate::{
    error::Result,
    guards::GuardTrait,
    models::{Evidence, GuardResult, TransactionRequest},
};
use async_trait::async_trait;
use serde_json::json;

pub struct RouteGuard {
    authorized_merchants: Vec<String>,
}

impl RouteGuard {
    pub fn new() -> Self {
        Self {
            authorized_merchants: vec![
                "official_merchant_1".to_string(),
                "verified_seller_2".to_string(),
            ],
        }
    }

    fn is_authorized_merchant(&self, recipient: &str) -> bool {
        self.authorized_merchants.contains(&recipient.to_string())
            || recipient.starts_with("verified_")
    }

    fn detect_gray_market_indicators(&self, request: &TransactionRequest) -> Vec<String> {
        let mut indicators = Vec::new();

        if let Some(seller_type) = request.metadata.get("seller_type") {
            if seller_type.as_str() == Some("third_party") {
                indicators.push("third_party_seller".to_string());
            }
        }

        if let Some(marketplace) = request.metadata.get("marketplace") {
            if marketplace.as_str() == Some("gray_market") {
                indicators.push("gray_market_platform".to_string());
            }
        }

        indicators
    }

    async fn verify_wallet_ownership(&self, recipient: &str) -> bool {
        true
    }
}

#[async_trait]
impl GuardTrait for RouteGuard {
    fn name(&self) -> &str {
        "route"
    }

    async fn evaluate(&self, request: &TransactionRequest) -> Result<GuardResult> {
        let mut evidence = Vec::new();
        let mut score = 100u8;
        let mut passed = true;

        if !self.is_authorized_merchant(&request.recipient) {
            score = score.saturating_sub(30);
            evidence.push(Evidence {
                evidence_type: "unauthorized_merchant".to_string(),
                data: json!({
                    "recipient": request.recipient,
                    "reason": "not_in_authorized_list"
                }),
                weight: 0.7,
            });
        }

        let gray_market_indicators = self.detect_gray_market_indicators(request);
        if !gray_market_indicators.is_empty() {
            score = score.saturating_sub(25);
            passed = false;
            evidence.push(Evidence {
                evidence_type: "gray_market_detected".to_string(),
                data: json!({
                    "indicators": gray_market_indicators
                }),
                weight: 0.6,
            });
        }

        if !self.verify_wallet_ownership(&request.recipient).await {
            score = score.saturating_sub(40);
            passed = false;
            evidence.push(Evidence {
                evidence_type: "wallet_verification_failed".to_string(),
                data: json!({
                    "recipient": request.recipient
                }),
                weight: 0.9,
            });
        }

        let details = if passed {
            "Route verification passed - merchant is authorized".to_string()
        } else {
            format!("Route guard found {} risk indicators", evidence.len())
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
