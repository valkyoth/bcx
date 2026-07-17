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
| The tokenless semantic-overlay position could be read as philosophy rather than an enforceable design contract. | Added `v0.52.0 - Tokenless Operation Contract`. |
| `bcx-state` was named as a future crate but did not have implementation milestones. | Added `v0.53.0` through `v0.57.0` for deterministic state contracts, crate skeleton, local transitions, program identity, and effect proof inputs. |
| Smart-contract-like local execution was implied by state and proof crates but not versioned. | Added `v0.65.0 - Local Contract Workflow Fixture` and `v0.66.0 - Tokenless Offline Institution Demo`. |
| COSE was listed as the likely proof foundation but not assigned to a crate milestone. | Added `v0.58.0 - COSE Proof Crate`. |
| Threshold witness proofs were listed as future but not assigned to a concrete release. | Added `v0.59.0 - Threshold Proof Crate`. |
| ZK proof integrations were named but did not have provider-boundary and implementation milestones. | Added `v0.60.0` through `v0.62.0` for the ZK proof contract, SP1 provider crate, and RISC Zero provider crate. |
| `bcx-witness` was named as a service but did not have a profile contract or service milestone. | Added `v0.63.0 - Witness Service Contract` and `v0.64.0 - Witness Service Skeleton`. |
| Blockchain-independent double-spend/order prevention needed an explicit non-chain path. | Added witness, threshold proof, offline demo, and settlement-policy milestones before public-chain profiles become the only examples. |
| The `1.0.0` scope did not explicitly require local deterministic execution and tokenless proof workflows. | Updated the `1.0.0` required scope with deterministic state, COSE, threshold witness, and local contract workflow evidence. |
| ML-DSA-65 signature length is wrong in the current crypto constants. | Added `v0.5.1 - ML-DSA-65 Constant Correction` as the next patch release before more protocol surface is added. |
| Statement IDs and detached verified bytes can be supplied by callers instead of derived from canonical typed statements. | Expanded `v0.14.0` and `v0.22.0`, and added `v0.67.0 - Sealed Statement Commitment API`. |
| Multi-node cycle prevention is optional, non-atomic, and missing-parent resolution can introduce cycles after acceptance. | Added `v0.68.0 - Graph Store Contract`, `v0.69.0 - Atomic Graph Store Skeleton`, and `v0.70.0 - Missing Parent Reconciliation`. |
| `CauseCapsule` and `CausalEdgeSet` remain parallel causal-parent APIs with different graph hooks. | Added `v0.68.0` to normalize compact capsules through the graph-store edge model and define deprecation behavior if needed. |
| `ParentStatus::Missing` can be caller-selected even though availability is verifier-local state. | Added `v0.67.0` and `v0.70.0` to keep canonical edges to parent ID plus relationship and derive availability in stores and explanations. |
| Relationship kinds do not yet enforce semantic cardinality or target-kind constraints. | Added `v0.71.0 - Relationship Semantics Policy`. |
| Duplicate-parent checks are quadratic and canonical parent ordering is not yet defined. | Added `v0.14.0` canonical parent ordering and `v0.68.0` adjacent-duplicate graph-store checks. |
| `WireLimits` is a limits container, not an early parser or aggregate resource budget. | Expanded `v0.12.0` and added `v0.72.0 - Wire Prefix Parser` and `v0.73.0 - Aggregate Decode Budgets`. |
| Variable-length identifiers are constructor-validated but not wire-parsed or canonical-profile validated. | Expanded `v0.13.0` and `v0.72.0` to cover borrowed wire parsing and canonical identifier fixture rejection. |
| Semantic model crates and crypto crates depend on transport-named limits. | Added `v0.74.0 - Core Limit And Budget Split`. |
| Public identifier storage choices need an explicit indexing and zeroization policy. | Added `v0.75.0 - Public Identifier Storage Policy`. |
| Hybrid verification lacks a non-overridable coordinator, framed message representative, composite key binding, and exact suite policy. | Added `v0.76.0` through `v0.79.0` for the hybrid coordinator, composite key records, signed representative, and exact suite enforcement. |
| Provider scratch lifecycle and provider side-effect boundaries are not specified. | Added `v0.80.0 - Provider Scratch And Side-Effect Contract`. |
| COSE proof suites need protected-header, critical-metadata, and `kid` identity rules. | Expanded `v0.58.0` and added `v0.81.0 - COSE Proof Hardening`. |
| Revocation and contradiction are vocabulary, not checkpoint-relative semantic validity. | Expanded `v0.28.0` and added `v0.82.0 - Semantic Validity Engine` and `v0.83.0 - Revocation And Conflict Roots`. |
| WHY bundles need distinct operational receipts and transparency receipts with inclusion, consistency, and non-inclusion proofs. | Expanded `v0.27.0` and added `v0.84.0 - Receipt Model Split` and `v0.85.0 - Transparency Receipt Integration`. |
| Privacy requirements for pseudonymous keys, selective disclosure, and anonymous proof suites need a scheduled design pass. | Added `v0.86.0 - Privacy And Disclosure Hardening`. |
| Carrier profiles need normative schemas, registries, critical-extension behavior, downgrade rules, and finality contracts. | Added `v0.87.0 - Profile Normative Specification Pack`. |
| The wire version must remain draft/experimental until canonical BCX/1 is frozen. | Added `v0.88.0 - Draft Wire Version And Registry Gate`. |
| Parser fuzzing, graph modeling, crypto conformance, cross-system consistency, mandatory target evidence, and formal assurance need concrete releases. | Added `v0.89.0` through `v0.94.0` for parser fuzzing, graph/state modeling, cryptographic conformance, cross-system consistency, platform evidence, and mandatory target gates. |

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

