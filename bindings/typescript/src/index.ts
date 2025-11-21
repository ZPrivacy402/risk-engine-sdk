export { RiskEngine, RiskEngineConfig } from './risk-engine';
export { ZKProver, ProofOptions } from './zk-prover';
export { SolanaProvider, ProviderConfig } from './providers/solana';
export {
  TransactionRequest,
  RiskAssessment,
  AgentContext,
  ExecutionTrace,
  ToolCall,
  GuardResult,
  Proof,
  PublicInputs,
  ProofMetadata,
} from './types';
export { IntentGuard, RouteGuard, SubscriptionGuard, ToolchainGuard } from './guards';
export { BehavioralAnalyzer } from './behavioral';
export { AuditLogger } from './audit';

export const version = '1.0.0';
