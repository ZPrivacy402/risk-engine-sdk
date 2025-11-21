"""
ZPrivacy402 Python SDK

Production-grade risk protection for autonomous AI agent payments on Solana.
"""

from .risk_engine import RiskEngine, RiskEngineConfig
from .zk_prover import ZKProver, ProofOptions, Proof
from .providers import SolanaProvider
from .types import (
    TransactionRequest,
    RiskAssessment,
    AgentContext,
    ExecutionTrace,
    ToolCall,
    GuardResult,
)

__version__ = "1.0.0"
__all__ = [
    "RiskEngine",
    "RiskEngineConfig",
    "ZKProver",
    "ProofOptions",
    "Proof",
    "SolanaProvider",
    "TransactionRequest",
    "RiskAssessment",
    "AgentContext",
    "ExecutionTrace",
    "ToolCall",
    "GuardResult",
]
