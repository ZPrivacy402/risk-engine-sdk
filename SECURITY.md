# Security Policy

## Supported Versions

We release patches for security vulnerabilities for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 1.x.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to: **security@zprivacy402.com**

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

Please include the following information:

* Type of vulnerability (e.g., cryptographic weakness, injection attack, etc.)
* Full paths of source file(s) related to the manifestation of the vulnerability
* Location of the affected source code (tag/branch/commit or direct URL)
* Step-by-step instructions to reproduce the vulnerability
* Proof-of-concept or exploit code (if possible)
* Impact of the vulnerability, including how an attacker might exploit it

## Security Disclosure Process

When we receive a security bug report, we will:

1. **Confirm the problem** and determine affected versions
2. **Audit code** to find similar problems
3. **Prepare fixes** for all supported versions
4. **Release new versions** as soon as possible
5. **Publish security advisory** with credit to reporter (if desired)

## Security Best Practices

### For SDK Users

1. **Keep Updated**: Always use the latest version of the SDK
2. **Secure API Keys**: Never commit API keys or secrets to version control
3. **Use Environment Variables**: Store sensitive configuration in environment variables
4. **Enable zk-Proofs**: Use privacy-preserving proofs for sensitive transactions
5. **Validate Inputs**: Always validate and sanitize agent inputs
6. **Monitor Transactions**: Set up alerts for unusual transaction patterns
7. **Review Logs**: Regularly audit transaction logs for suspicious activity

### For Contributors

1. **Code Review**: All PRs require security review for cryptographic code
2. **Dependency Scanning**: Automated scanning via Snyk and cargo-audit
3. **Static Analysis**: Use clippy (Rust), ESLint (TS), and Bandit (Python)
4. **Fuzzing**: Run fuzzing tests on critical paths
5. **Secret Scanning**: Automated secret detection in commits
6. **Signed Commits**: Use GPG-signed commits for maintainers

## Cryptographic Security

### zk-STARK Parameters

* **Security Level**: 128-bit minimum
* **Soundness Error**: ≤ 2^-120
* **Hash Function**: Rescue-Prime (post-quantum secure)
* **Field**: Prime field with ≥ 256-bit modulus

### Key Management

* **Wallet Security**: Use hardware wallets for production keys
* **Key Rotation**: Rotate signing keys every 90 days
* **Access Control**: Implement multi-sig for critical operations
* **Backup**: Maintain encrypted backups of all keys

## Known Security Considerations

### Current Limitations

1. **Proof Size**: STARK proofs are larger than SNARKs (trade-off for transparency)
2. **Verification Cost**: On-chain verification requires ~1.4M compute units
3. **Side Channels**: Timing attacks possible on proof generation (use constant-time operations)

### Mitigations

* Proof compression via STARK → Groth16 conversion
* Batch verification for multiple proofs
* Constant-time cryptographic implementations
* Rate limiting on proof generation endpoints

## Security Audits

| Date | Auditor | Scope | Report |
|------|---------|-------|--------|
| TBD  | Trail of Bits | Core risk engine | Pending |
| TBD  | OpenZeppelin | zk-STARK prover | Pending |
| TBD  | Kudelski Security | Full platform | Pending |

## Bug Bounty Program

We are planning to launch a bug bounty program with rewards up to:

* **Critical**: $10,000
* **High**: $5,000
* **Medium**: $2,000
* **Low**: $500

Details will be published at https://zprivacy402.com/docs

## Compliance

* **SOC 2 Type II**: Planned for Q2 2026
* **GDPR**: Privacy-by-design architecture
* **PCI DSS**: Payment card data handling (if applicable)

## Contact

* **Security Email**: security@zprivacy402.com
* **PGP Key**: Available at https://zprivacy402.com/docs
* **Security Updates**: Follow [@ZPrivacy402](https://x.com/ZPrivacy402)

---

Thank you for helping keep ZPrivacy402 and our users safe!
