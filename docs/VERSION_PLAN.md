# BCX Version Plan

Status: active planning document

BCX is a security-sensitive protocol family. The roadmap intentionally uses
small tags. Add more patch or milestone tags whenever a release would otherwise
mix too many semantics, dependencies, or security surfaces.

Tags use:

```text
v0.N.0      small milestone release
v0.N.P      patch/fix release for milestone N
v1.0.0      first production-ready BCX protocol foundation
```

Protocol versions and crate versions are separate. For example, `bcx-http`
crate `0.12.3` may still implement `BCX-HTTP/1`.

Crate package versions are tracked in
[`CRATE_VERSION_MATRIX.md`](CRATE_VERSION_MATRIX.md). The matrix exists so BCX
can grow into many crates without republishing every crate for every milestone.
The release gate checks that the matrix matches Cargo metadata, that local path
dependency version requirements match the referenced local packages, and that
crate source or manifest changes since the latest tag are accompanied by a
package-version bump.

## Release Principles

Every release must have:

- a clear definition of done,
- local verification commands,
- release notes,
- security review notes,
- known limitations,
- a completed pentest report for the release,
- no hidden dependency on one developer machine.

Every release should prefer:

- small protocol increments,
- no-std core tests before transport integration,
- deterministic behavior before provider-specific behavior,
- bounded parsing and graph traversal before federation,
- explicit capability-aware APIs even when enforcement is still simple.

No roadmap, release note, pentest response, or limitation may defer work with
phrases such as "future work", "later", or "deferred" unless it names the exact
version or version range where the work is scheduled.

No production claim is allowed before `1.0.0`.

## Crate Package Versioning

Milestone tags such as `v0.3.0` describe the repository release. Individual
crate package versions describe publishable crate artifacts.

Rules:

- publish only crates whose package contents changed or whose dependency pins
  need to move,
- do not bump a leaf crate just because another unrelated crate changed,
- bump the root `bcx` facade when its exports, embedded docs, or local
  dependency pins change,
- when a local crate version changes, update and version-bump every dependent
  crate that needs the new local dependency pin, repeating transitively through
  the workspace dependency graph,
- update `docs/CRATE_VERSION_MATRIX.md` whenever any package version changes,
- keep local path dependency `version = "..."`
  requirements synchronized with the referenced local package version,
- run `scripts/validate-crate-version-matrix.py` before release handoff.

This avoids unnecessary crates.io uploads while keeping compatibility metadata
reviewable.

## Pentest Before Tags

Every version must pass security review and pentest before it is tagged. This
applies to tiny `v0.N.P` patch tags as well as milestone tags.

A version is not tag-ready until:

- `scripts/checks.sh` passes,
- the version-specific release gate passes,
- the release gate has checked latest pinned security and GitHub tooling,
- `cargo deny check` passes,
- `cargo audit` passes,
- release notes exist at `release-notes/RELEASE_NOTES_<version>.md`,
- a pentest report exists at `security/pentest/<tag>.md`,
- the pentest report records `Tag: <tag>`,
- the pentest report records `Commit: <40-character git commit>`,
- the pentest report has `Status: PASS`,
- the pentest report has non-blank `Tester:` and `Scope:` fields,
- the pentest report has a `Date: YYYY-MM-DD` field,
- the report commit matches current `HEAD` or `HEAD^` when the current commit
  only adds the permanent pentest report,
- root `PENTEST.md` does not exist,
- the tag does not already exist locally,
- `scripts/validate-release-readiness.sh <tag>` passes.

When a version's implementation criteria are done, stop before tagging and say:

```text
vX.Y.Z implementation stop reached. Run pentest for this release.
```

### Pentest Handoff Flow

1. The implementation owner finishes the criteria and reports the release is
   ready for review.
2. The maintainer runs pentest and writes temporary findings to root
   `PENTEST.md`.
3. The implementation owner reviews root `PENTEST.md`, fixes release-scope
   findings, updates release notes or tracking docs, and records the scratch
   report into `security/pentest/<tag>.md`:

```bash
scripts/record_pentest_report.py \
  --version X.Y.Z \
  --tester "<tester>" \
  --scope "<scope>" \
  --date YYYY-MM-DD
```

4. Root `PENTEST.md` is deleted after the permanent digest report has been
   reviewed.
5. The maintainer decides whether another pentest pass is needed. If yes,
   repeat from step 2. If no, commit the implementation and permanent pentest
   report, then wait for GitHub CI and CodeQL default setup.
6. When the maintainer reports that GitHub is green, the tag-only finalizer is:

```bash
scripts/finalize_release.py \
  --version X.Y.Z
```

The finalizer refuses to run while root `PENTEST.md` exists, refuses dirty
tracked worktrees, runs the version release gate, creates the annotated tag,
and optionally pushes `main` or the tag when explicitly requested.

The permanent report follows the Aesynx-style release evidence model. It names
the exact implementation commit that was reviewed. The release gate accepts a
report for current `HEAD`, or for `HEAD^` when current `HEAD` is only the
permanent pentest report commit. This keeps traceability without requiring a new
pentest for the evidence commit itself.

Never commit root `PENTEST.md`; it is scratch input and is ignored by git.

## Completeness Review Register

Every planning, implementation, and pentest pass must check this register for
implied work that is not assigned to a release. If a row affects the `1.0.0`
foundation scope, it must have a concrete pre-1.0 milestone before work
continues past the relevant dependency point.

