# Contributing To BCX

BCX is security-sensitive protocol infrastructure. Contributions are welcome
when they keep the project small, clear, tested, and honest about what is
stable.

## License

BCX is licensed under the European Union Public Licence 1.2. By contributing,
you agree that your contribution is provided under the same license.

## Development Setup

Use the pinned Rust toolchain from `rust-toolchain.toml`.

```bash
cargo test --workspace
scripts/checks.sh
```

## Security-Sensitive Changes

Treat these areas as high risk:

- canonical encoding and decoding;
- signature envelopes and verifier traits;
- capability and replay handling;
- admission and effect receipts;
- WHY query traversal and disclosure;
- HTTP and native transport bindings;
- dependency updates.

## Project Shape

The root `bcx` crate should stay no-std and dependency-light. Add optional
ecosystem or transport behavior in focused subcrates.
