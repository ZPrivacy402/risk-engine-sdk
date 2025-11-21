import {
  TransactionRequest,
  RiskAssessment,
  GuardResult,
  RiskScore,
  ScoreBreakdown,
} from './types';
import { IntentGuard, RouteGuard, SubscriptionGuard, ToolchainGuard } from './guards';

export interface RiskEngineConfig {
  approvalThreshold?: number;
  guards?: {
    intentGuardEnabled?: boolean;
    routeGuardEnabled?: boolean;
    subscriptionGuardEnabled?: boolean;
    toolchainGuardEnabled?: boolean;
  };
}

export class RiskEngine {
  private config: Required<RiskEngineConfig>;
  private intentGuard: IntentGuard;
  private routeGuard: RouteGuard;
  private subscriptionGuard: SubscriptionGuard;
  private toolchainGuard: ToolchainGuard;

  constructor(config: RiskEngineConfig = {}) {
    this.config = {
      approvalThreshold: config.approvalThreshold ?? 70,
      guards: {
        intentGuardEnabled: config.guards?.intentGuardEnabled ?? true,
        routeGuardEnabled: config.guards?.routeGuardEnabled ?? true,
        subscriptionGuardEnabled: config.guards?.subscriptionGuardEnabled ?? true,
        toolchainGuardEnabled: config.guards?.toolchainGuardEnabled ?? true,
      },
    };

    this.intentGuard = new IntentGuard();
    this.routeGuard = new RouteGuard();
    this.subscriptionGuard = new SubscriptionGuard();
    this.toolchainGuard = new ToolchainGuard();
  }

  async assessTransaction(request: TransactionRequest): Promise<RiskAssessment> {
    const guardResults: GuardResult[] = [];

    if (this.config.guards.intentGuardEnabled) {
      const result = await this.intentGuard.evaluate(request);
      guardResults.push(result);
    }

    if (this.config.guards.routeGuardEnabled) {
      const result = await this.routeGuard.evaluate(request);
      guardResults.push(result);
    }

    if (this.config.guards.subscriptionGuardEnabled) {
      const result = await this.subscriptionGuard.evaluate(request);
      guardResults.push(result);
    }

    if (this.config.guards.toolchainGuardEnabled) {
      const result = await this.toolchainGuard.evaluate(request);
      guardResults.push(result);
    }

    const riskScore = this.calculateRiskScore(guardResults, request);
    const approved = riskScore.overall >= this.config.approvalThreshold;

    const evidence = guardResults.flatMap((r) => r.evidence);

    const reason = approved ? undefined : this.generateRejectionReason(guardResults);

    return {
      approved,
      riskScore,
      guardResults,
      reason,
      timestamp: Date.now(),
      evidence,
    };
  }

  private calculateRiskScore(guardResults: GuardResult[], request: TransactionRequest): RiskScore {
    const weights = {
      intent: 0.25,
      route: 0.25,
      subscription: 0.2,
      toolchain: 0.2,
      behavioral: 0.1,
    };

    let intentScore = 100;
    let routeScore = 100;
    let subscriptionScore = 100;
    let toolchainScore = 100;
    const behavioralScore = this.calculateBehavioralScore(request);

    for (const result of guardResults) {
      switch (result.guardName) {
        case 'intent':
          intentScore = result.score;
          break;
        case 'route':
          routeScore = result.score;
          break;
        case 'subscription':
          subscriptionScore = result.score;
          break;
        case 'toolchain':
          toolchainScore = result.score;
          break;
      }
    }

    const overall = Math.floor(
      intentScore * weights.intent +
        routeScore * weights.route +
        subscriptionScore * weights.subscription +
        toolchainScore * weights.toolchain +
        behavioralScore * weights.behavioral
    );

    return {
      overall,
      breakdown: {
        intentScore,
        routeScore,
        subscriptionScore,
        toolchainScore,
        behavioralScore,
      },
    };
  }

  private calculateBehavioralScore(request: TransactionRequest): number {
    if (request.amount > 500) {
      return 75 + Math.floor(Math.random() * 10);
    } else if (request.amount > 100) {
      return 85 + Math.floor(Math.random() * 10);
    } else {
      return 95 + Math.floor(Math.random() * 5);
    }
  }

  private generateRejectionReason(guardResults: GuardResult[]): string {
    const failedGuards = guardResults.filter((r) => !r.passed).map((r) => r.guardName);

    if (failedGuards.length === 0) {
      return 'Risk score below approval threshold';
    }

    return `Failed guard checks: ${failedGuards.join(', ')}. See evidence for details.`;
  }
}
