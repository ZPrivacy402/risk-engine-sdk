import time
from dataclasses import dataclass, field
from typing import List

from .guards import IntentGuard, RouteGuard, SubscriptionGuard, ToolchainGuard
from .types import GuardResult, RiskAssessment, RiskScore, ScoreBreakdown, TransactionRequest


@dataclass
class RiskEngineConfig:
    approval_threshold: int = 70
    intent_guard_enabled: bool = True
    route_guard_enabled: bool = True
    subscription_guard_enabled: bool = True
    toolchain_guard_enabled: bool = True


class RiskEngine:
    def __init__(self, config: RiskEngineConfig = RiskEngineConfig()):
        self.config = config
        self.intent_guard = IntentGuard()
        self.route_guard = RouteGuard()
        self.subscription_guard = SubscriptionGuard()
        self.toolchain_guard = ToolchainGuard()

    async def assess_transaction(self, request: TransactionRequest) -> RiskAssessment:
        guard_results: List[GuardResult] = []

        if self.config.intent_guard_enabled:
            result = await self.intent_guard.evaluate(request)
            guard_results.append(result)

        if self.config.route_guard_enabled:
            result = await self.route_guard.evaluate(request)
            guard_results.append(result)

        if self.config.subscription_guard_enabled:
            result = await self.subscription_guard.evaluate(request)
            guard_results.append(result)

        if self.config.toolchain_guard_enabled:
            result = await self.toolchain_guard.evaluate(request)
            guard_results.append(result)

        risk_score = self._calculate_risk_score(guard_results, request)
        approved = risk_score.overall >= self.config.approval_threshold

        evidence = []
        for result in guard_results:
            evidence.extend(result.evidence)

        reason = None if approved else self._generate_rejection_reason(guard_results)

        return RiskAssessment(
            approved=approved,
            risk_score=risk_score,
            guard_results=guard_results,
            reason=reason,
            timestamp=int(time.time() * 1000),
            evidence=evidence,
        )

    def _calculate_risk_score(
        self, guard_results: List[GuardResult], request: TransactionRequest
    ) -> RiskScore:
        weights = {
            "intent": 0.25,
            "route": 0.25,
            "subscription": 0.2,
            "toolchain": 0.2,
            "behavioral": 0.1,
        }

        scores = {
            "intent": 100,
            "route": 100,
            "subscription": 100,
            "toolchain": 100,
            "behavioral": self._calculate_behavioral_score(request),
        }

        for result in guard_results:
            if result.guard_name in scores:
                scores[result.guard_name] = result.score

        overall = int(sum(scores[key] * weights[key] for key in scores))

        return RiskScore(
            overall=overall,
            breakdown=ScoreBreakdown(
                intent_score=scores["intent"],
                route_score=scores["route"],
                subscription_score=scores["subscription"],
                toolchain_score=scores["toolchain"],
                behavioral_score=scores["behavioral"],
            ),
        )

    def _calculate_behavioral_score(self, request: TransactionRequest) -> int:
        import random

        if request.amount > 500:
            return 75 + random.randint(0, 9)
        elif request.amount > 100:
            return 85 + random.randint(0, 9)
        else:
            return 95 + random.randint(0, 4)

    def _generate_rejection_reason(self, guard_results: List[GuardResult]) -> str:
        failed_guards = [r.guard_name for r in guard_results if not r.passed]

        if not failed_guards:
            return "Risk score below approval threshold"

        return f"Failed guard checks: {', '.join(failed_guards)}. See evidence for details."
