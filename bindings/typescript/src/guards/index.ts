import { TransactionRequest, GuardResult, Evidence } from '../types';

export abstract class Guard {
  abstract name: string;
  abstract evaluate(request: TransactionRequest): Promise<GuardResult>;
}

export class IntentGuard extends Guard {
  name = 'intent';

  async evaluate(request: TransactionRequest): Promise<GuardResult> {
    const evidence: Evidence[] = [];
    let score = 100;
    let passed = true;

    const maliciousPatterns = [
      /ignore\s+(previous|all)\s+instructions?/i,
      /system\s*:\s*you\s+are/i,
      /disregard\s+(above|previous)/i,
      /forget\s+everything/i,
    ];

    for (const pattern of maliciousPatterns) {
      if (pattern.test(request.agentContext.intent)) {
        score -= 40;
        passed = false;
        evidence.push({
          evidenceType: 'prompt_injection_detected',
          data: { pattern: pattern.source },
          weight: 0.8,
        });
        break;
      }
    }

    return {
      guardName: this.name,
      passed,
      score,
      details: passed ? 'Intent analysis passed' : 'Prompt injection detected',
      evidence,
    };
  }
}

export class RouteGuard extends Guard {
  name = 'route';

  async evaluate(request: TransactionRequest): Promise<GuardResult> {
    const evidence: Evidence[] = [];
    let score = 100;
    let passed = true;

    const isAuthorized = request.recipient.startsWith('verified_');

    if (!isAuthorized) {
      score -= 30;
      evidence.push({
        evidenceType: 'unauthorized_merchant',
        data: { recipient: request.recipient },
        weight: 0.7,
      });
    }

    return {
      guardName: this.name,
      passed,
      score,
      details: passed ? 'Route verification passed' : 'Unauthorized merchant',
      evidence,
    };
  }
}

export class SubscriptionGuard extends Guard {
  name = 'subscription';

  async evaluate(request: TransactionRequest): Promise<GuardResult> {
    const evidence: Evidence[] = [];
    let score = 100;
    let passed = true;

    if (request.metadata?.checkout_terms) {
      const terms = request.metadata.checkout_terms as Record<string, unknown>;
      if (terms.recurring === true && !request.agentContext.intent.toLowerCase().includes('subscription')) {
        score -= 50;
        passed = false;
        evidence.push({
          evidenceType: 'hidden_subscription',
          data: { terms },
          weight: 0.9,
        });
      }
    }

    return {
      guardName: this.name,
      passed,
      score,
      details: passed ? 'Subscription check passed' : 'Hidden subscription detected',
      evidence,
    };
  }
}

export class ToolchainGuard extends Guard {
  name = 'toolchain';

  async evaluate(request: TransactionRequest): Promise<GuardResult> {
    const evidence: Evidence[] = [];
    let score = 100;
    let passed = true;

    for (const toolCall of request.agentContext.toolCalls) {
      if (toolCall.toolName === 'create_invoice') {
        const result = toolCall.result as Record<string, unknown>;
        if (result.tenant_id !== request.metadata?.expected_tenant) {
          score -= 60;
          passed = false;
          evidence.push({
            evidenceType: 'tenant_id_swap',
            data: { toolCall },
            weight: 0.95,
          });
        }
      }
    }

    return {
      guardName: this.name,
      passed,
      score,
      details: passed ? 'Toolchain integrity verified' : 'Toolchain tampering detected',
      evidence,
    };
  }
}
