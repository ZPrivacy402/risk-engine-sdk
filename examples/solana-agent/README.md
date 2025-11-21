# Solana AI Agent Example

This example demonstrates how to integrate ZPrivacy402 Risk Engine with a Solana-based AI agent for secure autonomous transactions.

## Features

- Real-time risk assessment before transactions
- zk-STARK proof generation for privacy
- On-chain verification via Bonsol
- TRUSTLINE guard protection
- Transaction monitoring and audit logging

## Installation

```bash
cd examples/solana-agent
npm install
```

## Configuration

Create a `.env` file:

```env
SOLANA_RPC_URL=https://api.devnet.solana.com
WALLET_PRIVATE_KEY=your_private_key_here
RISK_ENGINE_THRESHOLD=70
ZK_PROOFS_ENABLED=true
```

## Usage

### TypeScript

```typescript
import { RiskEngine, SolanaProvider, ZKProver } from '@zprivacy402/sdk';

const engine = new RiskEngine({
  approvalThreshold: 70,
});

const provider = new SolanaProvider({
  rpcUrl: process.env.SOLANA_RPC_URL!,
});

const prover = new ZKProver({
  securityLevel: 128,
});

const request = {
  amount: 100,
  recipient: '7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU',
  agentContext: {
    intent: 'Purchase laptop under $1500',
    executionTrace: [],
    reasoningChain: ['Compare prices', 'Select best option'],
    toolCalls: [],
  },
};

const assessment = await engine.assessTransaction(request);

if (assessment.approved) {
  const proof = await prover.generateProof({
    riskScore: assessment.riskScore.overall,
    approvalStatus: true,
    timestamp: Date.now(),
    transactionHash: 'tx_hash',
  });

  console.log('Transaction approved with proof:', proof);
} else {
  console.log('Transaction rejected:', assessment.reason);
}
```

### Python

```python
import asyncio
from zprivacy402 import RiskEngine, SolanaProvider, ZKProver
from zprivacy402.types import TransactionRequest, AgentContext

async def main():
    engine = RiskEngine()
    provider = SolanaProvider(rpc_url="https://api.devnet.solana.com")
    prover = ZKProver()

    request = TransactionRequest(
        amount=100,
        recipient="7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
        agent_context=AgentContext(
            intent="Purchase laptop under $1500",
            execution_trace=[],
            reasoning_chain=["Compare prices", "Select best option"],
            tool_calls=[]
        )
    )

    assessment = await engine.assess_transaction(request)

    if assessment.approved:
        print(f"Transaction approved: {assessment.risk_score.overall}/100")
    else:
        print(f"Transaction rejected: {assessment.reason}")

if __name__ == "__main__":
    asyncio.run(main())
```

## Running the Example

```bash
# TypeScript
npm run start

# Python
python main.py
```

## Architecture

```
AI Agent → Risk Engine → Guards → Risk Score
                ↓
          zk-STARK Prover
                ↓
          Bonsol Client
                ↓
       Solana Blockchain
```

## Learn More

- [Documentation](https://zprivacy402.com/docs)
