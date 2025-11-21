use crate::{trace::RiskAssessmentTrace, PublicInputs};
use winterfell::{
    Air, AirContext, Assertion, EvaluationFrame, FieldExtension, ProofOptions,
    TraceInfo, TransitionConstraintDegree,
};
use winterfell::math::{fields::f128::BaseElement, FieldElement};

pub struct RiskAssessmentAir {
    context: AirContext<BaseElement>,
    trace: RiskAssessmentTrace,
    public_inputs: PublicInputs,
}

impl RiskAssessmentAir {
    pub fn new(trace: RiskAssessmentTrace, public_inputs: PublicInputs) -> Self {
        let trace_length = trace.len().next_power_of_two();
        let num_assertions = 2;

        let context = AirContext::new(
            TraceInfo::new(2, trace_length),
            TransitionConstraintDegree::new(vec![1, 1]),
            num_assertions,
            ProofOptions::new(
                27,
                8,
                20,
                FieldExtension::None,
                4,
                31,
            ),
        );

        Self {
            context,
            trace,
            public_inputs,
        }
    }
}

impl Air for RiskAssessmentAir {
    type BaseField = BaseElement;
    type PublicInputs = PublicInputs;

    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }

    fn evaluate_transition<E: FieldElement + From<Self::BaseField>>(
        &self,
        frame: &EvaluationFrame<E>,
        _periodic_values: &[E],
        result: &mut [E],
    ) {
        let current = frame.current();
        let next = frame.next();

        result[0] = next[1] - (current[1] + current[0]);

        result[1] = E::ZERO;
    }

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        let last_step = self.trace_length() - 1;
        vec![
            Assertion::single(0, 0, BaseElement::new(self.public_inputs.risk_score as u128)),
            Assertion::single(1, last_step, BaseElement::ZERO),
        ]
    }
}