### v0.6.0 - Claim Status And Assurance

Goal: prevent signed claims from being treated as proven reality.

Deliverables:

- claim status enum,
- assurance level enum,
- contradiction marker,
- unknown and redacted evidence markers,
- tests showing declared purpose is not verified truth.

Verification:

- `cargo test -p bcx-model`
- docs with warnings denied

Exit criteria:

- public APIs cannot collapse declared, observed, verified, enforced, and
  finalized claims into one truth state.

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

- one statement shape is ready for canonical encoding in `v0.14.0` and native
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

- one checkpoint is ready for settlement policy in `v0.20.0` and multiple
  receipt models in `v0.56.0`.

## Phase 2: Canonical Codec And Test Vectors

### v0.11.0 - Canonical CBOR Codec Contract

Goal: define deterministic CBOR as the first canonical binary representation.

Deliverables:

- deterministic CBOR security contract,
- canonical map ordering rules,
- integer and byte-string canonicality rules,
- JSON inspection boundary,
- no-std and alloc impact analysis,
- dependency admission notes.

Verification:

- docs checks,
- dependency policy review for the CBOR crate admission.

Exit criteria:

- no signed or hashed object can use ad hoc JSON or Rust memory layout.

### v0.12.0 - Codec Crate Skeleton

Goal: add `bcx-codec` as a no-std crate with errors and limits only.

Deliverables:

- crate scaffold,
- encode/decode error model,
- borrowed decode cursor plan,
- maximum depth,
- maximum item counts,
- maximum byte lengths,
- maximum map entries, array entries, extension counts, proof counts, and
  signature counts,
- malformed-input fail-fast tests.

Verification:

- `cargo test -p bcx-codec`
- `cargo test --workspace --no-default-features`

Exit criteria:

- decoder bounds exist before full decoding exists.

### v0.13.0 - Canonical Identifier Encoding

Goal: produce stable bytes for core identifiers and digests.

Deliverables:

- canonical bytes for IDs,
- canonical bytes for digests,
- borrowed wire parsing for variable-length IDs,
- rejection fixtures for noncanonical identifier encodings,
- cross-platform fixtures,
- fixture regeneration guard.

Verification:

- `cargo test -p bcx-codec`
- fixture diff check

Exit criteria:

- all supported platforms produce the same bytes for identifier fixtures.

### v0.14.0 - Canonical Statement Encoding

Goal: encode the statement envelope and body skeleton deterministically.

Deliverables:

- statement encoder,
- statement decoder,
- unknown-extension behavior,
- canonical parent ordering rules,
- duplicate-parent rejection using adjacent canonical order,
- mutation fixtures,
- statement ID derivation from canonical statement bytes,
- sealed `CanonicalStatementBytes` or `StatementCommitment` producer.

Verification:

- canonical test vectors,
- malformed fixture tests,
- no-default-features pass.

Exit criteria:

- the same logical statement produces the same `StatementId` everywhere.
- caller-supplied statement IDs cannot bypass canonical identity checks.

