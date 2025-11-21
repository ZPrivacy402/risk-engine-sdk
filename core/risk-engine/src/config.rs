use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskEngineConfig {
    pub approval_threshold: u8,
    pub guards: GuardConfig,
    pub telemetry: TelemetryConfig,
    pub behavioral_analytics: Option<BehavioralConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardConfig {
    pub intent_guard_enabled: bool,
    pub route_guard_enabled: bool,
    pub subscription_guard_enabled: bool,
    pub toolchain_guard_enabled: bool,
    pub guard_weights: GuardWeights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardWeights {
    pub intent: f32,
    pub route: f32,
    pub subscription: f32,
    pub toolchain: f32,
    pub behavioral: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    pub enabled: bool,
    pub endpoint: Option<String>,
    pub sample_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralConfig {
    pub enabled: bool,
    pub model_path: String,
    pub feature_set: Vec<String>,
}

impl Default for RiskEngineConfig {
    fn default() -> Self {
        Self {
            approval_threshold: 70,
            guards: GuardConfig::default(),
            telemetry: TelemetryConfig::default(),
            behavioral_analytics: None,
        }
    }
}

impl Default for GuardConfig {
    fn default() -> Self {
        Self {
            intent_guard_enabled: true,
            route_guard_enabled: true,
            subscription_guard_enabled: true,
            toolchain_guard_enabled: true,
            guard_weights: GuardWeights::default(),
        }
    }
}

impl Default for GuardWeights {
    fn default() -> Self {
        Self {
            intent: 0.25,
            route: 0.25,
            subscription: 0.20,
            toolchain: 0.20,
            behavioral: 0.10,
        }
    }
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: None,
            sample_rate: 1.0,
        }
    }
}
