# BCX Version Plan

Status: planning document

Tags use:

```text
v0.N.0      milestone release
v0.N.P      patch/fix release
v1.0.0      first serious production-ready BCX crate
```

## Release Principles

Every release must have:

- definition of done,
- local verification command,
- security review notes,
- known limitations,
- release notes,
- no hidden dependency on one developer machine.

No production claim is allowed before `1.0.0`.

## Pentest Before Tags

When implementation criteria for a version are done, stop before tagging and
ask for pentest of that exact commit.

Flow:

1. Local gates pass: `scripts/checks.sh`, `cargo deny check`, and `cargo audit`.
2. Maintainer writes temporary findings to root `PENTEST.md`.
3. Findings are fixed or explicitly deferred outside the release scope.
4. `PENTEST.md` is removed.
5. Local gates run again.
6. Permanent report is written under `security/pentest/<tag>.md` only for a tag
   candidate with `Status: PASS`.
7. Tags are created only when explicitly requested.

## v0.1.0 - Repository Foundation

Goal: initialize the serious Rust workspace and policy baseline.

Deliverables:

- root `bcx` crate,
- focused no-std subcrates,
- EUPL-1.2 license,
- CI and local check script,
- dependency policy,
- security policy,
- implementation, version, threat-model, toolchain, and modularity docs.

Verification:

- `scripts/checks.sh`
- `cargo test --workspace`

## v0.2.0 - Canonical Type Kernel

Goal: make BCX causal primitives complete enough to model local Fluxheim WHY
events.

Deliverables:

- invocation skeleton,
- parent edge model,
- cause capsule validation,
- bounded lists,
- explicit truth-status handling,
- negative tests for malformed and ambiguous claims.

## v0.3.0 - Deterministic Encoding Boundary

Goal: define canonical bytes without relying on Rust layout or JSON.

Deliverables:

- admitted encoding format decision,
- schema/version model,
- strict decoder limits,
- malformed-input tests,
- cross-platform byte-equivalence tests.

## v0.4.0 - Crypto Envelope And Verification Traits

Goal: make signed BCX objects verifiable through provider traits.

Deliverables:

- signature-set bounds,
- algorithm allow-list policy,
- key identifier bounds,
- audience binding,
- signature verification trait,
- negative tests for empty, oversized, unknown, and mismatched signatures.

## v0.5.0 - Capability And Replay Model

Goal: make authority and replay explicit.

Deliverables:

- proof-of-possession capability metadata,
- attenuation model,
- issuer sequence model,
- nonce and expiry checks,
- replay-cache trait,
- tests for stolen bytes, replay, expired, and wrong-audience operations.

## v0.6.0 - Admission Receipts

Goal: record why an operation was allowed, narrowed, denied, or quarantined.

Deliverables:

- admission receipt type,
- policy digest,
- configuration digest,
- identity and capability status,
- obligations,
- tests for required fields and policy mismatch.

## v0.7.0 - Effect Receipts

Goal: record what actually happened.

Deliverables:

- effect receipt type,
- response and state commitments,
- child invocation references,
- effect assurance levels,
- tests that gateway receipts cannot claim application-only effects.

## v0.8.0 - Explanation Bundles

Goal: answer bounded WHY queries locally.

Deliverables:

- explanation query type,
- explanation bundle type,
- redacted and missing edges,
- contradiction reporting,
- query budget enforcement,
- tests for depth, node, and byte limits.

## v0.9.0 - Append-Only Store Traits

Goal: support durable local event chains without selecting one database.

Deliverables:

- append-only event store trait,
- Merkle-root or hash-chain metadata,
- retention markers,
- witness commitment hooks,
- tests for tamper and missing-parent detection.

## v0.10.0 - Fluxheim Local Integration

Goal: make Fluxheim produce local BCX events at its security boundary.

Deliverables:

- local ingress and route events,
- auth and policy decisions,
- upstream dispatch and response receipts,
- `fluxheim why` prototype,
- fixture tests.

## v0.11.0 - HTTP Compatibility Binding

Goal: carry canonical BCX objects over HTTP without trusting the HTTP wrapper.

Deliverables:

- `bcx-http` crate,
- `.well-known/bcx` endpoint shape,
- request and response mapping,
- no silent downgrade rule,
- tampered-wrapper tests.

## v0.12.0 - Federated Peer Model

Goal: let two BCX-aware peers exchange signed causal facts.

Deliverables:

- peer identity model,
- trust-domain metadata,
- cross-acknowledged edges,
- key revocation metadata,
- tests for untrusted peer and false-parent claims.

## v0.13.0 - Federated WHY

Goal: query another peer safely.

Deliverables:

- audit capability checks,
- selective disclosure,
- query purpose,
- depth and node budgets,
- cycle detection,
- redacted explanation tests.

## v0.14.0 - Native QUIC Mapping

Goal: add `bcx/1` over QUIC as the high-security transport.

Deliverables:

- ALPN profile,
- frame state machine,
- no state-changing 0-RTT rule,
- stream budget limits,
- malformed frame tests.

## v0.15.0 - Sovereign Profile

Goal: define strict high-assurance deployment behavior.

Deliverables:

- mandatory signatures,
- mandatory mutual peer auth,
- no bearer-only capabilities,
- no downgrade,
- receipt storage requirements,
- profile conformance tests.

## v0.16.0 - Provider Qualification Hooks

Goal: prepare for admitted crypto, storage, and transport providers.

Deliverables:

- provider capability traits,
- provider self-description,
- audit metadata,
- dependency review templates,
- test vectors.

## v1.0.0 - Production Protocol Crate

Goal: ship the first serious production-ready BCX crate.

Deliverables:

- stable root API,
- complete canonical model,
- local and federated WHY verification,
- Fluxheim reference integration,
- HTTP compatibility binding,
- native QUIC binding,
- full threat model,
- release notes,
- SBOM,
- pentest pass for exact commit.