### v0.15.0 - Canonical Attestation And Binding Encoding

Goal: encode attestations and native bindings deterministically.

Deliverables:

- attestation encoding,
- binding encoding,
- proof byte limits,
- native evidence limits,
- mutation fixtures.

Verification:

- `cargo test -p bcx-codec`
- tamper fixture tests

Exit criteria:

- attestation and binding hashes are stable and mutation-sensitive.

### v0.16.0 - Canonical Checkpoint Encoding

Goal: encode checkpoints and roots deterministically.

Deliverables:

- checkpoint encoding,
- root ordering rules,
- empty-root rejection where required,
- previous-checkpoint validation.

Verification:

- checkpoint vector tests,
- cross-platform fixture tests.

Exit criteria:

- settlement profiles can rely on stable checkpoint IDs.

## Phase 3: Policy, Replay, And Delegation

### v0.17.0 - Validity And Replay Policy

Goal: make replay resistance explicit before any carrier profile exists.

Deliverables:

- validity window,
- expiry/freshness type with overflow-safe remaining-time checks,
- nonce policy,
- issuer sequence policy,
- idempotency policy,
- replay cache trait.

Verification:

- expired statement tests,
- duplicate nonce and sequence tests.

Exit criteria:

- consequential statements cannot be accepted without a replay policy.

### v0.18.0 - Delegation Narrowing

Goal: permit authority delegation that narrows but never silently broadens.

Deliverables:

- delegation body validation,
- child scope checks,
- purpose narrowing,
- time-window narrowing,
- maximum delegation depth.

Verification:

- valid narrowing tests,
- broadening rejection tests.

Exit criteria:

- a child delegation cannot remove parent restrictions.

### v0.19.0 - Disclosure Policy

Goal: encode what can be revealed, redacted, committed, or withheld.

Deliverables:

- disclosure policy vocabulary,
- redaction marker,
- private evidence commitment,
- public evidence marker,
- unknown evidence marker.

Verification:

- redaction validation tests,
- missing evidence tests.

Exit criteria:

- explanations can preserve privacy without pretending to be complete.

### v0.20.0 - Settlement Policy

Goal: describe how checkpoints may be witnessed or settled.

Deliverables:

- primary backend policy,
- primary-with-witnesses policy,
- require-all policy,
- threshold settlement policy,
- normalized finality status.

Verification:

- threshold validation tests,
- finality transition tests.

Exit criteria:

- one checkpoint can express several settlement receipts without requiring a
  blockchain.

## Phase 4: Verification Core

### v0.21.0 - Verifier Provider Boundary

Goal: define crypto verification without choosing production providers.

Deliverables:

- verifier trait,
- proof-suite policy hook,
- narrow primitive-verifier trait shape,
- provider capability metadata,
- provider error model,
- opaque external error contract,
- deterministic test verifier.

Verification:

- `cargo test -p bcx-crypto`
- provider failure tests.

Exit criteria:

- provider crates scheduled from `v0.58.0` through `v0.62.0` can be added
  without changing statement semantics.
- primitive verification providers cannot resolve keys, perform network I/O, or
  choose policy inside the core verification operation.

### v0.22.0 - Statement Verification

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
- wrong-audience tests.

Exit criteria:

- changing any security-relevant statement field invalidates verification.

### v0.23.0 - Attestation Verification

Goal: verify attestations over canonical statement IDs.

Deliverables:

- proof-suite dispatch,
- issuer/key checks,
- multi-attestation validation,
- unknown proof-suite handling.

Verification:

- valid and invalid proof fixtures,
- unknown-suite tests.

Exit criteria:

- attestations are verifiable without native profile dependencies.

### v0.24.0 - Binding Verification

Goal: verify that native binding evidence commits to the intended statement.

Deliverables:

- binding verifier trait,
- profile ID checks,
- native commitment checks,
- mutated evidence tests.

Verification:

- `cargo test --workspace`
- binding mutation fixtures.

Exit criteria:

- profiles can plug in native verification without changing core logic.

### v0.25.0 - Checkpoint Verification

Goal: verify checkpoint membership and continuity.

Deliverables:

- membership proof model,
- previous-checkpoint check,
- root consistency checks,
- missing-root behavior.

Verification:

- checkpoint fixture tests,
- continuity tests.

