use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRequest {
    pub amount: f64,
    pub recipient: String,
    pub agent_context: AgentContext,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentContext {
    pub intent: String,
    pub execution_trace: Vec<ExecutionTrace>,
    pub reasoning_chain: Vec<String>,
    pub tool_calls: Vec<ToolCall>,
    pub environment: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionTrace {
    pub timestamp: i64,
    pub action: String,
    pub input: serde_json::Value,
    pub output: serde_json::Value,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub tool_name: String,
    pub arguments: serde_json::Value,
    pub result: serde_json::Value,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub approved: bool,
    pub risk_score: RiskScore,
    pub guard_results: Vec<GuardResult>,
    pub reason: Option<String>,
    pub timestamp: i64,
    pub evidence: Vec<Evidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskScore {
    pub overall: u8,
    pub breakdown: ScoreBreakdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreBreakdown {
    pub intent_score: u8,
    pub route_score: u8,
    pub subscription_score: u8,
    pub toolchain_score: u8,
    pub behavioral_score: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardResult {
    pub guard_name: String,
    pub passed: bool,
    pub score: u8,
    pub details: String,
    pub evidence: Vec<Evidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub evidence_type: String,
    pub data: serde_json::Value,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerchantInfo {
    pub name: String,
    pub wallet_address: String,
    pub authorized: bool,
    pub kyb_verified: bool,
    pub reputation_score: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutTerms {
    pub total_amount: f64,
    pub recurring: bool,
    pub renewal_amount: Option<f64>,
    pub renewal_period: Option<String>,
    pub cancellation_policy: Option<String>,
}
