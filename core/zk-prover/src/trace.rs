use serde::{Deserialize, Serialize};
use winterfell::{
    math::{fields::f128::BaseElement, FieldElement},
    Matrix, TraceTable,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessmentTrace {
    risk_scores: Vec<u8>,
}

impl RiskAssessmentTrace {
    pub fn new(risk_scores: Vec<u8>) -> Self {
        Self { risk_scores }
    }

    pub fn len(&self) -> usize {
        self.risk_scores.len()
    }

    pub fn build_trace_table(&self) -> TraceTable<BaseElement> {
        let trace_length = self.risk_scores.len().next_power_of_two();

        let mut scores = vec![BaseElement::ZERO; trace_length];
        for (i, &score) in self.risk_scores.iter().enumerate() {
            scores[i] = BaseElement::new(score as u128);
        }

        let mut cumulative = vec![BaseElement::ZERO; trace_length];
        cumulative[0] = scores[0];
        for i in 1..trace_length {
            cumulative[i] = cumulative[i - 1] + scores[i];
        }

        TraceTable::init(vec![scores, cumulative])
    }
}

pub struct TraceBuilder {
    scores: Vec<u8>,
}

impl TraceBuilder {
    pub fn new() -> Self {
        Self { scores: Vec::new() }
    }

    pub fn add_score(mut self, score: u8) -> Self {
        self.scores.push(score);
        self
    }

    pub fn build(self) -> RiskAssessmentTrace {
        RiskAssessmentTrace::new(self.scores)
    }
}