Exit criteria:

- a checkpoint can be verified independently of where it is settled.

## Phase 5: Explanation And Offline Use

### v0.26.0 - Explain Crate Skeleton

Goal: add `bcx-explain` for bounded WHY semantics.

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

- `cargo test -p bcx-explain`
- oversized query tests.

Exit criteria:

- no WHY query can request unbounded work.

### v0.27.0 - Explanation Bundle

Goal: return bounded proof bundles for local/offline verification.

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
- missing, withheld, redacted, truncated, stale, and contradicted markers,
- disclosure map binding fields to plaintext, ciphertext, commitments, or
  predicate proofs,
- final bundle commitment or signature.

Verification:

- bundle validation tests,
- missing-parent fixture tests.

Exit criteria:

- an offline verifier can see what is proven, missing, redacted, or unknown.

### v0.28.0 - Contradiction Handling

Goal: preserve conflicting claims without overwriting them.

Deliverables:

- contradiction statement checks,
- conflicting claim relation,
- target-kind and authority checks,
- checkpoint-relative conflict status,
- non-invalidation rule for historical authentic evidence,
- assurance summary,
- explanation output for conflicts.

Verification:

- conflicting fixture tests,
- no-overwrite tests.

Exit criteria:

- BCX can represent "A says sent" and "B says not received" at the same time.

### v0.29.0 - Offline Profile

Goal: add `bcx-offline` for air-gapped bundles.

Deliverables:

- offline bundle profile,
- bundle manifest,
- evidence file commitments,
- detached private evidence references.

Verification:

- `cargo test -p bcx-offline`
- tampered bundle tests.

Exit criteria:

- BCX can work without HTTP, Fluxheim, or any blockchain.

### v0.30.0 - CLI Skeleton

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

## Phase 6: HTTP And Fluxheim

### v0.31.0 - HTTP Profile Security Contract

Goal: define `BCX-HTTP/1` before implementation.

Deliverables:

- attached mode contract,
- encapsulated mode contract,
- committed HTTP components,
- intermediary mutation rules,
- replay rules,
- limits.

Verification:

- docs checks,
- threat-model update.

Exit criteria:

- implementation cannot start without a precise HTTP security contract.

### v0.32.0 - HTTP Profile Crate

Goal: add dependency-light `bcx-http`.

Deliverables:

- profile identifiers,
- header names,
- attached-mode commitment builder,
- encapsulated-mode envelope model.

Verification:

- `cargo test -p bcx-http`
- mutation tests for committed components.

Exit criteria:

- HTTP commitments can be verified without Hyper or Axum dependencies.

### v0.33.0 - HTTP Hyper Integration

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

### v0.34.0 - Fluxheim Profile Contract

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

### v0.35.0 - Fluxheim Integration Skeleton

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

### v0.36.0 - Two-Fluxheim Demonstration

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

## Phase 7: First Settlement Profiles

### v0.37.0 - Ethereum Profile Security Contract

Goal: define what `BCX-ETHEREUM/1` commits and can prove.

Deliverables:

- chain ID commitment,
- contract address commitment,
- calldata digest commitment,
- value commitment,
- sender or authorization commitment,
- expiry and nullifier rules,
- finality policy.

Verification:

- docs checks,
- threat-model update.

Exit criteria:

- Ethereum implementation cannot start without a precise security contract.

### v0.38.0 - Ethereum Profile Crate

Goal: add dependency-light `bcx-ethereum`.

Deliverables:

- Ethereum binding vocabulary,
- checkpoint anchoring model,
- finality status mapping,
- mock receipt verification.

Verification:

- `cargo test -p bcx-ethereum`
- mutation tests.

Exit criteria:

- Ethereum can bind and settle checkpoints conceptually without Alloy yet.

### v0.39.0 - Ethereum Alloy Integration

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

### v0.40.0 - Cardano Profile Security Contract

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

### v0.41.0 - Cardano Profile Crate

Goal: add dependency-light `bcx-cardano`.

Deliverables:

- Cardano binding vocabulary,
- checkpoint-in-UTXO model,
- finality status mapping,
- mock receipt verification.

Verification:

- `cargo test -p bcx-cardano`
- mutation tests.

Exit criteria:

- a BCX checkpoint can have both Ethereum and Cardano receipt models.

### v0.42.0 - Cardano Pallas Integration

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

