from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional


@dataclass
class ExecutionTrace:
    timestamp: int
    action: str
    input: Any
    output: Any
    metadata: Dict[str, str] = field(default_factory=dict)


@dataclass
class ToolCall:
    tool_name: str
    arguments: Any
    result: Any
    timestamp: int


@dataclass
class AgentContext:
    intent: str
    execution_trace: List[ExecutionTrace]
    reasoning_chain: List[str]
    tool_calls: List[ToolCall]
    environment: Dict[str, str] = field(default_factory=dict)


@dataclass
class TransactionRequest:
    amount: float
    recipient: str
    agent_context: AgentContext
    metadata: Dict[str, Any] = field(default_factory=dict)


@dataclass
class Evidence:
    evidence_type: str
    data: Any
    weight: float


@dataclass
class GuardResult:
    guard_name: str
    passed: bool
    score: int
    details: str
    evidence: List[Evidence]


@dataclass
class ScoreBreakdown:
    intent_score: int
    route_score: int
    subscription_score: int
    toolchain_score: int
    behavioral_score: int


@dataclass
class RiskScore:
    overall: int
    breakdown: ScoreBreakdown


@dataclass
class RiskAssessment:
    approved: bool
    risk_score: RiskScore
    guard_results: List[GuardResult]
    reason: Optional[str]
    timestamp: int
    evidence: List[Evidence]


@dataclass
class PublicInputs:
    risk_score: int
    approval_status: bool
    timestamp: int
    transaction_hash: str


@dataclass
class ProofMetadata:
    security_level: int
    proof_size_bytes: int
    generation_time_ms: int
    soundness_error: str


@dataclass
class Proof:
    stark_proof: bytes
    public_inputs: PublicInputs
    proof_metadata: ProofMetadata
