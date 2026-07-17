# BCX Implementation Plan

Status: active planning document

Project and crate name: `bcx`

Expanded protocol name: Bifrost Causal Exchange

1.0 target: a production-ready no-std Rust protocol foundation for signed
causal meaning, proof composition, native bindings, checkpoints, and bounded
explanations across multiple underlying systems.

## Core Position

BCX is a semantic overlay protocol. It does not replace HTTP, QUIC, Ethereum,
Cardano, Bitcoin, XRP, Kafka, object stores, institutional ledgers, or future
networks. BCX defines what an operation means, why it was authorized, what it
caused, who attested to it, what evidence exists, and what remains unknown.

Underlying systems carry, execute, observe, store, or settle BCX objects. They
do not change the invariant BCX statement model.

```text
Application, institution, agent, or service
    -> BCX semantic layer
    -> BCX profile or integration
    -> HTTP, QUIC, Ethereum, Cardano, Bitcoin, XRP, storage, or offline bundle
```

The central rule is:

```text
BCX defines causal meaning and proof structure.
Profiles bind that meaning to native systems.
Integrations make those bindings usable in real stacks.
```

## Non-Negotiable Engineering Rules

- Root published crate name is `bcx`.
- Rust stable `1.96.1` is the preferred toolchain.
- MSRV is Rust `1.90.0`.
- Compatibility from Rust `1.90.0` through `1.96.1` must remain explicit in
  the README evidence table.
- Edition 2024 and workspace resolver `3`.
- Core crates are `no_std` by default.
- The root crate stays no-std by default.
- The core never depends on HTTP, blockchain, async runtime, database, CLI, or
  provider-specific crates.
- Profiles depend on the core; the core never depends on profiles.
- Integrations depend on profiles; profiles do not depend on one concrete
  framework.
- Third-party crates are admitted only after review, documentation, and tests.
- Normal `.rs` files must stay under 500 lines.
- Security and testability are part of every release.
- Unsafe Rust is forbidden until a documented boundary crate is approved.

## Non-Negotiable Protocol Invariants

- Statement identity is sealed and derived from canonical typed statement
  bytes.
- Commitments are domain-separated by object class, version, codec, and suite.
- Carriers never rewrite statements; they bind native evidence to unchanged
  statement identities.
- Local availability, missing-parent state, transport metadata, attestations,
  and native bindings are not part of canonical statement identity.
- Graph admission is atomic: cycle checks and insertion happen as one operation.
- Wire parsing is bounded and rejects invalid input before allocation, hashing,
  key lookup, or cryptographic verification.
- Semantic validity is derived from policy, revocation, conflict, checkpoint,
  and evidence roots; immutable historical statements are not mutated into
  invalid history.
- Key resolution, signing, verification, replay, capability checks, and
  settlement finality use explicit policy and trust snapshots.
- WHY bundles disclose what is proven, missing, redacted, withheld, stale,
  contradicted, or truncated.

## Published Foundation Crates

The crates already published for `0.1.0` still fit the new model and should be
evolved rather than renamed casually.

| Crate | Foundation role |
| --- | --- |
| `bcx` | Published facade crate and stable user entry point. |
| `bcx-core` | Identifiers, digests, nonces, sequence numbers, and common validation errors. |
| `bcx-model` | Statement vocabulary, causal edges, evidence facets, effects, admissions, and contradictions. |
| `bcx-crypto` | Crypto-agile attestation envelope metadata and verifier traits. |
| `bcx-policy` | Profiles, proof levels, disclosure levels, replay policy, and settlement policy vocabulary. |
| `bcx-wire` | Protocol versions, wire limits, bounded message entry rules, and future canonical framing boundaries. |

## Future Workspace Families

Future crates should be introduced only when the preceding release has a clean
stop, tests, release notes, and pentest handoff.

### Core Crates

- `bcx-codec`: canonical deterministic encoding, initially CBOR if admitted.
- `bcx-state`: optional deterministic state transition engine for rollups,
  consortium ledgers, synchronized causal state, or replayable simulations.
- `bcx-explain`: WHY queries, causal traversal, contradictions, unknowns,
  selective disclosure, and proof-bundle verification.
- `bcx-registry`: type, profile, algorithm, proof-suite, and extension
  registries.
- `bcx-conformance`: interoperability vectors and cross-platform fixtures.
- `bcx-testkit`: deterministic test builders and adversarial fixtures.

### Profiles

Profiles define normative mappings. They should be small and dependency-light.