## Phase 8: Additional Bindings And Standards

### v0.43.0 - SCITT Profile Contract

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

### v0.44.0 - OpenTelemetry Profile Contract

Goal: define observability correlation without turning telemetry into proof.

Deliverables:

- statement ID to trace/span mapping,
- non-proof warning,
- export boundary.

Verification:

- docs checks.

Exit criteria:

- telemetry remains operational context, not cryptographic evidence.

### v0.45.0 - Bitcoin Anchoring Profile Contract

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

### v0.46.0 - XRP Payment Evidence Profile Contract

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

## Phase 9: Conformance And Registry

### v0.47.0 - Registry Crate

Goal: add `bcx-registry` for profile, type, algorithm, and extension IDs.

Deliverables:

- registry ID model,
- reserved ranges,
- experimental ranges,
- unknown-extension rules.

Verification:

- registry validation tests.

Exit criteria:

- ecosystem crates can avoid ID collisions.

### v0.48.0 - Conformance Crate

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

### v0.49.0 - Testkit Crate

Goal: add `bcx-testkit` for deterministic builders and adversarial fixtures.

Deliverables:

- statement builders,
- attestation builders,
- binding builders,
- tamper helpers,
- deterministic keys for tests only.

Verification:

- `cargo test -p bcx-testkit`

Exit criteria:

- future releases can add tests faster without weakening production crates.

## Phase 10: Domain Profiles

### v0.50.0 - Banking Domain Contract

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

### v0.51.0 - AI Agent Domain Contract

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

## Phase 11: Tokenless Local Execution And Proofs

### v0.52.0 - Tokenless Operation Contract

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

### v0.53.0 - Deterministic State Contract

Goal: define `bcx-state` before implementation.

Deliverables:

- deterministic state transition contract,
- state root vocabulary,
- input commitment rules,
- output commitment rules,
- fail-closed non-determinism rules.

Verification:

- docs checks,
- adversarial determinism review.

Exit criteria:

- local state execution has a written security contract before any state crate
  exists.

### v0.54.0 - State Crate Skeleton

Goal: add `bcx-state` as a dependency-light core crate.

Deliverables:

- crate scaffold,
- state transition trait,
- state root type,
- transition input and output bounds,
- deterministic test state.

Verification:

- `cargo test -p bcx-state`
- `cargo test --workspace --no-default-features`

Exit criteria:

- BCX can model local state transitions without pulling in a runtime, database,
  VM, or blockchain dependency.

### v0.55.0 - Local State Transition Verification

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

### v0.56.0 - Program Identity And Admission

Goal: bind local contract-like programs to explicit BCX admission.

Deliverables:

- program identifier type,
- program version commitment,
- admission policy link,
- deterministic input schema commitment,
- program downgrade rejection tests.

Verification:

- `cargo test -p bcx-state`
- downgrade and wrong-program fixtures.

Exit criteria:

- a state transition is admitted for one exact program identity and cannot be
  silently reinterpreted as another program.

### v0.57.0 - Effect Proof Inputs

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

### v0.58.0 - COSE Proof Crate

Goal: add `bcx-proof-cose` as the first standard proof-suite crate.

Deliverables:

- COSE proof-suite identifiers,
- COSE signature envelope binding,
- detached payload verification boundary,
- key identifier binding,
- protected-header algorithm binding,
- critical-header behavior,
- duplicate-header rejection,
- external authenticated data binding,
- negative fixtures.

Verification:

- `cargo test -p bcx-proof-cose`
- malformed COSE fixture tests.

Exit criteria:

- BCX has a standard, deterministic signing proof suite before `1.0.0`.
- COSE `kid` is treated as a lookup hint bound by policy, not as a globally
  unique security identity.

### v0.59.0 - Threshold Proof Crate

Goal: add `bcx-proof-threshold` for private witness and institutional notary
sets.

Deliverables:

- threshold policy vocabulary,
- signer set commitment,
- threshold count validation,
- witness signature bundle model,
- duplicate signer rejection.

Verification:

- `cargo test -p bcx-proof-threshold`
- threshold mutation tests.

Exit criteria:

- a checkpoint can be witnessed by a private or federated group without using a
  public blockchain.

### v0.60.0 - ZK Proof Provider Contract

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

### v0.61.0 - SP1 Proof Provider Crate

