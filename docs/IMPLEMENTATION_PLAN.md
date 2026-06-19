# BCX Implementation Plan

Status: planning document

Project and crate name: `bcx`

Expanded name: Bifrost Casual Exchange

1.0 target: a production-ready Rust protocol crate for verifiable causal
network accountability, with Fluxheim as the first serious implementation.

## Core Position

BCX is a proof-carrying causal protocol for consequential operations. It is not
an HTTP replacement and not an observability trace format. HTTP, queues, native
QUIC streams, files, and air-gapped bundles may carry BCX objects, but the
security decision is based on canonical signed BCX objects.

The fundamental sequence is:

```text
Invocation -> AdmissionDecision -> ExecutedEffect -> VerifiableReceipt
```

BCX must answer:

- why an operation happened,
- who or what caused it,
- what authority permitted it,
- what policy was applied,
- what effect was observed,
- what downstream operations followed,
- what cannot be proven.

## Non-Negotiable Engineering Rules

- Root published crate name is `bcx`.
- Rust stable `1.96.0` is the preferred toolchain.
- MSRV is Rust `1.90.0`.
- Compatibility from Rust `1.90.0` through `1.96.0` must remain explicit in
  the README evidence table.
- Edition 2024 and workspace resolver `3`.
- Core crates are `no_std` by default.
- The root crate stays no-std by default.
- Third-party crates are admitted only after review, documentation, and tests.
- Transport, crypto provider, storage, CLI, and HTTP/3 bindings must be
  separate crates so the main crate does not lose no-std.
- Normal `.rs` files must stay under 500 lines.
- Security and testability are part of every release, not cleanup work.
- Unsafe Rust is forbidden until a documented boundary crate is approved.

## Workspace Shape

- `bcx`: published facade crate.
- `bcx-core`: IDs, digests, nonces, sequence numbers, validation errors.
- `bcx-model`: cause capsules, actions, admissions, effects, truth statuses,
  and assurance levels.
- `bcx-crypto`: crypto-agile envelopes, algorithm identifiers, verifier traits,
  and future provider boundaries.
- `bcx-policy`: profiles, proof levels, disclosure levels, and policy result
  skeletons.
- `bcx-wire`: versioning, size limits, message headers, and future canonical
  encoding boundaries.

Future crates:

- `bcx-codec`: deterministic CBOR or admitted canonical binary encoding.
- `bcx-capability`: proof-of-possession capabilities and attenuation rules.
- `bcx-receipt`: admission, effect, witness, and transparency receipts.
- `bcx-why`: bounded explanation queries and proof bundle verification.
- `bcx-store`: append-only event storage traits and default stores.
- `bcx-http`: HTTP/1.1, HTTP/2, and HTTP/3 compatibility binding.
- `bcx-quic`: native `bcx/1` protocol over QUIC.
- `bcx-cli`: `why`, `verify`, `bundle`, and inspection commands.

## Phase 1: Protocol Kernel

Build minimal no-std canonical types:

- event IDs,
- digest commitments,
- nonces,
- issuer sequence numbers,
- cause kinds,
- relationship kinds,
- operation actions,
- truth statuses,
- assurance levels,
- admission and effect result enums.

The first correctness question is: can BCX represent causal claims without
confusing observation, declaration, verification, enforcement, and receipt?

## Phase 2: Canonical Encoding

Introduce canonical encoding after the model stabilizes.

Requirements:

- deterministic bytes,
- bounded lengths,
- schema versioning,
- unknown-field policy,
- no native Rust memory layout dependency,
- no JSON security boundary,
- malformed input rejection before expensive verification.

## Phase 3: Crypto And Capability Boundary

Build crypto-agile signatures and proof-of-possession capabilities.

Requirements:

- algorithm identifiers,
- key identifiers,
- admitted algorithm policy,
- bounded signature set size,
- audience binding,
- expiry,
- issuer sequence,
- nonce,
- replay cache hooks,
- capability attenuation.

BCX must not invent cryptographic primitives. It owns the envelope semantics
and provider traits; provider crates own actual algorithm implementations.

## Phase 4: Admission And Effect Receipts

Add signed receipts:

- admission allow, deny, narrow, require approval, quarantine,
- policy digest,
- configuration digest,
- identity and capability status,
- observed network effect,
- application effect hooks,
- child invocation references,
- obligations.

A gateway may receipt network effects. Application, database, storage, or worker
integrations are required before BCX can claim deeper effects.

## Phase 5: Local Fluxheim Integration

Fluxheim should first use BCX locally:

- ingress event,
- authentication result,
- route decision,
- rate-limit/security decision,
- upstream selection,
- upstream dispatch,
- response receipt,
- cache decision,
- response completion.

This phase requires no browser support and no federation.

## Phase 6: Federated WHY

Add authenticated bounded explanation queries.

Requirements:

- audit capabilities,
- query purpose,
- maximum depth,
- maximum nodes,
- maximum response bytes,
- cycle detection,
- redacted edges,
- missing-event disclosure,
- contradictory claim handling,
- cross-domain trust policy.

WHY must never be an unauthenticated recursive graph crawl.

## Phase 7: Native BCX Over QUIC

Add `bcx/1` as a native QUIC application protocol after the semantics are
stable.

The native protocol may define frames such as:

- `HELLO`,
- `TRUST_CONTEXT`,
- `INVOKE`,
- `ADMISSION`,
- `DATA`,
- `EFFECT`,
- `WHY`,
- `EXPLANATION`,
- `REVOKE`,
- `CANCEL`,
- `CLOSE`.

HTTP/3 remains a compatibility binding, not the source of proof.

## Phase 8: High-Assurance Profile

Define the Sovereign profile:

- mutual peer authentication,
- no unsigned consequential invocations,
- no bearer-only authorization,
- no state-changing 0-RTT,
- no silent downgrade,
- bounded graph traversal,
- key rotation and revocation,
- append-only receipt storage,
- optional remote attestation,
- optional traffic padding.

## 1.0 Definition

BCX 1.0 is ready when:

- the root `bcx` crate is stable enough for external users,
- the canonical object model is documented and tested,
- a Fluxheim integration can produce and verify local WHY explanations,
- two Fluxheim peers can exchange signed causal events,
- tampering, replay, false-parent, and downgrade tests fail closed,
- release notes, threat model, security controls, and supply-chain docs are
  complete for the exact release.