| Gap | Resolution |
| --- | --- |
| The tokenless semantic-overlay position could be read as philosophy rather than an enforceable design contract. | Added `v0.73.0 - Tokenless Operation Contract`. |
| `bcx-state` was named as a crate but did not have implementation milestones. | Added `v0.74.0` through `v0.78.0` for deterministic state contracts, resource metering, crate skeleton, local transitions, program identity, and effect proof inputs. |
| Smart-contract-like local execution was implied by state and proof crates but not versioned. | Added `v0.85.0 - Local Contract Workflow Fixture` and `v0.86.0 - Tokenless Offline Institution Demo`. |
| COSE was listed as the likely proof foundation but not assigned to a dependency-ordered crate milestone. | Added `v0.36.0 - COSE Proof Envelope Crate` after key, signing, verification, and hybrid policy prerequisites. |
| Threshold witness proofs were listed but not assigned to a concrete release. | Added `v0.79.0 - Threshold Proof Crate`. |
| ZK proof integrations were named but did not have provider-boundary and implementation milestones. | Added `v0.80.0` through `v0.82.0` for the ZK proof contract, SP1 provider crate, and RISC Zero provider crate. |
| `bcx-witness` was named as a service but did not have a profile contract or service milestone. | Added `v0.83.0 - Witness Service Contract` and `v0.84.0 - Witness Service Skeleton`. |
| Blockchain-independent double-spend/order prevention needed an explicit non-chain path. | Added semantic validity roots in `v0.40.0`, threshold proofs in `v0.79.0`, witness service releases in `v0.83.0` and `v0.84.0`, and the offline institution demo in `v0.86.0`. |
| The `1.0.0` scope did not explicitly require local deterministic execution and tokenless proof workflows. | Updated the `1.0.0` required scope with deterministic state, threshold witness, local contract workflow, and tokenless institution evidence. |
| ML-DSA-65 signature length is wrong in the current crypto constants. | Added `v0.5.1 - ML-DSA-65 Constant Correction` as the next patch release before more protocol surface is added. |
| Statement IDs and detached verified bytes can be supplied by callers instead of derived from canonical typed statements. | Folded sealed commitment work into `v0.15.0 - Canonical Statement Encoding And Sealed Identity` and verification enforcement into `v0.38.0 - Statement Verification`. |
| Phase 12 assurance work was ordered after profiles, demos, and providers that depend on it. | Reordered the roadmap so commitments, codec, limits, graph admission, replay, capabilities, keys, signing, verification, semantic validity, receipts, privacy, specs, parser fuzzing, and crypto conformance all precede the first carrier profile. |
| Multi-node cycle prevention is optional, non-atomic, and missing-parent resolution can introduce cycles after acceptance. | Moved graph safety to `v0.19.0 - Graph Store Contract`, `v0.20.0 - Atomic Graph Store Skeleton`, and `v0.21.0 - Missing Parent Reconciliation`. |
| `CauseCapsule` and `CausalEdgeSet` remain parallel causal-parent APIs with different graph hooks. | Added `v0.19.0` normalization rules so compact capsules pass through the graph-store edge model or are explicitly deprecated. |
| `ParentStatus::Missing` can be caller-selected even though availability is verifier-local state. | Added canonical-edge separation in `v0.15.0` and derived local availability plus reconciliation behavior in `v0.21.0`. |
| Relationship kinds do not yet enforce semantic cardinality, dependency roles, or target-kind constraints. | Added `v0.22.0 - Relationship Semantics And Edge Roles`. |
| Duplicate-parent checks are quadratic and canonical parent ordering is not yet defined. | Added canonical parent ordering in `v0.15.0` and adjacent-duplicate graph checks in `v0.19.0`. |
| `WireLimits` is a limits container, not an early parser or aggregate resource budget. | Added `v0.13.0 - Codec Crate Skeleton And Decode Cursor`, `v0.17.0 - Core Limit And Budget Split`, `v0.53.0 - Carrier Framing Parser`, and `v0.54.0 - Carrier Parser Fuzzing Program`. |
| Variable-length identifiers are constructor-validated but not wire-parsed or canonical-profile validated. | Added borrowed identifier parsing in `v0.14.0` and prefix/parser rejection coverage in `v0.53.0`. |
| Semantic model crates and crypto crates depend on transport-named limits. | Added `v0.17.0 - Core Limit And Budget Split`. |
| Public identifier storage choices need an explicit indexing and zeroization policy. | Added `v0.18.0 - Public Identifier Storage Policy`. |
| Hybrid verification lacks a non-overridable coordinator, framed message representative, composite key binding, and exact suite policy. | Added `v0.30.0` through `v0.33.0` for the hybrid coordinator, composite key records, signed representative, and exact suite enforcement. |
| Provider scratch lifecycle and provider side-effect boundaries are not specified. | Added `v0.34.0 - Provider Scratch And Side-Effect Contract`. |
| COSE proof suites need protected-header, critical-metadata, and `kid` identity rules. | Folded COSE hardening into `v0.36.0 - COSE Proof Envelope Crate`; no incomplete COSE boundary is scheduled first. |
| Revocation and contradiction are vocabulary, not checkpoint-relative semantic validity. | Added `v0.39.0 - Semantic Validity Engine` and `v0.40.0 - Revocation, Conflict, And Checkpoint Roots`. |
| WHY bundles need distinct operational receipts and transparency receipts with inclusion, consistency, and non-inclusion proofs. | Added `v0.41.0 - Receipt Model Split` and `v0.42.0 - Transparency Receipt Integration` before `v0.45.0 - Explanation Bundle`. |
| Privacy requirements for pseudonymous keys, selective disclosure, and anonymous proof suites need to shape explanation bundles before profiles. | Added `v0.47.0 - Privacy And Disclosure Hardening` before offline, HTTP, blockchain, witness, and ZK work. |
| Carrier profiles need normative schemas, registries, critical-extension behavior, downgrade rules, and finality contracts. | Added `v0.51.0 - Profile Normative Specification Pack` before the first HTTP profile. |
| The wire version must remain draft/experimental until canonical BCX/1 is frozen. | Added `v0.52.0 - Draft Wire Version And Registry Gate`. |
| Parser fuzzing must precede untrusted network endpoints. | Added core parser tests to `v0.13.0` and carrier parser fuzzing in `v0.54.0 - Carrier Parser Fuzzing Program` before HTTP implementation starts. |
| Cryptographic conformance must precede reliance on COSE and hybrid provider surfaces. | Added `v0.37.0 - Cryptographic Conformance Program` before statement and attestation verification are used by profiles. |
| Normative security specifications should grow alongside implementation rather than appear only at the end. | Added `v0.50.0 - Core And Codec Specification Draft` and kept `v0.95.0 - Security Specification Freeze`. |
| Digest is an unnamed 32-byte value and commitments need domain separation, algorithm codes, empty-root rules, and migration policy. | Added `v0.11.0 - Commitment Suite And Registry Scaffold`. |
| Statement IDs must exclude themselves from their own hash preimage and exclude attestations, bindings, local availability, and transport metadata. | Added explicit preimage rules to `v0.15.0 - Canonical Statement Encoding And Sealed Identity`. |
| The plan did not resolve whether causal graph nodes are `EventId`, `StatementId`, or an authenticated mapping. | Added `v0.19.0` requirements for the graph-node identity decision and authenticated mapping behavior. |
| Key resolution and immutable trust snapshots were missing. | Added `v0.27.0 - Key Resolution And Trust Snapshots`. |
| Signing, entropy, private-key handles, and atomic hybrid signing were missing. | Added `v0.29.0 - Signing Provider Boundary`. |
| Capability references were opaque and capability verification was not scheduled. | Added `v0.24.0 - Capability Verification`. |
| Trusted time and replay need atomic check-and-record semantics, clock policy, sequence policy, and cache saturation behavior. | Added those requirements to `v0.23.0 - Validity And Atomic Replay Policy`. |
| Truth and assurance need orthogonal evidence facets or lattice behavior rather than a single mutually exclusive status enum. | Added early modeling in `v0.6.0`, dependency roles in `v0.22.0`, and semantic composition in `v0.39.0`. |
| Missing-parent admission must distinguish staged, structurally accepted, promoted, rejected, and garbage-collected objects. | Added those storage states and abuse bounds to `v0.21.0 - Missing Parent Reconciliation`. |
| Checkpoint continuity alone does not prevent equivocation or rollback. | Added anti-equivocation requirements to `v0.40.0 - Revocation, Conflict, And Checkpoint Roots`. |
| Multi-attestation support lacks acceptance semantics. | Added duplicate signer, canonical ordering, roles, threshold, and aggregation rules to `v0.35.0 - Attestation Verification And Multi-Attestation Policy`. |
| Deterministic CBOR restrictions need explicit duplicate-key, indefinite-length, integer, tag, text, unknown-field, and extension-stripping behavior. | Added those restrictions to `v0.12.0 - Canonical CBOR Codec Contract`. |
| Deterministic state needs metering, host-function, rollback, and cross-architecture resource rules before it is required for `1.0.0`. | Added those requirements to `v0.74.0 - Deterministic State Contract And Resource Model`. |
| Evidence modeling still used enum wording that could imply mutually exclusive truth states or a false total order. | Reworked `v0.6.0 - Evidence Facets And Assurance Lattice` to require orthogonal facets, composition, partial order, and no blanket ordering traits unless justified. |
| Aggregate decode budgeting was mentioned but did not define the accounting mechanism. | Expanded `v0.13.0` and `v0.17.0` with a concrete checked `DecodeBudget` covering bytes, nesting, entries, extensions, signatures, proofs, disclosures, decompressed/referenced bytes, and pre-crypto work units. |
| Actual classical and post-quantum provider crates were not scheduled before crypto conformance. | Added `v0.36.1 - Ed25519 Provider Crate` and `v0.36.2 - ML-DSA-65 Provider Crate` before `v0.37.0 - Cryptographic Conformance Program`. |
| Hybrid construction details were underspecified beyond verifying listed components. | Expanded `v0.30.0 - Hybrid Verification Coordinator` with BCX AND-composition versus external composite choice, component order, canonical serialization, prehash/context rules, key separation, stripping resistance, cross-protocol reuse resistance, and migration rules. |
| Policy references did not prove that policy was actually evaluated. | Added `v0.26.1 - Policy Record And Evaluation Contract` before key resolution and semantic validity. |
| Revocation and contradiction semantics needed typed targets, authorized revokers, scope, effective point, adjudication, and historical visibility rules. | Expanded `v0.40.0` and `v0.46.0` with concrete revocation and contradiction semantics. |
| Private disclosure cryptography needed more than raw hashes for low-entropy values. | Expanded `v0.26.0` and `v0.47.0` with salted/blinded commitments, AEAD suite IDs, nonce uniqueness, recipient/audience binding, ciphertext domain separation, length-leakage policy, and key rotation. |
| Offline and CLI parsing depended on core parser safety that was scheduled too late. | Added core prefix and streaming parser safety to `v0.13.0`, made `v0.53.0` carrier framing-specific, and made `v0.54.0` carrier parser fuzzing. |
| Basic graph and parser assurance was too late for persistent graph and carrier work. | Added early property, acyclicity, concurrent insertion, streaming chunk-boundary, prefix-before-allocation, and budget monotonicity requirements to `v0.13.0`, `v0.17.0`, `v0.20.0`, and `v0.21.0`. |
| Security controls documentation needs to mirror atomic replay, orphan DoS, signer entropy, policy evaluation, trust snapshots, and checkpoint equivocation. | Added security-controls update requirements to the responsible milestones: `v0.23.0`, `v0.26.1`, `v0.27.0`, `v0.29.0`, and `v0.40.0`. |

## Phase 0: Published Foundation And Direction Pivot

### v0.1.0 - Repository Foundation

Status: published.

Goal: initialize the serious Rust workspace and policy baseline.

Delivered:

- root `bcx` crate,
- focused no-std subcrates,
- EUPL-1.2 license,
- CI and local check script,
- dependency policy,
- security policy,
- implementation, version, threat-model, toolchain, and modularity docs.

### v0.2.0 - Protocol Family Pivot

Goal: change BCX from a narrow Fluxheim/HTTP-oriented protocol foundation into
a broader causal exchange protocol family without breaking the published crate
names.

Deliverables:

- preserve the original idea as `docs/original-idea.md`,
- update README and planning docs for the semantic overlay model,
- document core, profile, integration, proof, domain, and service families,
- create repository placeholders for future families,
- state that `bcx-core`, `bcx-model`, `bcx-crypto`, `bcx-policy`, and
  `bcx-wire` remain valid foundation crates,