Goal: add `bcx-proof-sp1` as an optional SP1 integration crate.

Deliverables:

- SP1 proof-suite identifier,
- verification key commitment,
- proof byte bounds,
- public input adapter,
- mocked verifier tests.

Verification:

- `cargo test -p bcx-proof-sp1`
- no root dependency regression.

Exit criteria:

- BCX can verify SP1-produced effect proofs through an optional provider
  boundary.

### v0.62.0 - RISC Zero Proof Provider Crate

Goal: add `bcx-proof-risc0` as an optional RISC Zero integration crate.

Deliverables:

- RISC Zero proof-suite identifier,
- image ID commitment,
- receipt byte bounds,
- public input adapter,
- mocked verifier tests.

Verification:

- `cargo test -p bcx-proof-risc0`
- no root dependency regression.

Exit criteria:

- BCX can verify RISC Zero-produced effect proofs through an optional provider
  boundary.

### v0.63.0 - Witness Service Contract

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

### v0.64.0 - Witness Service Skeleton

Goal: add `bcx-witness` as an optional service crate.

Deliverables:

- service crate scaffold,
- in-memory witness store for tests,
- checkpoint request model,
- threshold proof output hook,
- duplicate checkpoint rejection.

Verification:

- `cargo test -p bcx-witness`
- no root dependency regression.

Exit criteria:

- BCX has a local/federated witness path that does not depend on Ethereum,
  Cardano, Bitcoin, or XRP.

### v0.65.0 - Local Contract Workflow Fixture

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

### v0.66.0 - Tokenless Offline Institution Demo

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

## Phase 12: Protocol Assurance Gap Closure

### v0.67.0 - Sealed Statement Commitment API

Goal: prevent callers from pairing typed statements with unrelated verified
bytes or caller-chosen IDs.

Deliverables:

- sealed `CanonicalStatementBytes` or `StatementCommitment`,
- statement ID derivation only from canonical typed statements,
- detached signature verification that consumes sealed commitments,
- substitution tests where object A's signature is paired with object B,
- availability state kept outside canonical edge semantics.

Verification:

- `cargo test --workspace`
- canonical identity mutation fixtures.

Exit criteria:

- BCX verification accepts only canonical commitments produced by the codec
  path scheduled in `v0.14.0`.

### v0.68.0 - Graph Store Contract

Goal: define graph insertion as one atomic causal operation.

Deliverables:

- `GraphStore` contract,
- `insert_checked(event, parents)` semantics,
- reachability check requirements,
- compact `CauseCapsule` normalization into causal edges,
- adjacent duplicate checks over canonical parent order,
- public identifier indexing policy hooks.

Verification:

- contract tests with deterministic in-memory graph fixtures,
- duplicate, self-parent, and two-node cycle fixtures.

Exit criteria:

- graph acceptance is specified as validation plus insertion in one operation,
  not as a boolean pre-check against a stale snapshot.

### v0.69.0 - Atomic Graph Store Skeleton

Goal: add the first no-std graph-store implementation for tests and core
verification.

Deliverables:

- crate or module skeleton,
- in-memory bounded graph store,
- atomic insert API,
- iterative reachability implementation,
- no recursion in traversal,
- explicit node, edge, and depth limits.

Verification:

- arbitrary insertion-order tests,
- cycle and duplicate-delivery tests,
- no-default-features workspace pass.

Exit criteria:

- every accepted insertion leaves the bounded in-memory graph acyclic.

### v0.70.0 - Missing Parent Reconciliation

Goal: make missing-parent resolution preserve graph integrity.

Deliverables:

- unresolved edge table,
- parent arrival reconciliation operation,
- recheck of cycles when missing parents arrive,
- deterministic missing-node explanation marker,
- incomplete graph status.

Verification:

- mutually missing parent fixtures,
- late-parent cycle rejection tests,
- explanation bundle tests for incomplete graphs.

Exit criteria:

- two objects accepted while unresolved cannot form an accepted cycle when both
  become available.

### v0.71.0 - Relationship Semantics Policy

Goal: enforce semantic constraints that require resolved parent metadata or
profile policy.

Deliverables:

- relationship cardinality rules,
- target-kind checks for delegation, retry, scheduling, derivation, and joins,
- profile hook for domain-specific relationship constraints,
- fail-closed behavior for missing required parent metadata.

Verification:

