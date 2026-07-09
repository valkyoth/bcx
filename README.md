<p align="center">
  <b>no_std-first causal exchange primitives for Rust.</b><br>
  Signed causal meaning, proof composition, native bindings, checkpoints, and bounded explanations.
</p>

<div align="center">
  <a href="https://docs.rs/bcx">Docs.rs</a>
  |
  <a href="docs/threat-model.md">Threat Model</a>
  |
  <a href="docs/security-controls.md">Security Controls</a>
  |
  <a href="SECURITY.md">Security</a>
</div>

<br>

<p align="center">
  <a href="https://github.com/valkyoth/bcx">
    <img src="https://raw.githubusercontent.com/valkyoth/bcx/main/.github/images/bcx.webp" alt="BCX Rust crate overview">
  </a>
</p>

# bcx

`bcx` is the Rust crate for **Bifröst Causal Exchange**.

BCX is a `no_std`-first protocol foundation for signed causal meaning, proof
composition, native bindings, checkpoints, and bounded explanations across
multiple underlying systems.

BCX is not an HTTP replacement, an Ethereum protocol, or a Cardano protocol. It
is a semantic overlay layer. BCX defines what an operation means, why it was
authorized, what it caused, who attested to it, what evidence exists, and what
remains unknown. HTTP, QUIC, Ethereum, Cardano, Fluxheim, Skrifheim, offline
bundles, and future systems can carry, observe, store, or settle BCX objects
through explicit profiles and integrations.

## Current Status

`bcx` is at foundation stage. The crate is not production-ready.

Implemented now:

- `no_std` default build.
- root publishable crate named `bcx`.
- focused `no_std` subcrates for core IDs, model types, crypto envelope
  metadata, policy profiles, and wire limits.
- constant-shape digest, nonce, and variable-length identifier comparison
  where the current primitives need it.
- redacted `Debug` for nonce, digest, signature, and identifier surfaces that
  should not dump raw bytes casually.
- bounded wire limits before expensive parsing or verification.
- crypto-agile signature envelope metadata and verifier traits.
- hybrid Ed25519 plus ML-DSA-65 metadata with both-component verification
  requirements at the BCX trait boundary.
- first statement body vocabulary: intent, admission, effect, delegation,
  revocation, checkpoint, and contradiction.
- explicit causal edge sets with per-parent relationship kinds, missing-parent
  markers, duplicate-parent rejection, and cycle-prevention hooks.
- crate version matrix for independent subcrate publication.
- local check script, modularity guard, release metadata guard, and GitHub CI.
- implementation, version, protocol-family, threat-model, toolchain,
  modularity, unsafe-policy, supply-chain, and security-control docs.
- repository families for future profiles, integrations, proofs, domain
  profiles, and services.

Not implemented yet:

- canonical encoding.
- statement envelopes and canonical statement encoding.
- real cryptographic providers.
- capability verification.
- receipts or WHY bundles.
- transport, blockchain, storage, or service integrations.

## Trust Dashboard

| Area | Status |
| --- | --- |
| License | `EUPL-1.2` |
| MSRV | Rust `1.90.0` |
| Preferred toolchain | Rust `1.96.1` |
| Default target | `no_std` |
| Runtime dependencies | small, explicit, and audited |
| Unsafe policy | `unsafe_code = "forbid"` |
| Main crate | `bcx` |
| Foundation subcrates | published only for `bcx`, not standalone protocol products |
| First serious target | `1.0.0` production-ready protocol crate |
| High-security position | trust canonical signed BCX objects, not one carrier or chain |

Read [Threat Model](docs/threat-model.md),
[Security Controls](docs/security-controls.md), and
[Unsafe Policy](docs/unsafe-policy.md) before using BCX in any
security-sensitive system.

## Rust Version Support

The minimum supported Rust version is Rust `1.90.0`. New deployments should
prefer the latest stable Rust validated by the project.

Compatibility evidence:

