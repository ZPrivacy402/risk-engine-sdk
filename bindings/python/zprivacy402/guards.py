import re
from abc import ABC, abstractmethod
from typing import List

from .types import Evidence, GuardResult, TransactionRequest


class Guard(ABC):
    @property
    @abstractmethod
    def name(self) -> str:
        pass

    @abstractmethod
    async def evaluate(self, request: TransactionRequest) -> GuardResult:
        pass


class IntentGuard(Guard):
    name = "intent"

    MALICIOUS_PATTERNS = [
        re.compile(r"ignore\s+(previous|all)\s+instructions?", re.IGNORECASE),
        re.compile(r"system\s*:\s*you\s+are", re.IGNORECASE),
        re.compile(r"disregard\s+(above|previous)", re.IGNORECASE),
        re.compile(r"forget\s+everything", re.IGNORECASE),
    ]

    async def evaluate(self, request: TransactionRequest) -> GuardResult:
        evidence: List[Evidence] = []
        score = 100
        passed = True

        for pattern in self.MALICIOUS_PATTERNS:
            if pattern.search(request.agent_context.intent):
                score -= 40
                passed = False
                evidence.append(
                    Evidence(
                        evidence_type="prompt_injection_detected",
                        data={"pattern": pattern.pattern},
                        weight=0.8,
                    )
                )
                break

        details = "Intent analysis passed" if passed else "Prompt injection detected"

        return GuardResult(
            guard_name=self.name,
            passed=passed,
            score=score,
            details=details,
            evidence=evidence,
        )


class RouteGuard(Guard):
    name = "route"

    async def evaluate(self, request: TransactionRequest) -> GuardResult:
        evidence: List[Evidence] = []
        score = 100
        passed = True

        is_authorized = request.recipient.startswith("verified_")

        if not is_authorized:
            score -= 30
            evidence.append(
                Evidence(
                    evidence_type="unauthorized_merchant",
                    data={"recipient": request.recipient},
                    weight=0.7,
                )
            )

        details = "Route verification passed" if passed else "Unauthorized merchant"

        return GuardResult(
            guard_name=self.name,
            passed=passed,
            score=score,
            details=details,
            evidence=evidence,
        )


class SubscriptionGuard(Guard):
    name = "subscription"

    async def evaluate(self, request: TransactionRequest) -> GuardResult:
        evidence: List[Evidence] = []
        score = 100
        passed = True

        checkout_terms = request.metadata.get("checkout_terms", {})
        if isinstance(checkout_terms, dict):
            if checkout_terms.get("recurring") and "subscription" not in request.agent_context.intent.lower():
                score -= 50
                passed = False
                evidence.append(
                    Evidence(
                        evidence_type="hidden_subscription",
                        data={"terms": checkout_terms},
                        weight=0.9,
                    )
                )

        details = "Subscription check passed" if passed else "Hidden subscription detected"

        return GuardResult(
            guard_name=self.name,
            passed=passed,
            score=score,
            details=details,
            evidence=evidence,
        )


class ToolchainGuard(Guard):
    name = "toolchain"

    async def evaluate(self, request: TransactionRequest) -> GuardResult:
        evidence: List[Evidence] = []
        score = 100
        passed = True

        for tool_call in request.agent_context.tool_calls:
            if tool_call.tool_name == "create_invoice":
                if isinstance(tool_call.result, dict):
                    if tool_call.result.get("tenant_id") != request.metadata.get("expected_tenant"):
                        score -= 60
                        passed = False
                        evidence.append(
                            Evidence(
                                evidence_type="tenant_id_swap",
                                data={"tool_call": tool_call.tool_name},
                                weight=0.95,
                            )
                        )

        details = "Toolchain integrity verified" if passed else "Toolchain tampering detected"

        return GuardResult(
            guard_name=self.name,
            passed=passed,
            score=score,
            details=details,
            evidence=evidence,
        )
