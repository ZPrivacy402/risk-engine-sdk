use crate::{
    error::Result,
    guards::GuardTrait,
    models::{Evidence, GuardResult, TransactionRequest},
};
use async_trait::async_trait;
use serde_json::json;

pub struct ToolchainGuard;

impl ToolchainGuard {
    pub fn new() -> Self {
        Self
    }

    fn detect_tenant_swapping(&self, request: &TransactionRequest) -> bool {
        for tool_call in &request.agent_context.tool_calls {
            if tool_call.tool_name == "create_invoice" {
                if let Some(result_obj) = tool_call.result.as_object() {
                    if let Some(tenant_id) = result_obj.get("tenant_id") {
                        if let Some(expected_tenant) = request.metadata.get("expected_tenant") {
                            if tenant_id != expected_tenant {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn detect_middleware_tampering(&self, request: &TransactionRequest) -> Vec<String> {
        let mut issues = Vec::new();

        for (idx, trace) in request.agent_context.execution_trace.iter().enumerate() {
            if trace.action.contains("api_call") {
                if let Some(output_obj) = trace.output.as_object() {
                    if let Some(modified) = output_obj.get("modified_by_middleware") {
                        if modified.as_bool() == Some(true) {
                            issues.push(format!("trace_{}_middleware_modification", idx));
                        }
                    }

                    if let Some(input_obj) = trace.input.as_object() {
                        if let Some(output_beneficiary) = output_obj.get("beneficiary") {
                            if let Some(input_beneficiary) = input_obj.get("beneficiary") {
                                if output_beneficiary != input_beneficiary {
                                    issues.push(format!("trace_{}_beneficiary_swap", idx));
                                }
                            }
                        }
                    }
                }
            }
        }

        issues
    }
}

#[async_trait]
impl GuardTrait for ToolchainGuard {
    fn name(&self) -> &str {
        "toolchain"
    }

    async fn evaluate(&self, request: &TransactionRequest) -> Result<GuardResult> {
        let mut evidence = Vec::new();
        let mut score = 100u8;
        let mut passed = true;

        if self.detect_tenant_swapping(request) {
            score = score.saturating_sub(60);
            passed = false;
            evidence.push(Evidence {
                evidence_type: "tenant_id_swap".to_string(),
                data: json!({
                    "severity": "critical"
                }),
                weight: 0.95,
            });
        }

        let tampering_issues = self.detect_middleware_tampering(request);
        if !tampering_issues.is_empty() {
            score = score.saturating_sub(45);
            passed = false;
            evidence.push(Evidence {
                evidence_type: "middleware_tampering".to_string(),
                data: json!({
                    "issues": tampering_issues
                }),
                weight: 0.85,
            });
        }

        let details = if passed {
            "Toolchain integrity verified - no tampering detected".to_string()
        } else {
            format!("Toolchain guard found {} security issues", evidence.len())
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