- `bcx-http`: HTTP binding for attached and encapsulated BCX exchanges.
- `bcx-quic`: native `bcx/1` exchange over QUIC.
- `bcx-ethereum`: Ethereum binding, observation, action gating, and settlement
  profile.
- `bcx-cardano`: Cardano EUTXO binding and settlement profile.
- `bcx-offline`: air-gapped and file-bundle profile.
- `bcx-scitt`: transparency-service profile.
- `bcx-opentelemetry`: observability correlation profile.
- future `bcx-bitcoin`, `bcx-xrp`, or other settlement/binding profiles when
  a concrete security contract is written.

### Integrations

Integrations connect profiles to real libraries, frameworks, services, or
products.

- `bcx-http-hyper`
- `bcx-http-axum`
- `bcx-http-h3`
- `bcx-quic-quinn`
- `bcx-ethereum-alloy`
- `bcx-ethereum-contracts`
- `bcx-cardano-pallas`
- `bcx-skrifheim`
- `bcx-fluxheim`

### Proof Crates

- `bcx-proof-cose`
- `bcx-proof-threshold`
- `bcx-proof-sp1`
- `bcx-proof-risc0`

### Domain Profiles

Domain profiles define business or operational semantics. They must not be
confused with transport or settlement bindings.

- `bcx-profile-banking`
- `bcx-profile-ai-agent`
- `bcx-profile-supply-chain`
- `bcx-profile-healthcare`

### Services

Services are optional applications around the protocol.

- `bcx-cli`
- `bcx-node`
- `bcx-witness`
- `bcx-prover`
- `bcx-query`

## Core Object Model

BCX separates logical statements, attestations, and native bindings.

```text
Statement
    What is being claimed?

Attestation
    Who signed, witnessed, or proved that claim?

Native binding
    Which HTTP request, Ethereum transaction, Cardano UTXO, offline bundle, or
    other native event does the statement bind to?
```

Statement identifiers are derived from canonical statement bytes, not from a
specific signature. Multiple parties can attest to the same statement without
changing the statement identity.

BCX/1 starts with a small statement body set:

- `Intent`
- `Admission`
- `Effect`
- `Delegation`
- `Revocation`
- `Checkpoint`
- `Contradiction`

## Truth And Assurance

BCX must never collapse a signed claim into proven reality. A signature proves
authorship and integrity of a statement. It does not prove an internal motive or
the truth of an offchain business claim.

Truth and assurance are modeled as evidence facets plus checkpoint-relative
derived validity, not as one mutually exclusive status enum. Evidence facets
include:

- declared,
- observed,
- cryptographically verified,
- policy enforced,
- counterparty acknowledged,
- independently witnessed,
- settlement finalized,
- contradicted,
- unknown.

`Unknown` means absence of evidence in the evaluated context. Contradiction and
revocation can coexist with authentic historical evidence; they affect derived
usability and assurance rather than rewriting history.

Adapters may attest only to effects they can actually observe. For example, an
HTTP gateway can attest that it received a response; it cannot claim a database
commit without database or application evidence.

## Profile Security Contract

Every BCX profile must document:

- exactly what native fields are committed,
- how replay is prevented,
- what finality means,
- what intermediaries may alter,
- what information becomes public,
- what the adapter can truthfully observe,
- how downgrades are prevented,
- how unknown extensions are handled,
- what happens when native evidence disappears.

No profile may silently downgrade to unsigned or unaudited operation.

## First Implementation Strategy

Do not start with Ethereum rollup, Cardano validator, Bitcoin anchoring, or a
large Fluxheim integration. Build the invariant core slowly first.

The first serious sequence is:

1. make the protocol-family architecture explicit in `0.2.0`,
2. harden statement, attestation, binding, and checkpoint vocabulary,
3. add deterministic canonical encoding and test vectors,
4. add bounded verification and replay hooks,
5. add local/offline WHY proof bundles,
6. add HTTP as the first carrier profile,
7. add Fluxheim as the first live integration,
8. add one settlement profile,
9. add a second settlement profile to prove BCX is not chain-specific.

## 1.0 Definition

BCX 1.0 is ready when:

- the root `bcx` crate is stable enough for external users,
- the canonical object model is documented and tested,
- statement IDs are stable across profiles,
- the standard proof suite and canonical codec are implemented,
- local/offline WHY bundles verify correctly,
- HTTP and Fluxheim can carry and verify signed causal objects,
- at least two settlement or witness profiles can record the same checkpoint,
- tampering, replay, false-parent, overclaiming, and downgrade tests fail
  closed,
- release notes, threat model, security controls, and supply-chain docs are
  complete for the exact release.