- one-parent `JoinedFrom` rejection where policy requires several parents,
- multiple `RetryOf` rejection where policy requires one retry source,
- wrong-target-kind fixtures.

Exit criteria:

- relationship kinds mean more than labels once the verifier has enough parent
  context.

### v0.72.0 - Wire Prefix Parser

Goal: reject invalid wire data before allocation, hashing, key lookup, or
cryptographic verification.

Deliverables:

- fixed wire prefix,
- magic value,
- wire object type,
- draft/version fields,
- flags and critical-flag behavior,
- exact frame-length check,
- trailing-byte rejection.

Verification:

- malformed prefix fixtures,
- unsupported version, type, and critical-flag fixtures,
- short, long, and trailing-byte fixtures.

Exit criteria:

- untrusted input has a fail-fast parser boundary before high-cost work.

### v0.73.0 - Aggregate Decode Budgets

Goal: replace independent limits with a total resource budget model.

Deliverables:

- `DecodeBudget`,
- checked debiting for bytes, nesting, map entries, array entries, extensions,
  proofs, signatures, and disclosed fields,
- decompressed-size budget slot,
- cumulative referenced-byte budget,
- work-unit budget before cryptographic dispatch.

Verification:

- budget exhaustion fixtures,
- checked-arithmetic overflow tests,
- nested and aggregate amplification tests.

Exit criteria:

- an input cannot stay under every per-field maximum while exceeding total
  parser or verifier resources.

### v0.74.0 - Core Limit And Budget Split

Goal: remove transport-named limit coupling from model and crypto crates.

Deliverables:

- general graph limits in core or policy,
- verification budgets in core or policy,
- byte-decoding limits kept in wire or codec,
- migration of model validation from `bcx-wire::WireLimits`,
- migration of crypto verification limits from `bcx-wire::WireLimits`.

Verification:

- `cargo tree -p bcx-model`,
- `cargo tree -p bcx-crypto`,
- workspace tests and no-default-features tests.

Exit criteria:

- semantic model and cryptographic verification do not depend on a
  transport-named crate for non-wire budgets.

### v0.75.0 - Public Identifier Storage Policy

Goal: decide which public commitments are indexable and which buffers require
zeroization.

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

### v0.76.0 - Hybrid Verification Coordinator

Goal: make hybrid signature acceptance non-overridable by provider traits.

Deliverables:

- BCX-owned hybrid coordinator,
- separate primitive provider traits for Ed25519 and ML-DSA-65,
- exact suite dispatch,
- coordinator-owned AND-combination,
- public-data early-failure policy,
- invalid-input cost budget before PQ dispatch.

Verification:

- hybrid truth-table tests,
- stripped-component tests,
- provider override rejection tests,
- hostile invalid-input benchmark.

Exit criteria:

- hybrid acceptance cannot be redefined by one provider implementation.

### v0.77.0 - Composite Key Record

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

### v0.78.0 - Signed Message Representative

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

### v0.79.0 - Exact Suite Policy Enforcement

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

### v0.80.0 - Provider Scratch And Side-Effect Contract

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

### v0.81.0 - COSE Proof Hardening

Goal: align COSE proof handling with protected metadata and critical-header
requirements.

Deliverables:

- protected algorithm binding,
- external authenticated data binding,
- duplicate protected and unprotected header rejection,
- critical-header registry behavior,
- `kid` privacy and lookup-hint guidance.

Verification:

- COSE malformed header fixtures,
- critical-header fixtures,
- wrong external authenticated data fixtures.

Exit criteria:

- COSE proof verification fails closed on metadata ambiguity.

### v0.82.0 - Semantic Validity Engine

Goal: derive validity as a checkpoint-relative result instead of embedding
absolute invalidation into immutable statements.

Deliverables:

- semantic validity result type,
- dimensions for cryptographic validity, semantic support, revocation,
  conflict, completeness, and assurance,
- required-dependency propagation,
- deny-overrides rule for consequential operations,
- fail-closed behavior for missing required authorization evidence.

Verification:

- revocation and contradiction fixtures,
- required versus observational dependency fixtures,
- deny-overrides tests.

Exit criteria:

- BCX can distinguish authentic history from usable authority.

### v0.83.0 - Revocation And Conflict Roots

Goal: make semantic validity non-bypassable across policy and checkpoint
changes.

Deliverables:

