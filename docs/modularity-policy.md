# BCX Modularity Policy

Status: active policy

BCX must remain a crate ecosystem, not one giant crate.

Rules:

- The root `bcx` crate is a facade and stable user entry point.
- Protocol model code belongs in focused subcrates.
- Carrier, binding, observer, settlement, identity, storage, and proof profiles
  are separate crates.
- Concrete integrations are separate from normative profiles.
- Crypto provider integrations are separate crates.
- Storage backends are separate crates.
- CLI, Fluxheim, Skrifheim, HTTP framework, blockchain RPC, and service
  integrations are separate from the root protocol crate.
- Domain semantics such as banking or AI-agent behavior belong in domain
  profile crates, not in the invariant core.
- The core never depends on HTTP, QUIC, blockchain, async runtime, database, or
  service crates.
- Non-generated Rust files must stay under 500 lines.
- Prefer files under 300 lines when practical.
- Keep `lib.rs` as module wiring and public API shaping, not implementation
  dumping ground.

The local gate runs `scripts/validate-modularity-policy.sh`.
