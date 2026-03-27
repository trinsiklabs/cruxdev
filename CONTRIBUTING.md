# Contributing to CruxDev

## Reporting Issues

Found a bug or have a feature request? File an issue at:
https://github.com/trinsiklabs/cruxdev/issues/new

If you're using CruxDev on your project and the adoption tools find a gap that's a CruxDev problem (not your project's problem), please report it. This helps improve the tools for everyone.

## Development Setup

```bash
git clone https://github.com/trinsiklabs/cruxdev.git
cd cruxdev/rust
cargo test          # Run all tests
cargo clippy -- -D warnings  # Zero warnings required
cargo build --release  # Build binary
```

## Code Standards

- **TDD** — tests before code, no exceptions
- **100% test coverage** — enforced
- **Zero clippy warnings** — enforced
- **Two consecutive clean passes** = convergence
- **Convergence gate must pass** — `./scripts/convergence_gate.sh`

## Pull Request Process

1. Create a build plan describing what you're changing and why
2. Implement with tests
3. Run `./scripts/convergence_gate.sh` — must pass
4. Submit PR with build plan reference

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
