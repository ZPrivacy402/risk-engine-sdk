# ZPrivacy402 Risk Engine SDK

**Production-grade risk protection for autonomous AI agent payments on Solana**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)](https://www.rust-lang.org)
[![TypeScript](https://img.shields.io/badge/typescript-5.0%2B-blue)](https://www.typescriptlang.org)
[![Python](https://img.shields.io/badge/python-3.11%2B-green)](https://www.python.org)
[![CI/CD](https://img.shields.io/github/actions/workflow/status/zprivacy402/risk-engine-sdk/ci.yml)](https://github.com/zprivacy402/risk-engine-sdk/actions)

[Documentation](https://zprivacy402.com/docs) • [Examples](./examples) • [API Reference](https://zprivacy402.com/docs)

</div>

## 🚀 Overview

ZPrivacy402 Risk Engine SDK is an open-source, production-ready framework that adds a comprehensive trust layer to autonomous AI agent transactions on Solana blockchain. It combines advanced risk detection mechanisms with **zero-knowledge STARK proofs** for privacy-preserving transaction verification.

### Key Features

- 🛡️ **TRUSTLINE Framework**: Multi-layered risk protection against prompt injection, counterfeit routes, subscription traps, and tool-chain tampering
- 🔒 **zk-STARK Privacy**: Post-quantum secure transaction verification using Winterfell and Bonsol integration
- ⚡ **High Performance**: Sub-second risk assessment with parallel proof generation
- 🌐 **Multi-Language Support**: Native SDKs for TypeScript, Python, and Rust
- 📊 **Real-time Analytics**: Streaming behavioral analysis with ML-based fraud detection
- 🔗 **Blockchain Agnostic**: Modular architecture with Solana support (Ethereum/Polygon adapters coming soon)
- 🏛️ **Enterprise Ready**: Production-grade monitoring, audit trails, and compliance tools

## 📦 Installation

### TypeScript/JavaScript

```bash
npm install @zprivacy402/sdk
# or
yarn add @zprivacy402/sdk
# or
pnpm add @zprivacy402/sdk
```

### Python

```bash
pip install zprivacy402
# or
poetry add zprivacy402
```

### Rust

```toml
[dependencies]
zprivacy402 = "1.0.0"
```

## 🏃 Quick Start

### TypeScript Example

```typescript
import { RiskEngine, SolanaProvider } from '@zprivacy402/sdk';

// Initialize the risk engine
const engine = new RiskEngine({
  provider: new SolanaProvider({
    rpcUrl: process.env.SOLANA_RPC_URL,
  }),
  zkProofEnabled: true,
  privacyLevel: 'high',
});

// Assess transaction risk
const result = await engine.assessTransaction({
  amount: 100,
  recipient: '7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU',
  agentContext: {
    intent: 'Purchase laptop under $1500',
    executionTrace: traces,
  },
});

if (result.approved) {
  // Generate privacy-preserving proof
  const proof = await engine.generateProof(result);
  
  // Submit to Solana
  const signature = await engine.submitTransaction(proof);
  console.log(`Transaction approved: ${signature}`);
} else {
  console.log(`Transaction blocked: ${result.reason}`);
}
```

### Python Example

```python
from zprivacy402 import RiskEngine, SolanaProvider

# Initialize risk engine
engine = RiskEngine(
    provider=SolanaProvider(rpc_url=os.environ['SOLANA_RPC_URL']),
    zk_proof_enabled=True,
    privacy_level='high'
)

# Assess transaction
result = engine.assess_transaction(
    amount=100,
    recipient='7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU',
    agent_context={
        'intent': 'Purchase laptop under $1500',
        'execution_trace': traces
    }
)

if result.approved:
    # Generate zk-STARK proof
    proof = engine.generate_proof(result)
    
    # Submit to blockchain
    signature = engine.submit_transaction(proof)
    print(f'Transaction approved: {signature}')
else:
    print(f'Transaction blocked: {result.reason}')
```

### Rust Example

```rust
use zprivacy402::{RiskEngine, SolanaProvider, TransactionRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize risk engine
    let engine = RiskEngine::builder()
        .provider(SolanaProvider::new(rpc_url))
        .enable_zk_proofs(true)
        .privacy_level(PrivacyLevel::High)
        .build()?;

    // Create transaction request
    let request = TransactionRequest {
        amount: 100,
        recipient: "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU".parse()?,
        agent_context: AgentContext {
            intent: "Purchase laptop under $1500".to_string(),
            execution_trace: traces,
        },
    };

    // Assess risk
    let result = engine.assess(&request).await?;

    if result.approved {
        // Generate zk-STARK proof
        let proof = engine.generate_proof(&result).await?;
        
        // Submit to Solana
        let signature = engine.submit(proof).await?;
        println!("Transaction approved: {}", signature);
    } else {
        println!("Transaction blocked: {}", result.reason);
    }

    Ok(())
}
```

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     AI Agent Application                     │
└────────────────────┬────────────────────────────────────────┘
                     │
         ┌───────────▼───────────┐
         │   ZPrivacy402 SDK     │
         │  (TS/Python/Rust)     │
         └───────────┬───────────┘
                     │
    ┌────────────────┴────────────────┐
    │                                 │
┌───▼──────────┐           ┌─────────▼─────────┐
│ Risk Engine  │           │   zk-STARK        │
│  (Rust Core) │◄─────────►│   Prover          │
│              │           │  (Winterfell)     │
│ • Intent     │           └─────────┬─────────┘
│   Guard      │                     │
│ • Route      │           ┌─────────▼─────────┐
│   Guard      │           │  Bonsol Client    │
│ • Subscription│          │  (Solana Bridge)  │
│   Guard      │           └─────────┬─────────┘
│ • Toolchain  │                     │
│   Guard      │                     │
└──────┬───────┘                     │
       │                             │
       │         ┌───────────────────▼────────────────┐
       └────────►│      Solana Blockchain            │
                 │  • On-chain Verification          │
                 │  • State Commitments              │
                 │  • Privacy-Preserving Audit Trail │
                 └────────────────────────────────────┘
```

## 🛡️ TRUSTLINE Protection Mechanisms

### 1. **Intent Guard** - Prompt Injection Detection
Analyzes agent reasoning chains and execution traces for evidence of malicious prompt manipulation.

```typescript
// Detects hidden system prompts, URL rewrites, affiliate coercion
const intentCheck = await engine.guards.intent.verify(context);
```

### 2. **Route Guard** - Counterfeit Route Protection
Validates merchant identity, seller authorization, and wallet ownership against trusted registries.

```typescript
// Prevents gray-market sellers and unauthorized payment routes
const routeCheck = await engine.guards.route.verify(merchant);
```

### 3. **Subscription Guard** - Auto-Renew Trap Prevention
Parses checkout terms, detects recurring payment flags, and enforces mandate scope validation.

```typescript
// Blocks hidden subscriptions and unexpected renewals
const subCheck = await engine.guards.subscription.verify(terms);
```

### 4. **Toolchain Guard** - Tool Tampering Detection
Monitors API middleware, validates beneficiary identifiers, and ensures tenant isolation.

```typescript
// Prevents payment redirection and tenant ID swapping
const toolchainCheck = await engine.guards.toolchain.verify(invoiceData);
```

## 🔐 zk-STARK Integration

### Why STARKs?

- **Post-Quantum Security**: Relies only on collision-resistant hash functions
- **Transparent Setup**: No trusted ceremony required
- **Scalable Verification**: Logarithmic verification time for any computation size
- **Privacy-Preserving**: Prove transaction validity without revealing sensitive data

### Proof Generation

```typescript
import { ZKProver, ProofOptions } from '@zprivacy402/sdk';

const prover = new ZKProver({
  backend: 'winterfell',
  securityLevel: 128, // bits
  hashFunction: 'rescue-prime',
  compressionEnabled: true, // STARK → Groth16 via Bonsol
});

// Generate proof for risk assessment
const proof = await prover.prove({
  publicInputs: {
    riskScore: 95,
    approvalStatus: true,
    timestamp: Date.now(),
  },
  privateInputs: {
    executionTrace: agentTrace,
    sensitiveData: privateContext,
  },
  options: {
    soundnessError: 2n ** -120n,
    queryComplexity: 'standard',
  },
});
```

### On-Chain Verification

```rust
use zprivacy402::zk::{Verifier, BonsolClient};

// Submit proof to Solana for verification
let bonsol = BonsolClient::new(rpc_url);
let verification = bonsol.verify_proof(
    proof,
    program_id,
    compute_budget_cu: 1_400_000, // Within Solana CU limits
).await?;

assert!(verification.is_valid);
```

## 📊 Advanced Features

### Real-Time Behavioral Analytics

```typescript
import { BehavioralAnalyzer } from '@zprivacy402/sdk';

const analyzer = new BehavioralAnalyzer({
  kafkaEndpoint: process.env.KAFKA_URL,
  modelType: 'onnx',
  features: ['transaction_velocity', 'amount_pattern', 'time_of_day'],
});

// Stream-based anomaly detection
analyzer.on('anomaly', (event) => {
  console.log(`Suspicious behavior detected: ${event.reason}`);
  engine.escalate(event);
});
```

### Multi-Chain Support

```typescript
import { EthereumProvider, PolygonProvider } from '@zprivacy402/sdk';

// Switch between chains seamlessly
const engine = new RiskEngine({
  provider: process.env.CHAIN === 'ethereum' 
    ? new EthereumProvider(config)
    : new SolanaProvider(config),
});
```

### Privacy-Preserving Audit Trail

```typescript
import { AuditLogger } from '@zprivacy402/sdk';

const logger = new AuditLogger({
  storage: 'ipfs', // or 'arweave', 's3'
  merkleCommitments: true,
  selectiveDisclosure: true,
});

// Log events with cryptographic proof
await logger.log({
  event: 'transaction_approved',
  riskScore: 95,
  proofCommitment: merkleRoot,
  revealKeys: ['timestamp', 'riskScore'], // Only these fields can be revealed
});
```

## 🧪 Testing

```bash
# Run all tests
npm test

# Run specific test suite
npm test -- --grep "RiskEngine"

# Run with coverage
npm run test:coverage

# Benchmark zk-STARK performance
npm run benchmark:zk

# Integration tests with Solana test validator
npm run test:integration
```

## 📚 Documentation

- [Getting Started Guide](https://zprivacy402.com/docs)

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# Clone repository
git clone https://github.com/ZPrivacy402/risk-engine-sdk
cd risk-engine-sdk

# Install dependencies
pnpm install

# Build all packages
pnpm build

# Run development mode
pnpm dev
```

## 📜 License

This project is licensed under the MIT License - see [LICENSE](./LICENSE) file for details.

## 🔗 Links

- **Website**: https://zprivacy402.com
- **Documentation**: https://zprivacy402.com/docs
- **Twitter/X**: [@ZPrivacy402](https://x.com/ZPrivacy402)
- **Email**: support@zprivacy402.com

## 📈 Roadmap

- [x] Core risk engine implementation
- [x] zk-STARK proof generation (Winterfell)
- [x] Solana integration via Bonsol
- [x] TypeScript/Python/Rust SDKs
- [ ] Ethereum L2 support (Q1 2026)
- [ ] Decentralized validator network (Q2 2026)
- [ ] Mobile SDK (iOS/Android) (Q3 2026)
- [ ] Hardware wallet integration (Q4 2026)

---

<div align="center">

**Built with ❤️ by the ZPrivacy402 team**

If you find this project useful, please consider giving it a ⭐!

</div>
