export interface TransactionRequest {
  amount: number;
  recipient: string;
  agentContext: AgentContext;
  metadata?: Record<string, unknown>;
}

export interface AgentContext {
  intent: string;
  executionTrace: ExecutionTrace[];
  reasoningChain: string[];
  toolCalls: ToolCall[];
  environment?: Record<string, string>;
}

export interface ExecutionTrace {
  timestamp: number;
  action: string;
  input: unknown;
  output: unknown;
  metadata?: Record<string, string>;
}

export interface ToolCall {
  toolName: string;
  arguments: unknown;
  result: unknown;
  timestamp: number;
}

export interface RiskAssessment {
  approved: boolean;
  riskScore: RiskScore;
  guardResults: GuardResult[];
  reason?: string;
  timestamp: number;
  evidence: Evidence[];
}

export interface RiskScore {
  overall: number;
  breakdown: ScoreBreakdown;
}

export interface ScoreBreakdown {
  intentScore: number;
  routeScore: number;
  subscriptionScore: number;
  toolchainScore: number;
  behavioralScore: number;
}

export interface GuardResult {
  guardName: string;
  passed: boolean;
  score: number;
  details: string;
  evidence: Evidence[];
}

export interface Evidence {
  evidenceType: string;
  data: unknown;
  weight: number;
}

export interface Proof {
  starkProof: Uint8Array;
  publicInputs: PublicInputs;
  proofMetadata: ProofMetadata;
}

export interface PublicInputs {
  riskScore: number;
  approvalStatus: boolean;
  timestamp: number;
  transactionHash: string;
}

export interface ProofMetadata {
  securityLevel: number;
  proofSizeBytes: number;
  generationTimeMs: number;
  soundnessError: string;
}
