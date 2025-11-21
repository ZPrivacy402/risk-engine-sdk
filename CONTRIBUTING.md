# Contributing to ZPrivacy402 Risk Engine SDK

First off, thank you for considering contributing to ZPrivacy402! It's people like you that make ZPrivacy402 such a great tool for securing autonomous AI agent payments.

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues as you might find that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* **Use a clear and descriptive title**
* **Describe the exact steps which reproduce the problem**
* **Provide specific examples to demonstrate the steps**
* **Describe the behavior you observed after following the steps**
* **Explain which behavior you expected to see instead and why**
* **Include logs and error messages**

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* **Use a clear and descriptive title**
* **Provide a step-by-step description of the suggested enhancement**
* **Provide specific examples to demonstrate the steps**
* **Describe the current behavior and explain which behavior you expected to see instead**
* **Explain why this enhancement would be useful**

### Pull Requests

* Fill in the required template
* Do not include issue numbers in the PR title
* Include screenshots and animated GIFs in your pull request whenever possible
* Follow the Rust, TypeScript, and Python style guides
* Include thoughtfully-worded, well-structured tests
* Document new code based on the Documentation Style Guide
* End all files with a newline

## Development Process

### Setup Development Environment

```bash
# Clone your fork
git clone https://github.com/your-username/risk-engine-sdk
cd risk-engine-sdk

# Add upstream remote
git remote add upstream https://github.com/ZPrivacy402/risk-engine-sdk

# Install dependencies
pnpm install
cargo build
poetry install

# Run tests
pnpm test
cargo test
pytest
```

### Branch Naming Convention

* `feature/` - New features
* `fix/` - Bug fixes
* `docs/` - Documentation only changes
* `refactor/` - Code refactoring
* `test/` - Adding or updating tests
* `chore/` - Maintenance tasks

Example: `feature/zk-proof-compression`

### Commit Message Guidelines

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
* `feat`: A new feature
* `fix`: A bug fix
* `docs`: Documentation only changes
* `style`: Code style changes (formatting, missing semicolons, etc)
* `refactor`: Code refactoring
* `perf`: Performance improvements
* `test`: Adding or updating tests
* `chore`: Maintenance tasks

**Examples:**
```
feat(risk-engine): add prompt injection detection

Implement intent guard to detect malicious prompt manipulation in AI agent execution traces.

Closes #123
```

```
fix(zk-prover): resolve memory leak in proof generation

Fixed memory leak in Winterfell proof generation loop that caused OOM errors on large traces.

Fixes #456
```

### Code Style

#### Rust

* Follow [Rust Style Guidelines](https://doc.rust-lang.org/1.0.0/style/)
* Run `cargo fmt` before committing
* Run `cargo clippy` and address all warnings
* Maintain code coverage above 80%

#### TypeScript

* Follow [TypeScript Style Guide](https://google.github.io/styleguide/tsguide.html)
* Run `npm run lint` before committing
* Use Prettier for formatting
* Write JSDoc comments for public APIs

#### Python

* Follow [PEP 8](https://www.python.org/dev/peps/pep-0008/)
* Use [Black](https://black.readthedocs.io/) for formatting
* Use [mypy](http://mypy-lang.org/) for type checking
* Write docstrings for all public functions

### Testing Requirements

All contributions must include appropriate tests:

* **Unit Tests**: Test individual components in isolation
* **Integration Tests**: Test component interactions
* **Property Tests**: Use property-based testing for critical algorithms
* **ZK Proof Tests**: Include test vectors for cryptographic operations
* **Benchmarks**: Add benchmarks for performance-critical code

```bash
# Run all tests
pnpm test:all

# Run specific test suite
cargo test --package ZPrivacy402/risk-engine-sdk
pytest tests/unit/test_prover.py
npm test -- --testPathPattern=guards

# Run benchmarks
cargo bench
npm run benchmark
```

### Documentation

* Update README.md if adding new features
* Add JSDoc/rustdoc comments for public APIs
* Update architecture docs for significant changes
* Include code examples in documentation
* Update CHANGELOG.md following [Keep a Changelog](https://keepachangelog.com/)

### Security

* **Never commit secrets or API keys**
* Report security vulnerabilities to security@zprivacy402.com
* Use secure coding practices
* Follow [OWASP guidelines](https://owasp.org/)
* Add security tests for new features

### Review Process

1. Create a pull request from your fork
2. Ensure all CI checks pass
3. Request review from maintainers
4. Address review feedback
5. Maintainers will merge after approval

### Release Process

Releases are handled by maintainers:

1. Update version in `Cargo.toml`, `package.json`, `pyproject.toml`
2. Update CHANGELOG.md
3. Create release tag: `git tag -a v1.0.0 -m "Release 1.0.0"`
4. Push tags: `git push --tags`
5. GitHub Actions will publish to crates.io, npm, and PyPI

## Recognition

Contributors will be recognized in:

* CONTRIBUTORS.md file
* Release notes
* Project website

## Questions?

* Email: support@zprivacy402.com
* Twitter: [@ZPrivacy402](https://x.com/ZPrivacy402)

Thank you for contributing to ZPrivacy402! 🚀