- signed policy epoch input,
- revocation root input,
- conflict root input,
- verification cache key including roots,
- cache invalidation on root change,
- authenticated non-inclusion proof requirement before claiming not revoked.

Verification:

- stale cache fixtures,
- non-inclusion proof fixtures,
- root-change invalidation tests.

Exit criteria:

- verification results cannot outlive the revocation and conflict roots they
  were evaluated against.

### v0.84.0 - Receipt Model Split

Goal: separate operational receipts from transparency receipts.

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

### v0.85.0 - Transparency Receipt Integration

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

### v0.86.0 - Privacy And Disclosure Hardening

Goal: schedule wire-level privacy controls beyond debug redaction.

Deliverables:

- audience-specific pseudonymous key guidance,
- stable global key ID avoidance policy,
- selective disclosure binding,
- encrypted field disclosure contract,
- anonymous credential or ZK proof-suite admission hook,
- privacy review for issuer, realm, profile, and key metadata.

Verification:

- disclosure map tests,
- cross-audience linkability review,
- wrong-audience encrypted disclosure fixtures.

Exit criteria:

- privacy-sensitive deployments have protocol hooks for minimizing linkability
  and over-disclosure.

### v0.87.0 - Profile Normative Specification Pack

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

### v0.88.0 - Draft Wire Version And Registry Gate

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

### v0.89.0 - Parser Fuzzing Program

Goal: fuzz every untrusted parser before network endpoints are exposed.

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
- corpus regression tests.

Exit criteria:

- every parser has a reproducible malformed-input corpus and a fuzz entry
  point.

### v0.90.0 - Graph And State Modeling

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

### v0.91.0 - Cryptographic Conformance Program

Goal: prove primitive and hybrid verification against external vectors and
adversarial suite mutations.

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

### v0.92.0 - Cross-System Consistency Program

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

### v0.93.0 - Platform Evidence Program

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

### v0.94.0 - Mandatory Target Gate

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

## Phase 13: Hardening Toward 1.0

### v0.95.0 - Fuzzing And Negative Corpus

Goal: expand malformed input and mutation testing.

Deliverables:

- fuzz harness plan,
- malformed corpus,
- decode and verification negative corpus,
- CI integration plan,
- deterministic corpus regression tests.

Verification:

- local fuzz smoke,
- corpus regression tests.

Exit criteria:

- known malformed cases fail closed.

### v0.96.0 - MSRV Matrix Evidence

Goal: prove Rust `1.90.0` through `1.96.1` compatibility.

Deliverables:

- README evidence table update,
- scripted matrix check,
- CI or local reproducibility notes.

Verification:

- documented matrix run.

Exit criteria:

- MSRV support is evidence-based, not aspirational.

### v0.97.0 - no_std And Dependency Audit

Goal: verify the core remains no-std and dependency-light.

Deliverables:

- no-std audit,
- dependency tree audit,
- feature leakage tests,
- alloc/std feature documentation.

Verification:

- `cargo test --workspace --no-default-features`
- feature checks.

Exit criteria:

- root/core crates do not accidentally pull transport, runtime, database, VM,
  proof-provider, or blockchain dependencies.

### v0.98.0 - Security Specification Draft

Goal: turn implementation docs into normative specs.

Deliverables:

- `BCX-CORE/1` draft,
- `BCX-CODEC-CBOR/1` draft,
- `BCX-PROOF-COSE/1` draft,
- `BCX-STATE/1` draft,
- profile spec template.

Verification:

- docs checks,
- security review.

Exit criteria:

- crate behavior is traceable to written protocol requirements.

### v0.99.0 - Interop Demonstration

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

### v0.100.0 - API Freeze Candidate

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

### v0.101.0 - Release Candidate 1

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

### v0.102.0 - Release Candidate 2

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
- canonical encoding and conformance vectors,
- sealed statement commitments and derived statement IDs,
- early wire parser and aggregate decode budgets,
- graph-store insertion that preserves acyclicity under late parents and
  duplicate delivery,
- standard COSE proof-suite boundary,
- hybrid signature constants, coordinator, composite key records, and framed
  signed message representative,
- provider scratch and side-effect contract,
- checkpoint-relative semantic validity engine,
- revocation and conflict roots with authenticated non-inclusion behavior,
- operational and transparency receipt models,
- privacy and selective disclosure hardening,
- deterministic local state transition verification,
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