- keep release hardening from the earlier `0.2.0` work,
- document that blockchain profiles are optional settlement or binding layers,
  not the BCX core.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_2_gate.sh` after pentest report exists

Exit criteria:

- contributors can understand BCX as one invariant causal core with many
  native bindings,
- no future-profile crate is added before it has a written security contract,
- `v0.2.0` can be tagged as the architectural pivot.

## Phase 1: Core Statement Foundation

### v0.3.0 - Statement Identity Vocabulary

Goal: define no-std identifiers for statements, subjects, realms, profiles,
proof suites, policies, checkpoints, and native bindings.

Deliverables:

- `StatementId`,
- `SubjectId`,
- `RealmId`,
- `ProfileId`,
- `ProofSuiteId`,
- `PolicyId`,
- `CheckpointId`,
- `NativeBindingId`,
- zero-value and length validation tests.

Verification:

- `cargo test -p bcx-core`
- `cargo test --workspace --no-default-features`

Exit criteria:

- every public identifier constructor rejects invalid zero or malformed values.

### v0.4.0 - Statement Body Skeleton

Goal: add the first BCX/1 statement body vocabulary without encoding or
signatures.

Deliverables:

- `Intent`,
- `Admission`,
- `Effect`,
- `Delegation`,
- `Revocation`,
- `Checkpoint`,
- `Contradiction`,
- common `StatementKind`,
- minimum required-field validation.

Verification:

- `cargo test -p bcx-model`

Exit criteria:

- BCX can name the lifecycle without relying on HTTP or blockchain types.

### v0.5.0 - Causal Edge Model

Goal: model causal parentage and relationship kinds.

Deliverables:

- causal edge type,
- parent-count bounds,
- relationship kinds,
- cycle-prevention hooks,
- missing-parent representation.

Verification:

- `cargo test -p bcx-model`
- multi-parent fixture tests

Exit criteria:

- a statement can point to bounded parents with explicit edge meaning.

### v0.5.1 - ML-DSA-65 Constant Correction

Goal: correct the ML-DSA-65 signature length before adding more cryptographic
surface.

Deliverables:

- change ML-DSA-65 signature length from `3,293` bytes to the standard
  `3,309` bytes,
- change hybrid Ed25519 plus ML-DSA-65 signature length from `3,357` bytes to
  `3,373` bytes,
- add tests for exact ML-DSA-65, Ed25519 plus ML-DSA-65, and off-by-one
  rejection lengths,
- document the FIPS 204 and RFC 9964 basis in crypto comments or release notes.

Verification:

- `cargo test -p bcx-crypto`
- `cargo test --workspace`
- `scripts/checks.sh`

Exit criteria:

- conformant ML-DSA-65 and hybrid signatures are not rejected because of an
  internal length constant.

### v0.6.0 - Evidence Facets And Assurance Lattice

Goal: prevent signed claims from being treated as proven reality or as one
mutually exclusive status.

Deliverables:

- independent evidence facets such as observed, signed, admitted, settled,
  contradicted, and revoked,
- `Unknown` modeled as absence of evidence rather than a positive proof state,
- contradiction coexisting with historical authentic evidence,
- assurance composition operation,
- partial order definition where mathematically justified,
- no blanket `Ord` or `PartialOrd` on assurance values unless the lattice
  rules justify it,
- unknown and redacted evidence markers,
- tests showing declared purpose is not verified truth.

Verification:

- `cargo test -p bcx-model`
- docs with warnings denied

Exit criteria:

- public APIs cannot collapse declared, observed, verified, enforced, and
  finalized claims into one truth state or a false total order.

### v0.7.0 - Statement Envelope

Goal: define the transport-independent statement container.

Deliverables:

- protocol version,
- realm,
- subject,
- optional audience type and signed audience binding,
- parents,
- policy references,
- validity window field shape,
- replay policy reference,
- disclosure policy reference,
- payload/body.

Verification:

- constructor tests,
- invalid audience and expiry shape tests.

Exit criteria:

- one statement shape is ready for sealed canonical encoding in `v0.15.0` and native
  binding in `v0.9.0`.

### v0.8.0 - Attestation Envelope

Goal: separate statements from signatures, witnesses, and proofs.

Deliverables:

- attestation type,
- issuer,
- key identifier,
- proof-suite identifier,
- proof bytes bounds,
- multi-attestation support.

Verification:

- `cargo test -p bcx-crypto`
- malformed proof metadata tests

Exit criteria:

- multiple issuers can attest to the same statement without changing its
  logical identity.

### v0.9.0 - Native Binding Envelope

Goal: model the binding from a BCX statement to a native operation without any
profile-specific dependency.

Deliverables:

- native binding type,
- profile identifier,
- native commitment digest,
- evidence commitment,
- binding status,
- profile extension slot.

Verification:

- binding validation tests,
- mutation tests for statement/profile mismatch.

Exit criteria:

- HTTP, Ethereum, Cardano, Bitcoin, XRP, and offline profiles can all share
  the same binding concept.

### v0.10.0 - Checkpoint Vocabulary

Goal: define checkpoint commitments before settlement profiles exist.

Deliverables:

- statement root,
- attestation root,
- binding root,
- policy root,
- revocation root,
- previous checkpoint reference,
- checkpoint sequence.

Verification:

- checkpoint validation tests,
- missing-root tests.

Exit criteria:

- one checkpoint is ready for settlement and receipt verification in `v0.43.0`.

## Phase 2: Commitment, Registry, And Canonical Codec

### v0.11.0 - Commitment Suite And Registry Scaffold

Goal: define domain-separated commitments before canonical bytes or IDs are
implemented.

Deliverables:

- `CommitmentSuite` vocabulary,
- first living `BCX-CORE/1` and `BCX-CODEC-CBOR/1` draft files,
- draft wire version marker,
- exact hash algorithm and code point,
- domain separation labels for statements, attestations, bindings, policies,
  keys, checkpoints, Merkle leaves, and Merkle internal nodes,
- version and codec binding in every committed preimage,
- empty-root construction,
- algorithm migration and downgrade policy,
- registry scaffold for type, algorithm, extension, profile, and suite IDs,
- reserved and experimental code point ranges,
- initial canonical conformance-vector home.

Verification:

- commitment fixture tests,
- registry collision tests,
- docs checks.

Exit criteria:

- a raw 32-byte digest cannot be interpreted without its commitment domain and
  suite context,
- canonical commitments and draft wire/version labels have a living spec home
  before codec implementation continues.

### v0.12.0 - Canonical CBOR Codec Contract

Goal: define deterministic CBOR as the first canonical binary representation.

Deliverables:

- deterministic CBOR security contract,
- canonical map ordering rules,
- duplicate map key rejection before normalization,
- indefinite length rejection,
- shortest integer and length encodings only,
- float policy,
- tag allow-list,
- UTF-8 text policy or invariant-core text avoidance rule,
- unknown noncritical signed-field preservation,
- unknown critical field rejection,
- extension stripping resistance,
- JSON inspection boundary,
- no-std and alloc impact analysis,
- dependency admission notes.

Verification:

- docs checks,
- dependency policy review for the CBOR crate admission,
- canonicality fixture review.

Exit criteria:

- no signed or hashed object can use ad hoc JSON, Rust memory layout, or vague
  deterministic-CBOR shorthand.

### v0.13.0 - Codec Crate Skeleton And Decode Cursor

Goal: add `bcx-codec` as a no-std crate with bounded borrowed decoding before
full object decoding exists.

Deliverables:

- crate scaffold,
- encode/decode error model,
- borrowed decode cursor,
- core prefix parser hook for draft/version/type/flags before allocation,
- streaming decode support with chunk-boundary tests,
- assertion that no allocation, hashing, key lookup, or cryptographic work
  occurs before prefix validation,
- `DecodeBudget` type,
- total bytes consumed,
- maximum depth,
- map and array entry debits,
- maximum byte lengths,
- maximum map entries, array entries, extension counts, proof counts, and
  signature counts,
- disclosed field debits,
- decompressed size and referenced byte debits,
- pre-crypto work-unit debits,
- checked arithmetic with immediate exhaustion and overflow errors,
- monotonic debiting across nested and streaming decoders,
- malformed-input fail-fast tests.

Verification:

- `cargo test -p bcx-codec`,
- `cargo test --workspace --no-default-features`,
- streaming parser tests across every supported chunk boundary,
- budget monotonicity and bounded-work tests.

Exit criteria:

- decoder bounds and prefix validation exist before full decoding, offline
  bundles, CLI parsing, or carrier endpoints exist.

### v0.14.0 - Canonical Identifier Encoding

Goal: produce stable bytes for core identifiers and digests.

Deliverables:

- canonical bytes for IDs,
- canonical bytes for digests,
- borrowed wire parsing for variable-length IDs,
- rejection fixtures for noncanonical identifier encodings,
- cross-platform fixtures,
- fixture regeneration guard.

Verification:

- `cargo test -p bcx-codec`,
- fixture diff check.

Exit criteria:

- all supported platforms produce the same bytes for identifier fixtures.

### v0.15.0 - Canonical Statement Encoding And Sealed Identity

Goal: encode statements deterministically and derive statement IDs only from
canonical typed statement preimages.

Deliverables:

- statement encoder,
- statement decoder,
- canonical parent ordering rules,
- adjacent duplicate-parent rejection,
- statement ID derivation from canonical statement bytes,
- sealed `CanonicalStatementBytes` or `StatementCommitment` producer,
- rule that `StatementId` is excluded from its own hash preimage,
- rule that constructors do not accept a caller-derived ID,
- exclusion of attestations, native bindings, local availability, and transport
  metadata from the statement preimage,
- inclusion of schema version and security-relevant extensions,
- canonical edge semantics limited to parent ID plus relationship,
- mutation fixtures.

Verification:

- canonical test vectors,
- malformed fixture tests,
- no-default-features pass,
- detached-byte substitution fixtures.

Exit criteria:

- the same logical statement produces the same `StatementId` everywhere,
- caller-supplied statement IDs cannot bypass canonical identity checks.

### v0.16.0 - Canonical Attestation, Binding, And Checkpoint Encoding

Goal: encode attestations, native bindings, and checkpoints deterministically
after statement identity is sealed.

Deliverables:

- attestation encoding,
- binding encoding,
- checkpoint encoding,
- proof byte limits,
- native evidence limits,
- root ordering rules,
- previous-checkpoint validation,
- mutation fixtures.

Verification:

- `cargo test -p bcx-codec`,
- tamper fixture tests,
- cross-platform checkpoint vector tests.

Exit criteria:

- attestation, binding, and checkpoint commitments are stable and
  mutation-sensitive.

## Phase 3: Limits, Graph Admission, Replay, And Authority

### v0.17.0 - Core Limit And Budget Split

Goal: remove transport-named limit coupling from semantic model and crypto
verification.

Deliverables:

- general graph limits in core or policy,
- verification budgets in core or policy,
- byte-decoding limits kept in wire or codec,
- migration of model validation from `bcx-wire::WireLimits`,
- migration of crypto verification limits from `bcx-wire::WireLimits`,
- aggregate budget vocabulary shared by parser and verifier,
- common checked exhaustion and overflow error model,
- pre-crypto work budget policy.

Verification:

- `cargo tree -p bcx-model`,
- `cargo tree -p bcx-crypto`,
- workspace tests and no-default-features tests,
- tests that budgeted parser failures happen before allocation or crypto work.

Exit criteria:

- semantic model and cryptographic verification do not depend on a
  transport-named crate for non-wire budgets.

### v0.18.0 - Public Identifier Storage Policy

Goal: decide which public commitments are indexable and which buffers require
zeroization before graph storage APIs freeze.

Deliverables:

- public identifier storage policy,
- graph-indexing impact review,
- explicit `Hash` and `Ord` decision for public identifiers,
- zeroization policy for public commitments versus sensitive scratch,
- migration notes for affected APIs.

Verification:

- API review,
- graph indexing benchmarks or microbenchmarks where relevant.

Exit criteria:

- identifier ergonomics and memory clearing behavior are documented as a
  security and performance tradeoff, not an accident.

### v0.19.0 - Graph Store Contract

Goal: define graph insertion as one atomic causal operation before explanation,
offline bundles, or storage integrations depend on causal graph state.

Deliverables:

- `GraphStore` contract,
- graph-node identity decision: canonical `StatementId`, derived `EventId`, or
  authenticated mapping,
- `insert_checked(event, parents)` semantics,
- reachability check requirements,
- compact `CauseCapsule` normalization into causal edges or explicit
  deprecation behavior,
- adjacent duplicate checks over canonical parent order,
- public identifier indexing policy hooks.

Verification:

- contract tests with deterministic in-memory graph fixtures,
- duplicate, self-parent, and two-node cycle fixtures.

Exit criteria:

- graph acceptance is specified as validation plus insertion in one operation,
  not as a boolean pre-check against a stale snapshot.

### v0.20.0 - Atomic Graph Store Skeleton

Goal: add the first no-std graph-store implementation for tests and core
verification.

Deliverables:

- crate or module skeleton,
- caller-provided or const-generic storage,
- no unexpectedly large stack allocations,
- in-memory bounded graph store,
- atomic insert API,
- iterative reachability implementation,
- no recursion in traversal,
- explicit node, edge, and depth limits.

Verification:

- arbitrary insertion-order tests,
- property tests for atomic admission,
- bounded acyclicity checks,
- concurrent insertion tests for check-then-insert races,
- cycle and duplicate-delivery tests,
- no-default-features workspace pass.

Exit criteria:

- every accepted insertion leaves the bounded in-memory graph acyclic.

### v0.21.0 - Missing Parent Reconciliation

Goal: make missing-parent staging preserve graph integrity and resist orphan
storage abuse.

Deliverables:

- unresolved edge table,
- staged, structurally checked, promoted, rejected, and garbage-collected
  object states,
- parent arrival reconciliation operation,
- deterministic reconciliation ordering,
- recheck of cycles when missing parents arrive,
- unresolved-parent table size bound,
- per-issuer orphan count bound,
- orphan lifetime and retention policy,
- fetch attempt and referenced-byte budgets,
- garbage collection rules that preserve checkpointed evidence,
- deterministic missing-node explanation marker,
- incomplete graph status.

Verification:

- mutually missing parent fixtures,
- late-parent cycle rejection tests,
- property tests for orphan promotion ordering,
- orphan table saturation tests,
- explanation bundle tests for incomplete graphs.

Exit criteria:

- two objects accepted while unresolved cannot form an accepted cycle when both
  become available,
- unresolved objects are staged and not causally usable until deterministic
  promotion succeeds,
- validly shaped missing-parent references cannot fill storage without bounds.

### v0.22.0 - Relationship Semantics And Edge Roles

Goal: enforce semantic constraints that require resolved parent metadata or
profile policy.

Deliverables:

- relationship cardinality rules,
- canonical dependency roles such as required authorization, required input,
  and observational context,
- target-kind checks for delegation, retry, scheduling, derivation, and joins,
- profile hook for domain-specific relationship constraints,
- rule that profile hooks may only narrow or supplement core edge invariants,
  never weaken them,
- fail-closed behavior for missing required parent metadata.

Verification:

- one-parent `JoinedFrom` rejection where policy requires several parents,
- multiple `RetryOf` rejection where policy requires one retry source,
- wrong-target-kind fixtures,
- required versus observational dependency fixtures.

Exit criteria:

- relationship kinds and dependency roles have protocol meaning once the
  verifier has enough parent context.

### v0.23.0 - Validity And Atomic Replay Policy

Goal: make freshness and replay resistance explicit before any consequential
carrier profile exists.

Deliverables:

- validity window,
- expiry/freshness type with overflow-safe remaining-time checks,
- integration-supplied `Clock`,
- allowed skew and precision policy,
- offline checkpoint-relative freshness,
- nonce scope over realm, issuer, audience, operation class, and nonce,
- atomic `check_and_record` replay store trait,
- crash persistence and rollback behavior,
- issuer sequence policy,
- sequence-gap and concurrent-request policy,
- cache saturation behavior,
- idempotency policy,
- security-controls update for atomic replay and freshness.

Verification:

- expired statement tests,
- duplicate nonce and sequence tests,
- atomic replay race fixtures,
- cache saturation tests.

Exit criteria:

- consequential statements cannot be accepted without atomic replay and
  freshness policy.

### v0.24.0 - Capability Verification

Goal: turn opaque `CapabilityRef` values into verifiable authority evidence.

Deliverables:

- canonical capability commitment,
- issuer and holder binding,
- resource, action, scope, and audience binding,
- validity and replay fields,
- proof-of-possession requirement,
- attenuation and delegation chain model,
- maximum chain depth,
- revocation and policy epoch binding,
- deterministic resolution of multiple applicable capabilities.

Verification:

- proof-of-possession fixtures,
- wrong holder, resource, action, audience, and epoch fixtures,
- attenuation and chain-depth tests.

Exit criteria:

- admission can establish actual authority from capability evidence rather than
  an opaque digest alone.

### v0.25.0 - Delegation Narrowing

Goal: permit authority delegation that narrows but never silently broadens.

Deliverables:

- delegation body validation,
- child scope checks,
- purpose narrowing,
- time-window narrowing,
- capability and policy epoch links,
- maximum delegation depth.

Verification:

- valid narrowing tests,
- broadening rejection tests.

Exit criteria:

- a child delegation cannot remove parent restrictions.

### v0.26.0 - Disclosure Policy

Goal: encode what can be revealed, redacted, committed, encrypted, or withheld.

Deliverables:

- disclosure policy vocabulary,
- redaction marker,
- private evidence commitment,
- salted or blinded private commitments for low-entropy values,
- public evidence marker,
- encrypted field disclosure contract,
- AEAD suite identifier,
- nonce uniqueness rules,
- recipient and audience binding,
- ciphertext domain separation,
- padding or explicit length-leakage policy,
- disclosure key distribution and rotation behavior,
- unknown evidence marker.

Verification:

- redaction validation tests,
- missing evidence tests,
- encrypted disclosure binding fixtures,
- low-entropy private commitment fixtures,
- nonce reuse rejection tests.

Exit criteria:

- explanations can preserve privacy without pretending to be complete.

### v0.26.1 - Policy Record And Evaluation Contract

Goal: prove that a policy was evaluated, not merely referenced by ID.

Deliverables:

- canonical `PolicyRecord` commitment,
- policy language or evaluator identifier and version,
- exact evaluation-input commitment,
- resolver or trust snapshot used during evaluation,
- deterministic decision result,
- signed admission-decision evidence,
- distinction between `references policy X` and `verified as evaluated under
  policy X`,
- security-controls update for policy evaluation evidence.

Verification:

- policy commitment fixtures,
- wrong evaluator, version, input, and resolver fixtures,
- admission evidence mutation tests.

Exit criteria:

- admission can prove which policy artifact was evaluated, with which inputs
  and trust snapshot, before semantic validity consumes the decision.

## Phase 4: Keys, Signing, Verification, And Proof Envelopes

### v0.27.0 - Key Resolution And Trust Snapshots

Goal: resolve verification keys from immutable, policy-qualified trust state
before primitive verification occurs.

Deliverables:

- `VerificationKeyRecord`,
- `KeyResolver` or immutable `TrustSnapshot`,
- issuer, realm, audience, algorithm, and key-usage binding,
- validity intervals,
- rotation, compromise, and revocation evidence,
- trust-anchor selection,
- deterministic failure for ambiguous `kid` matches,
- no network I/O during primitive verification,
- security-controls update for trust snapshots.

Verification:

- ambiguous key fixtures,
- wrong realm, audience, usage, and interval fixtures,
- revocation and rotation fixtures.

Exit criteria:

- primitive verifiers receive resolved key records and cannot resolve trust on
  their own.

### v0.28.0 - Verifier Provider Boundary

Goal: define crypto verification without choosing production providers.

Deliverables:

- narrow primitive verifier traits,
- proof-suite policy hook,
- provider capability metadata,
- provider error model,
- opaque external error contract,
- deterministic test verifier.

Verification:

- `cargo test -p bcx-crypto`,
- provider failure tests.

Exit criteria:

- primitive verification providers cannot resolve keys, perform network I/O, or
  choose policy inside the core verification operation.

### v0.29.0 - Signing Provider Boundary

Goal: define how BCX creates attestations without exposing private key material
or partial hybrid signatures.

Deliverables:

- opaque private-key handles,
- signer capability metadata,
- injected RNG or entropy interface for no-std,
- ML-DSA and SLH-DSA deterministic versus hedged mode policy,
- algorithm context selection,
- secret scratch zeroization,
- atomic hybrid signing,
- no partial hybrid signature release when one component fails,
- key generation and import boundaries,
- security-controls update for signer entropy and private-key handling.

Verification:

- fake signer tests,
- entropy failure tests,
- partial hybrid failure tests,
- scratch zeroization tests where observable.

Exit criteria:

- signing has an explicit provider boundary and failure model before production
  proof suites create attestations.

### v0.30.0 - Hybrid Verification Coordinator

Goal: make hybrid signature acceptance non-overridable by provider traits.

Deliverables:

- BCX-owned hybrid coordinator,
- BCX proprietary AND-composition versus standardized composite construction
  decision,
- component order and canonical serialization,
- prehash and context-string rules,
- separate primitive provider traits for Ed25519 and ML-DSA-65,
- component key separation,
- signature stripping resistance,
- cross-protocol reuse resistance,
- exact suite dispatch,
- coordinator-owned AND-combination,
- public-data early-failure policy,
- invalid-input cost budget before PQ dispatch,
- migration rules if an external composite draft changes.

Verification:

- hybrid truth-table tests,
- stripped-component tests,
- provider override rejection tests,
- hostile invalid-input benchmark.

Exit criteria:

- hybrid acceptance cannot be redefined by one provider implementation.

### v0.31.0 - Composite Key Record

Goal: bind hybrid keys as immutable ordered component sets.

Deliverables:

- composite key record,
- component algorithm and usage binding,
- realm and profile binding,
- validity interval,
- revocation state reference,
- domain-separated composite key commitment.

Verification:

- component substitution tests,
- component order mutation tests,
- cross-suite key reuse rejection tests.

Exit criteria:

- a hybrid signature verifies against one exact composite key commitment.

### v0.32.0 - Signed Message Representative

Goal: sign a framed BCX message representative rather than raw caller bytes.

Deliverables:

- protocol label,
- object type,
- canonical version,
- algorithm suite,
- profile and realm,
- audience,
- payload commitment,
- policy epoch.

Verification:

- frame mutation tests,
- wrong audience, realm, suite, and policy epoch fixtures,
- replay-portability rejection fixtures.

Exit criteria:

- signatures are bound to BCX context and cannot be replayed as a different
  object class or audience.

### v0.33.0 - Exact Suite Policy Enforcement

Goal: prevent sender-selected downgrade inside high-assurance profiles.

Deliverables:

- operation-class to exact-suite policy mapping,
- policy epoch suite binding,
- denial of multi-algorithm sender choice for high-assurance operations,
- compatibility notes for lower-assurance profile policies.

Verification:

- downgrade fixtures,
- mixed-suite fixtures,
- profile policy tests.

Exit criteria:

- high-assurance acceptance depends on the required suite, not the sender's
  weakest admitted option.

### v0.34.0 - Provider Scratch And Side-Effect Contract

Goal: specify provider memory, diagnostics, and side-effect boundaries.

Deliverables:

- structured verification request,
- provider metadata and capability model,
- scratch length declaration,
- caller-owned sensitive scratch buffer,
- wipe-on-normal and wipe-on-error rule,
- no network, storage, or key-resolution side effects in primitive
  verification,
- redacted diagnostic policy.

Verification:

- fake provider tests,
- scratch wipe tests where the implementation can observe the buffer,
- error redaction tests.

Exit criteria:

- provider integration has a security contract before production providers are
  admitted.

### v0.35.0 - Attestation Verification And Multi-Attestation Policy

Goal: verify attestations over canonical statement commitments and define how
several attestations combine.

Deliverables:

- proof-suite dispatch,
- issuer and key checks,
- duplicate key and issuer rejection,
- canonical signer ordering,
- unique signer counting,
- signer roles and trust domains,
- all-of, any-of, threshold, and named-role policies,
- optional-attestation failure semantics,
- assurance aggregation rules,
- unknown proof-suite handling.

Verification:

- valid and invalid proof fixtures,
- duplicate signer fixtures,
- threshold and role policy fixtures,
- unknown-suite tests.

Exit criteria:

- attestations are verifiable and aggregatable without native profile
  dependencies.

### v0.36.0 - COSE Proof Envelope Crate

Goal: add `bcx-proof-cose` as the first standard proof-envelope crate after
key, signer, verifier, and suite policy prerequisites exist.

Deliverables:

- COSE proof-suite identifiers,
- canonical COSE proof envelope and verification profile,
- COSE signature envelope binding,
- detached payload verification boundary over sealed commitments,
- key identifier binding through trust snapshots,
- protected-header algorithm binding,
- critical-header behavior,
- duplicate-header rejection,
- external authenticated data binding,
- `kid` as lookup hint rather than globally unique identity,
- negative fixtures.

Verification:

- `cargo test -p bcx-proof-cose`,
- malformed COSE fixture tests,
- wrong external authenticated data fixtures.

Exit criteria:

- BCX has a canonical COSE proof envelope and verification profile before
  `1.0.0`.

### v0.36.1 - Ed25519 Provider Crate

Goal: add the first admitted classical signature provider without contaminating
no-std core traits.

Deliverables:

- optional Ed25519 provider crate or integration,
- feature-gated provider selection,
- signing known-answer tests,
- verification known-answer tests,
- malformed key and signature vectors,
- cross-provider-ready test interface.

Verification:

- `cargo test -p <ed25519-provider-crate>`,
- RFC 8032 vector smoke tests,
- no root dependency regression.

Exit criteria:

- BCX has one admitted classical provider for signing and verification before
  crypto conformance is claimed.

### v0.36.2 - ML-DSA-65 Provider Crate

Goal: add the first admitted post-quantum signature provider without
contaminating no-std core traits.

Deliverables:

- optional ML-DSA-65 provider crate or integration,
- feature-gated provider selection,
- signing known-answer tests,
- verification known-answer tests,
- malformed key and signature vectors,
- cross-provider signature generation and verification fixtures.

Verification:

- `cargo test -p <ml-dsa-provider-crate>`,
- NIST KAT or ACVP-vector smoke tests where available,
- no root dependency regression.

Exit criteria:

- BCX has one admitted post-quantum provider for signing and verification
  before hybrid and COSE conformance are claimed.

### v0.37.0 - Cryptographic Conformance Program

Goal: prove primitive and hybrid verification against external vectors and
adversarial suite mutations before profiles depend on these surfaces.

Deliverables:

- RFC 8032 Ed25519 vectors,
- NIST ACVP or KAT plan for ML-DSA and SLH-DSA,
- RFC 9964 JOSE/COSE vector plan,
- two-provider differential verification plan,
- hybrid component swap and strip corpus,
- downgrade and cross-suite reuse corpus.

Verification:

- conformance smoke tests,
- hybrid negative corpus.

Exit criteria:

- cryptographic acceptance is measured against standards vectors and
  adversarial suite mutations.

## Phase 5: Semantic Validity, Receipts, Explanation, And Offline Use

### v0.38.0 - Statement Verification

Goal: verify canonical statement identity and structural validity.

Deliverables:

- statement ID verification,
- canonical statement commitment verification,
- detached-byte substitution rejection,
- required field checks,
- audience binding checks,
- expiry/freshness checks,
- policy reference checks.

Verification:

- tamper tests,
- wrong-audience tests,
- detached substitution tests.

Exit criteria:

- changing any security-relevant statement field invalidates verification.

### v0.39.0 - Semantic Validity Engine

Goal: derive validity as a checkpoint-relative result instead of embedding
absolute invalidation into immutable statements.

Deliverables:

- semantic validity result type,
- orthogonal evidence facets or lattice behavior,
- dimensions for cryptographic validity, semantic support, revocation,
  conflict, completeness, and assurance,
- required-dependency propagation,
- deny-overrides rule for consequential operations,
- fail-closed behavior for missing required authorization evidence.

Verification:

- revocation and contradiction fixtures,
- required versus observational dependency fixtures,
- deny-overrides tests,
- assurance composition fixtures.

Exit criteria:

- BCX can distinguish authentic history from usable authority.

### v0.40.0 - Revocation, Conflict, And Checkpoint Roots

Goal: make semantic validity and checkpoint continuity non-bypassable across
policy, revocation, conflict, and checkpoint changes.

Deliverables:

- signed policy epoch input,
- revocation root input,
- conflict root input,
- typed revocation targets for statements, keys, capabilities, delegations, and
  policies,
- authorized revoker rules,
- revocation scope and effective causal point,
- prospective versus retroactive effect,
- reason and supersession references,
- checkpoint issuer and authority,
- strictly monotonic sequence rules,
- fork and equivocation evidence,
- rollback detection,
- signed tree size and root,
- consistency proofs between checkpoints,
- deterministic handling for competing roots,
- witness or gossip expectations for high-assurance profiles,
- verification cache key including roots,
- cache invalidation on root change,
- non-inclusion proof interface before claiming not revoked, with the concrete
  transparency proof model completed in `v0.42.0`,
- security-controls update for checkpoint equivocation.

Verification:

- stale cache fixtures,
- non-inclusion proof fixtures,
- root-change invalidation tests,
- fork and rollback fixtures.

Exit criteria:

- verification results cannot outlive the revocation, conflict, and checkpoint
  roots they were evaluated against.

### v0.41.0 - Receipt Model Split

Goal: separate operational receipts from transparency receipts before WHY
bundles depend on either concept.

Deliverables:

- operational receipt vocabulary for admission, execution, observation,
  witness, settlement, and synchronization,
- transparency receipt vocabulary for inclusion, consistency, disclosure, and
  non-inclusion,
- receipt-to-statement commitment rules,
- receipt assurance classification.

Verification:

- receipt validation fixtures,
- wrong receipt class fixtures,
- explanation bundle receipt tests.

Exit criteria:

- BCX does not treat a component observation as an append-only transparency
  proof, or the reverse.

### v0.42.0 - Transparency Receipt Integration

Goal: integrate COSE receipt concepts for append-only proof evidence.

Deliverables:

- COSE receipt profile decision,
- Merkle inclusion proof model,
- consistency proof model,
- non-inclusion proof model,
- disclosure proof model,
- checkpoint-root binding.

Verification:

- inclusion and consistency fixtures,
- wrong-root fixtures,
- non-inclusion mutation tests.

Exit criteria:

- WHY bundles can carry transparency evidence without inventing a separate
  incompatible receipt envelope.

### v0.43.0 - Settlement And Checkpoint Verification

Goal: verify checkpoint membership, continuity, anti-equivocation evidence, and
settlement policy.

Deliverables:

- primary backend policy,
- primary-with-witnesses policy,
- require-all policy,
- threshold settlement policy,
- normalized finality status,
- membership proof model,
- previous-checkpoint check,
- root consistency checks,
- issuer and sequence checks,
- fork/equivocation evidence handling,
- missing-root behavior.

Verification:

- threshold validation tests,
- finality transition tests,
- checkpoint fixture tests,
- continuity tests,
- equivocation fixtures.

Exit criteria:

- a checkpoint can express and verify several settlement or witness receipts
  without requiring a blockchain.

### v0.44.0 - Explain Crate Skeleton

Goal: add `bcx-explain` for bounded WHY semantics after graph, validity, and
receipt prerequisites exist.

Deliverables:

- crate scaffold,
- query type,
- query direction,
- maximum depth,
- maximum nodes,
- maximum response bytes,
- iterative traversal rule with explicit work queue,
- visited or tri-color traversal state,
- cancellation marker.

Verification:

- `cargo test -p bcx-explain`,
- oversized query tests.

Exit criteria:

- no WHY query can request unbounded work.

### v0.45.0 - Explanation Bundle

Goal: return bounded proof bundles for local/offline verification using the
receipt, privacy, and contradiction semantics scheduled in `v0.41.0`,
`v0.42.0`, `v0.46.0`, and `v0.47.0`.

Deliverables:

- explanation bundle type,
- canonical manifest with bundle version, target claim, purpose, policy epoch,
  trust roots, limits, and freshness,
- deduplicated statement table addressed by commitment,
- statement references,
- attestation references,
- binding references,
- operational receipt references,
- transparency receipt references,
- missing, withheld, redacted, truncated, and stale markers,
- contradiction marker slots populated by the `v0.46.0` semantics,
- disclosure map binding fields to plaintext, ciphertext, commitments, or
  predicate proofs,
- final bundle commitment or signature.

Verification:

- bundle validation tests,
- missing-parent fixture tests,
- disclosure map fixtures.

Exit criteria:

- an offline verifier can see what is proven, missing, redacted, stale,
  withheld, truncated, or unknown, and can carry contradiction data once
  `v0.46.0` completes.

### v0.46.0 - Contradiction Handling

Goal: preserve conflicting claims without overwriting historical evidence.

Deliverables:

- contradiction statement checks,
- conflicting claim relation,
- contradiction claim selectors or predicates,
- supporting evidence links,
- resolution or adjudication statements,
- visibility rule that resolved contradictions remain historical evidence
  unless an explicit policy says otherwise,
- target-kind and authority checks,
- checkpoint-relative conflict status,
- non-invalidation rule for historical authentic evidence,
- assurance summary,
- explanation output for conflicts.

Verification:

- conflicting fixture tests,
- no-overwrite tests,
- authentic-but-unusable fixtures.

Exit criteria:

- BCX can represent `A says sent` and `B says not received` at the same time
  without erasing either authentic claim.

### v0.47.0 - Privacy And Disclosure Hardening

Goal: shape wire-level privacy before offline, HTTP, blockchain, witness, and
ZK profiles use WHY bundles.

Deliverables:

- audience-specific pseudonymous key guidance,
- stable global key ID avoidance policy,
- salted or blinded private commitment guidance for low-entropy values,
- selective disclosure binding,
- encrypted field disclosure contract,
- AEAD suite identification,
- nonce uniqueness rules,
- recipient and audience binding,
- ciphertext domain separation,
- padding or explicit length-leakage policy,
- key distribution and rotation behavior,
- anonymous credential or ZK proof-suite admission hook,
- privacy review for issuer, realm, profile, and key metadata.

Verification:

- disclosure map tests,
- cross-audience linkability review,
- wrong-audience encrypted disclosure fixtures,
- ciphertext metadata mutation fixtures.

Exit criteria:

- privacy-sensitive deployments have protocol hooks for minimizing linkability
  and over-disclosure.

### v0.48.0 - Offline Profile

Goal: add `bcx-offline` for air-gapped bundles after explanation and privacy
rules exist.

Deliverables:

- offline bundle profile,
- bundle manifest,
- evidence file commitments,
- detached private evidence references,
- pinned trust and checkpoint roots,
- explicit incompleteness behavior.

Verification:

- `cargo test -p bcx-offline`,
- tampered bundle tests.

Exit criteria:

- BCX can work without HTTP, Fluxheim, or any blockchain.

### v0.49.0 - CLI Skeleton

Goal: add `bcx-cli` for inspection without making it a root dependency.

Deliverables:

- `bcx verify`,
- `bcx why`,
- `bcx inspect`,
- fixture-driven command tests.

Verification:

- CLI smoke tests,
- no root dependency regression.

Exit criteria:

- developers can inspect offline BCX objects locally.

## Phase 6: Specifications, Wire Parser, Fuzzing, HTTP, And Fluxheim

### v0.50.0 - Core And Codec Specification Draft

Goal: start normative security specifications alongside implementation, not at
the end.

Deliverables:

- `BCX-CORE/1` working draft,
- `BCX-CODEC-CBOR/1` working draft,
- commitment suite text,
- statement preimage text,
- graph admission text,
- verification and validity text.

Verification:

- docs checks,
- traceability review against implemented APIs.

Exit criteria:

- implementation behavior is traceable to written protocol requirements before
  carrier profiles start.

### v0.51.0 - Profile Normative Specification Pack

Goal: require every profile to specify both protocol bytes and security
semantics.

Deliverables:

- profile CDDL or schema template,
- numeric registry requirements,
- critical-extension behavior,
- profile security contract template,
- downgrade rules,
- finality rules,
- conformance vector requirements.

Verification:

- docs checks,
- profile template validation against HTTP, offline, witness, and Ethereum
  examples.

Exit criteria:

- Rust traits are supported by normative profile specifications, not treated as
  the whole protocol.

### v0.52.0 - Draft Wire Version And Registry Gate

Goal: prevent experimental encodings from being presented as frozen `BCX/1`.

Deliverables:

- draft wire version marker,
- experimental registry range,
- rule for promoting draft objects to `BCX/1`,
- release gate that checks unstable wire labels in pre-1.0 artifacts,
- documentation of compatibility expectations.

Verification:

- metadata validation script,
- docs checks,
- sample draft-vector validation.

Exit criteria:

- BCX uses draft/experimental wire labels until the normative BCX/1
  representation is frozen.

### v0.53.0 - Carrier Framing Parser

Goal: bind carrier-specific framing to the core prefix, budget, and parser
rules established in `v0.13.0` and `v0.17.0`.

Deliverables:

- fixed wire prefix,
- magic value,
- wire object type,
- draft/version fields,
- flags and critical-flag behavior,
- exact frame-length check,
- trailing-byte rejection,
- aggregate decode budget integration,
- assertion that carrier parsers do not allocate, hash, resolve keys, or verify
  signatures before core prefix validation.

Verification:

- malformed prefix fixtures,
- unsupported version, type, and critical-flag fixtures,
- short, long, and trailing-byte fixtures.

Exit criteria:

- carrier inputs reuse the fail-fast parser boundary before high-cost work.

### v0.54.0 - Carrier Parser Fuzzing Program

Goal: fuzz every carrier-facing parser before network endpoints are exposed.

Deliverables:

- cargo-fuzz targets for header, identifiers, statements, signatures, WHY
  bundles, and profile parsers,
- malformed seed corpus,
- truncation, nonminimal integer, duplicate key, trailing byte, deep nesting,
  unknown critical field, and oversized PQ signature seeds,
- accepted-object re-encode identity assertion,
- differential CBOR decode plan.

Verification:

- local fuzz smoke,
- corpus regression tests,
- per-profile fuzz target added for every profile milestone from `v0.55.0`
  onward that introduces parser surface.

Exit criteria:

- every parser has a reproducible malformed-input corpus and a fuzz entry
  point before HTTP implementation starts.

### v0.55.0 - HTTP Profile Security Contract

Goal: define `BCX-HTTP/1` before implementation.

Deliverables:

- attached mode contract,
- encapsulated mode contract,
- committed HTTP components,
- intermediary mutation rules,
- replay rules,
- limits,
- no state-changing 0-RTT rule.

Verification:

- docs checks,
- threat-model update.

Exit criteria:

- implementation cannot start without a precise HTTP security contract.

### v0.56.0 - HTTP Profile Crate

Goal: add dependency-light `bcx-http`.

Deliverables:

- profile identifiers,
- header names,
- attached-mode commitment builder,
- encapsulated-mode envelope model.

Verification:

- `cargo test -p bcx-http`,
- mutation tests for committed components.

Exit criteria:

- HTTP commitments can be verified without Hyper or Axum dependencies.

### v0.57.0 - HTTP Hyper Integration

Goal: add `bcx-http-hyper` as the first concrete HTTP integration.

Deliverables:

- request extraction,
- response receipt extraction,
- header validation,
- body digest integration.

Verification:

- integration tests,
- malformed request tests.

Exit criteria:

- one real Rust HTTP stack can carry BCX objects.

### v0.58.0 - Fluxheim Profile Contract

Goal: document exactly what Fluxheim can observe and enforce.

Deliverables:

- Fluxheim ingress effect scope,
- route decision scope,
- upstream response scope,
- cache decision scope,
- non-claims about upstream database commits.

Verification:

- docs checks,
- threat-model update.

Exit criteria:

- Fluxheim integration cannot overclaim application effects.

### v0.59.0 - Fluxheim Integration Skeleton

Goal: add `bcx-fluxheim` as an optional integration crate.

Deliverables:

- policy adapter traits,
- receipt builder,
- route decision binding,
- local WHY fixture.

Verification:

- integration fixture tests,
- no root dependency regression.

Exit criteria:

- Fluxheim can emit local BCX causal receipts.

### v0.60.0 - Two-Fluxheim Demonstration

Goal: trace one signed operation across two Fluxheim nodes.

Deliverables:

- local smoke script,
- signed intent,
- admission,
- effect,
- explanation bundle.

Verification:

- smoke test,
- tamper test.

Exit criteria:

- BCX proves useful over HTTP before any blockchain integration exists.

## Phase 7: Settlement Profiles And Additional Bindings

### v0.61.0 - Ethereum Profile Security Contract

Goal: define what `BCX-ETHEREUM/1` commits and can prove.

Deliverables:

- chain ID commitment,
- contract address commitment,
- calldata digest commitment,
- value commitment,
- sender or authorization commitment,
- transaction and log position,
- block hash and reorg handling,
- expiry and nullifier rules,
- finality policy.

Verification:

- docs checks,
- threat-model update.

Exit criteria:

- Ethereum implementation cannot start without a precise security contract.

### v0.62.0 - Ethereum Profile Crate

Goal: add dependency-light `bcx-ethereum`.

Deliverables:

- Ethereum binding vocabulary,
- checkpoint anchoring model,
- finality status mapping,
- mock receipt verification.

Verification:

- `cargo test -p bcx-ethereum`,
- mutation tests.

Exit criteria:

- Ethereum can bind and settle checkpoints conceptually without Alloy yet.

### v0.63.0 - Ethereum Alloy Integration

Goal: add `bcx-ethereum-alloy` for concrete RPC and primitive integration.

Deliverables:

- RPC receipt adapter,
- chain ID fetch adapter,
- transaction evidence builder,
- testnet-ready fixtures.

Verification:

- mocked RPC tests,
- optional live smoke script.

Exit criteria:

- Ethereum integration is useful without entering rollup scope.

### v0.64.0 - Cardano Profile Security Contract

Goal: define `BCX-CARDANO/1` around EUTXO semantics.

Deliverables:

- state UTXO commitment,
- consumed-output rule,
- created-output rule,
- validator version commitment,
- finality policy.

Verification:

- docs checks,
- threat-model update.

Exit criteria:

- Cardano is modeled natively rather than imitating Ethereum.

### v0.65.0 - Cardano Profile Crate

Goal: add dependency-light `bcx-cardano`.

Deliverables:

- Cardano binding vocabulary,
- checkpoint-in-UTXO model,
- finality status mapping,
- mock receipt verification.

Verification:

- `cargo test -p bcx-cardano`,
- mutation tests.

Exit criteria:

- a BCX checkpoint can have both Ethereum and Cardano receipt models.

### v0.66.0 - Cardano Pallas Integration

Goal: add `bcx-cardano-pallas` for offchain Rust integration.

Deliverables:

- transaction evidence builder,
- UTXO evidence parser,
- mock indexer adapter,
- fixture tests.

Verification:

- mocked indexer tests,
- optional local smoke script.

Exit criteria:

- a second deep settlement family proves BCX is not Ethereum-specific.

### v0.67.0 - SCITT Profile Contract

Goal: define how BCX checkpoints can use a transparency service.

Deliverables:

- signed statement mapping,
- transparency receipt model,
- witness finality status,
- privacy notes.

Verification:

- docs checks.

Exit criteria:

- BCX can use transparency infrastructure without becoming SCITT.

### v0.68.0 - OpenTelemetry Profile Contract

Goal: define observability correlation without turning telemetry into proof.

Deliverables:

- statement ID to trace/span mapping,
- non-proof warning,
- export boundary.

Verification:

- docs checks.

Exit criteria:

- telemetry remains operational context, not cryptographic evidence.

### v0.69.0 - Bitcoin Anchoring Profile Contract

Goal: define how Bitcoin fits as anchoring and payment evidence.

Deliverables:

- security contract,
- finality model,
- transaction commitment model,
- output and script commitment rules,
- privacy review,
- dependency plan.

Verification:

- docs checks,
- security review notes.

Exit criteria:

- Bitcoin has a concrete BCX profile contract before any Bitcoin crate is
  implemented.

### v0.70.0 - XRP Payment Evidence Profile Contract

Goal: define how XRP Ledger fits as payment and settlement evidence.

Deliverables:

- security contract,
- ledger finality model,
- transaction evidence model,
- destination tag and memo commitment rules,
- account and memo privacy review,
- dependency plan.

Verification:

- docs checks,
- security review notes.

Exit criteria:

- XRP has a concrete BCX profile contract before any XRP crate is implemented.

### v0.71.0 - Banking Domain Contract

Goal: define banking semantics separate from transport and settlement.

Deliverables:

- mandate model,
- approval model,
- transfer intent vocabulary,
- settlement effect vocabulary,
- compliance evidence hooks.

Verification:

- docs checks,
- threat-model update.

Exit criteria:

- banking meaning is not hard-coded into core BCX.

### v0.72.0 - AI Agent Domain Contract

Goal: define delegated machine-action semantics.

Deliverables:

- agent identity vocabulary,
- model/tool authority,
- human approval markers,
- tool-call effect markers,
- revocation hooks.

Verification:

- docs checks,
- threat-model update.

Exit criteria:

- agent workflows can use BCX without weakening the core.

## Phase 8: Tokenless Local Execution, Proofs, And Witnessing

### v0.73.0 - Tokenless Operation Contract

Goal: make the no-token, no-global-validator BCX operating model normative.

Deliverables:

- tokenless operation contract,
- external security-budget model,
- local admission and rate-limit assumptions,
- public-chain anchoring boundary,
- private witness boundary.

Verification:

- docs checks,
- threat-model update,
- security-controls update.

Exit criteria:

- BCX clearly states that it does not run validators, mint a token, or require
  a blockchain for local proof-carrying operation.

### v0.74.0 - Deterministic State Contract And Resource Model

Goal: define `bcx-state` before implementation, including deterministic
resource limits.

Deliverables:

- deterministic state transition contract,
- state root vocabulary,
- input commitment rules,
- output commitment rules,
- instruction and work metering,
- memory, stack, state-read, and state-write limits,
- transactional rollback on failure,
- deterministic host-function ABI,
- prohibition or strict definition of floats, clocks, randomness, concurrency,
  and external I/O,
- program upgrade and state migration rules,
- cross-architecture execution vectors,
- fail-closed non-determinism rules.

Verification:

- docs checks,
- adversarial determinism review,
- resource model review.

Exit criteria:

- local state execution has a written security and resource contract before any
  state crate exists.

### v0.75.0 - State Crate Skeleton

Goal: add `bcx-state` as a dependency-light core crate.

Deliverables:

- crate scaffold,
- state transition trait,
- state root type,
- transition input and output bounds,
- deterministic test state.

Verification:

- `cargo test -p bcx-state`,
- `cargo test --workspace --no-default-features`.

Exit criteria:

- BCX can model local state transitions without pulling in a runtime, database,
  VM, or blockchain dependency.

### v0.76.0 - Local State Transition Verification

Goal: verify deterministic state transitions as BCX effects.

Deliverables:

- transition verifier,
- pre-state and post-state commitment checks,
- transition effect binding,
- mutation tests.

Verification:

- valid transition fixture tests,
- mutated input, output, and state-root tests.

Exit criteria:

- a local transition can produce a verifiable `Effect` without global
  execution.

### v0.77.0 - Program Identity And Admission

Goal: bind local contract-like programs to explicit BCX admission.

Deliverables:

- program identifier type,
- program version commitment,
- admission policy link,
- deterministic input schema commitment,
- program downgrade rejection tests.

Verification:

- `cargo test -p bcx-state`,
- downgrade and wrong-program fixtures.

Exit criteria:

- a state transition is admitted for one exact program identity and cannot be
  silently reinterpreted as another program.

### v0.78.0 - Effect Proof Inputs

Goal: define common public inputs for local and ZK effect proofs.

Deliverables:

- effect proof input model,
- statement ID input,
- program ID input,
- pre-state and post-state inputs,
- admission ID input,
- canonical input ordering.

Verification:

- proof input fixture tests,
- ordering mutation tests.

Exit criteria:

- every proof provider can consume the same BCX effect-proof input shape.

### v0.79.0 - Threshold Proof Crate

Goal: add `bcx-proof-threshold` for private witness and institutional notary
sets.

Deliverables:

- threshold policy vocabulary,
- signer set commitment,
- threshold count validation,
- witness signature bundle model,
- duplicate signer rejection.

Verification:

- `cargo test -p bcx-proof-threshold`,
- threshold mutation tests.

Exit criteria:

- a checkpoint can be witnessed by a private or federated group without using a
  public blockchain.

### v0.80.0 - ZK Proof Provider Contract

Goal: define ZK proof integration rules before provider crates exist.

Deliverables:

- ZK proof provider trait,
- proof system identifier,
- program verification key commitment,
- public input binding,
- privacy and side-channel notes.

Verification:

- docs checks,
- provider-boundary tests with a deterministic fake provider.

Exit criteria:

- ZK providers can plug into BCX without changing statement, effect, or
  checkpoint semantics.

### v0.81.0 - SP1 Proof Provider Crate

Goal: add `bcx-proof-sp1` as an optional SP1 integration crate.

Deliverables:

- SP1 proof-suite identifier,
- verification key commitment,
- proof byte bounds,
- public input adapter,
- mocked verifier tests.

Verification:

- `cargo test -p bcx-proof-sp1`,
- no root dependency regression.

Exit criteria:

- BCX can verify SP1-produced effect proofs through an optional provider
  boundary.

### v0.82.0 - RISC Zero Proof Provider Crate

Goal: add `bcx-proof-risc0` as an optional RISC Zero integration crate.

Deliverables:

- RISC Zero proof-suite identifier,
- image ID commitment,
- receipt byte bounds,
- public input adapter,
- mocked verifier tests.

Verification:

- `cargo test -p bcx-proof-risc0`,
- no root dependency regression.

Exit criteria:

- BCX can verify RISC Zero-produced effect proofs through an optional provider
  boundary.

### v0.83.0 - Witness Service Contract

Goal: define `bcx-witness` before implementation.

Deliverables:

- witness service security contract,
- checkpoint admission rules,
- timestamp and ordering policy,
- duplicate statement policy,
- privacy and retention policy.

Verification:

- docs checks,
- threat-model update.

Exit criteria:

- non-chain ordering and double-spend prevention have a written BCX contract.

### v0.84.0 - Witness Service Skeleton

Goal: add `bcx-witness` as an optional service crate.

Deliverables:

- service crate scaffold,
- in-memory witness store for tests,
- checkpoint request model,
- threshold proof output hook,
- duplicate checkpoint rejection.

Verification:

- `cargo test -p bcx-witness`,
- no root dependency regression.

Exit criteria:

- BCX has a local/federated witness path that does not depend on Ethereum,
  Cardano, Bitcoin, or XRP.

### v0.85.0 - Local Contract Workflow Fixture

Goal: demonstrate smart-contract-like BCX operation without a blockchain.

Deliverables:

- local program fixture,
- intent fixture,
- admission fixture,
- state transition effect fixture,
- proof bundle fixture,
- explanation output.

Verification:

- local workflow smoke test,
- tamper tests for program ID, admission ID, state roots, and proof inputs.

Exit criteria:

- BCX can show `Intent -> Admission -> Effect` for deterministic local logic
  with verifiable proof evidence and no global VM.

### v0.86.0 - Tokenless Offline Institution Demo

Goal: prove a complete tokenless BCX path between known parties.

Deliverables:

- offline bundle,
- local deterministic effect,
- COSE attestation,
- threshold witness checkpoint,
- WHY explanation bundle,
- replay and duplicate rejection fixture.

Verification:

- end-to-end offline smoke script,
- tamper and replay tests.

Exit criteria:

- BCX demonstrates a free, tokenless, blockchain-independent workflow before
  relying on public-chain settlement profiles.

## Phase 9: Conformance, Modeling, Platform Evidence, And Release Candidates

### v0.87.0 - Conformance Crate

Goal: add `bcx-conformance` for mandatory interoperability vectors.

Deliverables:

- canonical encoding vectors,
- signature vectors,
- replay cases,
- unknown extension cases,
- binding mutation cases,
- checkpoint cases.

Verification:

- conformance test suite.

Exit criteria:

- independent implementations can validate compatibility.

### v0.88.0 - Testkit Crate

Goal: add `bcx-testkit` for deterministic builders and adversarial fixtures.

Deliverables:

- statement builders,
- attestation builders,
- binding builders,
- tamper helpers,
- deterministic keys for tests only.

Verification:

- `cargo test -p bcx-testkit`.

Exit criteria:

- future releases can add tests faster without weakening production crates.

### v0.89.0 - Graph And State Modeling

Goal: model graph and semantic-state invariants under reordering,
duplication, missing parents, and concurrent insertion.

Deliverables:

- property tests for arbitrary graph insertion orders,
- missing-parent resolution model,
- revocation and contradiction state model,
- bounded acyclicity proof plan using Kani or equivalent,
- TLA+/Apalache model for concurrent validation and commit,
- Loom plan for std storage adapters.

Verification:

- property test run,
- bounded model-check smoke where available,
- model artifact review.

Exit criteria:

- the roadmap has executable evidence for acyclicity and semantic-state
  invariants beyond example tests.

### v0.90.0 - Cross-System Consistency Program

Goal: prove statement identity is unchanged across carriers and settlement
profiles.

Deliverables:

- same-statement fixtures for offline, HTTP, database/witness, and Ethereum or
  Cardano paths,
- identical `StatementId` assertion,
- native-binding-only difference assertions,
- reorder, duplicate, loss, partition, rollback, reorg, stale checkpoint, and
  replay simulations,
- truth-status lattice convergence tests.

Verification:

- deterministic cross-profile fixture suite,
- fault simulation smoke.

Exit criteria:

- carriers and settlement backends change evidence and assurance, not the
  underlying statement identity.

### v0.91.0 - Platform Evidence Program

Goal: add runtime and platform evidence beyond ordinary unit tests.

Deliverables:

- Miri run where applicable,
- sanitizer run plan where applicable,
- code coverage report,
- mutation testing plan,
- maximum stack measurement plan for embedded targets,
- allocator budget tests.

Verification:

- local evidence scripts,
- documented unsupported-tool cases with reasons.

Exit criteria:

- release evidence covers memory, stack, allocation, and mutation-resistance
  concerns where the Rust/toolchain support exists.

### v0.92.0 - Mandatory Target Gate

Goal: make advertised platform support evidence-based.

Deliverables:

- required target list,
- optional target list,
- CI or documented local installation path for required targets,
- release gate that fails for missing required targets,
- clear skip behavior only for optional targets.

Verification:

- target availability script,
- release gate dry run.

Exit criteria:

- a green release gate proves required advertised targets compile, instead of
  silently skipping them.

### v0.93.0 - MSRV Matrix Evidence

Goal: prove Rust `1.90.0` through `1.96.1` compatibility.

Deliverables:

- README evidence table update,
- scripted matrix check,
- CI or local reproducibility notes.

Verification:

- documented matrix run.

Exit criteria:

- MSRV support is evidence-based, not aspirational.

### v0.94.0 - no_std And Dependency Audit

Goal: verify the core remains no-std and dependency-light.

Deliverables:

- no-std audit,
- dependency tree audit,
- feature leakage tests,
- alloc/std feature documentation.

Verification:

- `cargo test --workspace --no-default-features`,
- feature checks.

Exit criteria:

- root/core crates do not accidentally pull transport, runtime, database, VM,
  proof-provider, or blockchain dependencies.

### v0.95.0 - Security Specification Freeze

Goal: consolidate implementation-aligned drafts into specs that can be frozen
for release candidates.

Deliverables:

- `BCX-CORE/1` freeze candidate,
- `BCX-CODEC-CBOR/1` freeze candidate,
- `BCX-PROOF-COSE/1` freeze candidate,
- `BCX-STATE/1` freeze candidate,
- profile spec template freeze candidate.

Verification:

- docs checks,
- security review,
- traceability review.

Exit criteria:

- crate behavior is traceable to written protocol requirements and no major
  unstated protocol rule remains.

### v0.96.0 - Interop Demonstration

Goal: show one logical causal graph across multiple profiles.

Deliverables:

- offline bundle,
- local state effect,
- HTTP/Fluxheim evidence,
- threshold witness checkpoint,
- Ethereum or Cardano checkpoint receipt,
- explanation bundle.

Verification:

- end-to-end smoke script,
- tamper test.

Exit criteria:

- BCX is visibly one causal protocol with local, witness, carrier, and
  settlement bindings.

### v0.97.0 - API Freeze Candidate

Goal: identify APIs intended to survive into `1.0.0`.

Deliverables:

- public API audit,
- deprecation list,
- naming cleanup,
- migration notes.

Verification:

- docs build,
- examples build.

Exit criteria:

- no unstable experimental API is accidentally presented as stable.

### v0.98.0 - Release Evidence Freeze

Goal: freeze the release evidence package before release candidates.

Deliverables:

- conformance evidence summary,
- fuzzing evidence summary,
- platform evidence summary,
- crypto conformance evidence summary,
- residual risk register,
- pentest scope draft.

Verification:

- docs checks,
- release evidence review.

Exit criteria:

- release candidates start from a complete evidence package instead of
  discovering missing evidence during tagging.

### v0.99.0 - Release Candidate 1

Goal: cut the first `1.0.0` candidate.

Deliverables:

- release candidate notes,
- full conformance run,
- pentest scope,
- residual risk register.

Verification:

- full local and CI gates,
- pentest after implementation stop.

Exit criteria:

- only release-candidate fixes remain before `1.0.0`.

### v0.100.0 - Release Candidate 2

Goal: resolve RC1 findings and repeat the release audit.

Deliverables:

- fixed RC1 findings,
- updated conformance results,
- updated threat model,
- updated release notes.

Verification:

- full local and CI gates,
- pentest after implementation stop.

Exit criteria:

- no known blocker remains for `1.0.0`.

## v1.0.0 - Production-Ready Foundation

Goal: publish the first serious production-ready BCX protocol foundation.

Required scope:

- stable root `bcx` facade,
- stable core statement, attestation, binding, checkpoint, and explanation
  vocabulary,
- evidence facets and assurance lattice without false total ordering,
- domain-separated commitment suite and registry,
- canonical encoding and conformance vectors,
- sealed statement commitments and derived statement IDs,
- early wire parser and checked aggregate `DecodeBudget`,
- graph-store insertion that preserves acyclicity under late parents and
  duplicate delivery,
- capability verification, trusted time, and atomic replay checks,
- canonical policy records and signed policy-evaluation evidence,
- key resolution, signing provider, verifier provider, exact suite policy, and
  concrete admitted Ed25519 and ML-DSA-65 providers,
- standard COSE proof-envelope boundary,
- hybrid signature constants, coordinator, composite key records, and framed
  signed message representative,
- provider scratch and side-effect contract,
- checkpoint-relative semantic validity engine,
- revocation, conflict, checkpoint, and anti-equivocation roots,
- operational and transparency receipt models,
- privacy, salted or blinded private commitments, AEAD-bound disclosure, and
  selective disclosure hardening,
- deterministic local state transition verification with resource limits,
- local contract workflow fixtures,
- ZK proof provider boundaries for SP1 and RISC Zero,
- replay, delegation, disclosure, and settlement policy checks,
- local/offline WHY proof bundles,
- threshold witness checkpoints for tokenless operation,
- normative profile specification pack,
- parser fuzzing, graph/state modeling, crypto conformance, cross-system
  consistency, platform evidence, and mandatory target gates,
- at least one carrier profile,
- at least one live integration,
- at least two checkpoint witness or settlement receipt models,
- complete threat model, security controls, release notes, and supply-chain
  documentation,
- final pentest PASS for the exact reviewed commit.

Non-goals for `1.0.0`:

- replacing HTTP,
- becoming an Ethereum-only or Cardano-only protocol,
- supporting every blockchain,
- proving internal human motive,
- claiming production readiness for every future integration crate.