| Rust | Local Evidence |
| --- | --- |
| `1.90.0` | full release gate compatibility check |
| `1.91.0` | `cargo check --workspace --all-features` |
| `1.92.0` | `cargo check --workspace --all-features` |
| `1.93.0` | `cargo check --workspace --all-features` |
| `1.94.0` | `cargo check --workspace --all-features` |
| `1.95.0` | `cargo check --workspace --all-features` |
| `1.96.0` | `cargo check --workspace --all-features` |
| `1.96.1` | current pinned toolchain |

## Install

```toml
[dependencies]
bcx = "0.5.0"
```

Default builds are `no_std`.

For heap-enabled future APIs and dependent crate `alloc` surfaces:

```toml
[dependencies]
bcx = { version = "0.5.0", features = ["alloc"] }
```

For `std` applications:

```toml
[dependencies]
bcx = { version = "0.5.0", features = ["std"] }
```

## Features

| Feature | Default | Purpose |
| --- | --- | --- |
| `alloc` | no | Enables `alloc` surfaces across the facade and foundation crates when those APIs exist. |
| `std` | no | Enables `alloc` and reserves the feature line for future standard-library integrations. |

Default builds remain `no_std`.

## Workspace Shape

The root `bcx` crate is the stable user entry point. Foundation subcrates exist
to keep files small, boundaries reviewable, and future optional integrations
outside the core dependency set.

| Crate | Purpose |
| --- | --- |
| `bcx` | Published facade crate and stable user entry point. |
| `bcx-core` | Identifiers, nonces, replay sequence primitives, validation errors. |
| `bcx-model` | Causal edges, admissions, effects, truth, and assurance model types. |
| `bcx-crypto` | Crypto-agile signature envelope metadata and verifier traits. |
| `bcx-policy` | Protocol profiles, proof levels, and disclosure levels. |
| `bcx-wire` | Wire version and bounded-message limit primitives. |

Subcrates are documented so crate pages are readable, but they belong to the
main `bcx` crate family and are not intended as independent protocol products.

Future crates are organized by family:

| Family | Purpose |
| --- | --- |
| `profiles/` | Normative mappings such as HTTP, QUIC, Ethereum, Cardano, offline, SCITT, or future Bitcoin/XRP profiles. |
| `integrations/` | Concrete adapters such as Hyper, Axum, Alloy, Pallas, Fluxheim, or Skrifheim. |
| `proofs/` | Concrete proof suites such as COSE, threshold proofs, or ZK proof integrations. |
| `domains/` | Business and operational semantics such as banking or AI-agent profiles. |
| `services/` | Optional applications such as CLI, witness, prover, query, or node services. |

Package versions are tracked in
[Crate Version Matrix](docs/CRATE_VERSION_MATRIX.md) so future releases can
publish only the crates that changed instead of republishing the whole
ecosystem.

## Core Identifier Example

```rust
use bcx::prelude::{Digest, StatementId, SubjectId};

let statement = StatementId::new(&[7; Digest::LEN]).unwrap();
let subject = SubjectId::new(b"subject:invoice:123").unwrap();

assert_eq!(statement.len(), Digest::LEN);
assert_eq!(subject.as_bytes(), b"subject:invoice:123");
```

## Wire Limit Example

```rust
use bcx::prelude::{ProtocolVersion, WireHeader, WireLimits};

let limits = WireLimits::new(64 * 1024, 16, 5, 100).unwrap();
let header = WireHeader::new(ProtocolVersion::CURRENT, 4096, limits).unwrap();

assert_eq!(header.version(), ProtocolVersion::CURRENT);
assert_eq!(header.payload_len(), 4096);
```

## Verification

Run the local gate:

```bash
scripts/checks.sh
```

The gate checks formatting, tests, no-default-feature builds, all-feature
builds, clippy, docs, package metadata, file-size policy, release metadata,
dependency policy, RustSec advisories, and installed target triples.

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
