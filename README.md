# bcx

`bcx` is the Rust crate for **Bifrost Casual Exchange**: a no-std-first
protocol foundation for verifiable causal accountability across network
operations.

BCX is not an HTTP replacement. It defines signed operation facts, causal
parentage, admission decisions, effect receipts, bounded WHY queries, and
selective-disclosure explanation bundles. HTTP/3 can carry BCX objects for
compatibility, while high-assurance deployments can later use a native BCX
application protocol over QUIC.

## Current Status

`bcx` is at repository foundation stage. The crate is not production-ready.

Implemented now:

- `no_std` default build.
- zero third-party runtime dependencies.
- root publishable crate named `bcx`.
- focused no-std subcrates for core IDs, model types, crypto envelope metadata,
  policy profiles, and wire limits.
- local check script, modularity guard, release metadata guard, and GitHub CI.
- implementation, version, threat-model, toolchain, and modularity docs.

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
| High-security position | trust canonical signed BCX objects, not an HTTP wrapper |

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
| `bcx-model` | Cause, action, decision, effect, truth, and assurance model types. |
| `bcx-crypto` | Crypto-agile signature envelope metadata and verifier traits. |
| `bcx-policy` | Protocol profiles, proof levels, and disclosure levels. |
| `bcx-wire` | Wire version and bounded-message limit primitives. |

Future transport crates should stay separate from the core crate so `bcx`
remains no-std by default.

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
security-sensitive system.
