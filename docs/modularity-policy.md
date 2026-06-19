# BCX Modularity Policy

Status: active policy

BCX must remain a crate ecosystem, not one giant crate.

Rules:

- The root `bcx` crate is a facade and stable user entry point.
- Protocol model code belongs in focused subcrates.
- Transport bindings are separate crates.
- Crypto provider integrations are separate crates.
- Storage backends are separate crates.
- CLI and Fluxheim integration are separate from the root protocol crate.
- Non-generated Rust files must stay under 500 lines.
- Prefer files under 300 lines when practical.
- Keep `lib.rs` as module wiring and public API shaping, not implementation
  dumping ground.

The local gate runs `scripts/validate-modularity-policy.sh`.
