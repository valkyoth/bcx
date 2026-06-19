# bcx

`bcx` is the Rust crate for **Bifrost Causal Exchange**: a no-std-first
protocol foundation for signed causal meaning, proof composition, native
bindings, checkpoints, and bounded explanations across multiple underlying
systems.

BCX is not an HTTP replacement, an Ethereum protocol, or a Cardano protocol. It
is a semantic overlay layer. BCX defines what an operation means, why it was
authorized, what it caused, who attested to it, what evidence exists, and what
remains unknown. HTTP, QUIC, Ethereum, Cardano, Fluxheim, Skrifheim, offline
bundles, and future systems can carry, observe, store, or settle BCX objects
through explicit profiles and integrations.

## Current Status

`bcx` is at repository foundation stage. The crate is not production-ready.

Implemented now:

- `no_std` default build.
- zero third-party runtime dependencies.
- root publishable crate named `bcx`.
- focused no-std subcrates for core IDs, model types, crypto envelope metadata,
  policy profiles, and wire limits.
- local check script, modularity guard, release metadata guard, and GitHub CI.
- implementation, version, protocol-family, threat-model, toolchain, and
  modularity docs.
- repository families for future profiles, integrations, proofs, domain
  profiles, and services.

## Trust Dashboard

| Area | Status |
| --- | --- |
| License | `EUPL-1.2` |
| MSRV | Rust `1.90.0` |
| Preferred toolchain | Rust `1.96.0` |
| Default target | `no_std` |
| Runtime dependencies | zero third-party crates |
| Unsafe policy | `unsafe_code = "forbid"` |
| Main crate | `bcx` |
| First serious target | `1.0.0` production-ready protocol crate |
| High-security position | trust canonical signed BCX objects, not one carrier or chain |

## Rust Version Support

The minimum supported Rust version is Rust `1.90.0`. New deployments should
prefer the latest stable Rust validated by the project.

Compatibility evidence:

| Rust | Local Evidence |
| --- | --- |
| `1.90.0` | planned full check gate |
| `1.91.0` | planned `cargo check --all-features` |
| `1.92.0` | planned `cargo check --all-features` |
| `1.93.0` | planned `cargo check --all-features` |
| `1.94.0` | planned `cargo check --all-features` |
| `1.95.0` | planned `cargo check --all-features` |
| `1.96.0` | current pinned toolchain |

## Workspace Shape

| Crate | Purpose |
| --- | --- |
| `bcx` | Published facade crate and stable user entry point. |
| `bcx-core` | Identifiers, nonces, replay sequence primitives, validation errors. |
| `bcx-model` | Statements, causal edges, admissions, effects, truth, and assurance model types. |
| `bcx-crypto` | Crypto-agile signature envelope metadata and verifier traits. |
| `bcx-policy` | Protocol profiles, proof levels, and disclosure levels. |
| `bcx-wire` | Wire version and bounded-message limit primitives. |

Future crates are organized by family:

| Family | Purpose |
| --- | --- |
| `profiles/` | Normative mappings such as HTTP, QUIC, Ethereum, Cardano, offline, SCITT, or future Bitcoin/XRP profiles. |
| `integrations/` | Concrete adapters such as Hyper, Axum, Alloy, Pallas, Fluxheim, or Skrifheim. |
| `proofs/` | Concrete proof suites such as COSE, threshold proofs, or ZK proof integrations. |
| `domains/` | Business and operational semantics such as banking or AI-agent profiles. |
| `services/` | Optional applications such as CLI, witness, prover, query, or node services. |

The root `bcx` crate and foundation crates must remain independent of these
future families unless a release explicitly admits a new dependency boundary.
Package versions are tracked in
[Crate Version Matrix](docs/CRATE_VERSION_MATRIX.md) so future releases can
publish only the crates that changed instead of republishing the whole
ecosystem.

## Verification

Run the local gate:

```bash
scripts/checks.sh
```

The gate checks formatting, tests, no-default-feature builds, all-feature
builds, clippy, docs, package metadata, file-size policy, release metadata, and
installed target triples.

## Security Position

BCX must distinguish:

- what was observed,
- what was declared,
- what was cryptographically verified,
- what policy enforced,
- what another participant acknowledged,
- what remains unknown.

The protocol must not claim to prove a participant's internal motive. A signed
purpose is an attributable declaration, not proof that the declared purpose was
truthful.

Read [Security Policy](SECURITY.md), [Threat Model](docs/threat-model.md), and
[Implementation Plan](docs/IMPLEMENTATION_PLAN.md) before using BCX in any
security-sensitive system. The broader architecture is documented in
[BCX Protocol Family](docs/protocol-family.md), and the preserved idea draft is
kept as [Original Idea](docs/original-idea.md).
