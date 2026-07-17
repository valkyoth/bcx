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
| Parser fuzzing must precede untrusted network endpoints. | Added core parser tests to `v0.13.0`, core object and bundle fuzzing to `v0.47.3`, and carrier parser fuzzing in `v0.54.0 - Carrier Parser Fuzzing Program` before HTTP implementation starts. |
| Cryptographic conformance must precede reliance on COSE and hybrid provider surfaces. | Added `v0.37.0 - Cryptographic Conformance Program` before statement and attestation verification are used by profiles. |
| Normative security specifications should grow alongside implementation rather than appear only at the end. | Added living specification work from `v0.11.0` onward, renamed `v0.50.0` to `Core And Codec Specification Consolidation`, and kept `v0.95.0 - Security Specification Freeze`. |
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
| Offline and CLI parsing depended on core parser safety that was scheduled too late. | Added core prefix and streaming parser safety to `v0.13.0`, core object and bundle fuzzing to `v0.47.3`, made `v0.53.0` carrier framing-specific, and made `v0.54.0` carrier parser fuzzing. |
| Basic graph and parser assurance was too late for persistent graph and carrier work. | Added early property, acyclicity, concurrent insertion, streaming chunk-boundary, prefix-before-allocation, and budget monotonicity requirements to `v0.13.0`, `v0.17.0`, `v0.20.0`, and `v0.21.0`. |
| Security controls documentation needs to mirror atomic replay, orphan DoS, signer entropy, policy evaluation, trust snapshots, and checkpoint equivocation. | Added security-controls update requirements to the responsible milestones: `v0.23.0`, `v0.26.1`, `v0.27.0`, `v0.29.0`, and `v0.40.0`. |
| Explanation bundle schema still referenced contradiction and privacy work scheduled after it. | Split the bundle work: `v0.45.0` is a skeleton bundle and `v0.47.1 - Final Explanation Bundle Closure` finalizes the schema after `v0.46.0` contradiction and `v0.47.0` privacy hardening. |
| Root completion referenced non-inclusion fixtures before the concrete transparency proof model. | Limited `v0.40.0` to the non-inclusion interface and moved concrete non-inclusion fixtures to `v0.42.0`. |
| Graph-store atomicity and pruning safety were underspecified. | Expanded `v0.19.0` and `v0.20.0` with linearization, failure atomicity, isolation, conflict/retry, crash recovery, transaction/CAS equivalence, pruning boundaries, authenticated ancestry summaries, tombstones as existence markers only, and cycle-through-pruned-ancestor tests. |
| Parent duplicate rejection could be read as rejecting only identical `(parent, relationship)` pairs. | Updated `v0.15.0` and `v0.19.0` so parent-ID uniqueness is an absolute core invariant; several semantic roles for one parent must be represented as one canonical edge with a role set. |
| Verification work needed a concrete budget matching `DecodeBudget`. | Expanded `v0.17.0` with checked `VerificationBudget`, versioned `VerificationCostSchedule`, `CostScheduleId`, registry-defined suite/profile costs, and identical debit tests across providers for the same suite. |
| Signed policy decisions needed assurance modes that distinguish attestation from reproducibility and proof. | Expanded `v0.26.1` with attested, reproducible, and proven policy-decision modes plus executable-policy metering and sandbox requirements. |
| Self-contained WHY bundles needed a formal closure algorithm and trust-anchor boundary. | Added those requirements to `v0.47.1 - Final Explanation Bundle Closure`. |
| Provider conformance required two independent implementations but only one provider per mandatory algorithm was scheduled. | Added `v0.36.3 - Provider Oracle And SLH-DSA Scope Decision` before cryptographic conformance. |
| Provider scratch handling needed panic, abort, internal-copy, and HSM limitations. | Expanded `v0.34.0 - Provider Scratch And Side-Effect Contract`. |
| Implementation-plan language still used claim-status wording. | Updated `docs/IMPLEMENTATION_PLAN.md` to use evidence facets and checkpoint-relative assurance language. |
| Hiding commitments and AEAD nonces needed exact construction rules. | Tightened `v0.26.0` and `v0.47.0` to require suite IDs, domain-separated preimages, minimum blinding entropy, no blinding reuse, canonical openings, context and audience binding, explicit assumptions, and non-circular AEAD nonce derivation. |
| Differential CBOR fuzzing must compare canonical acceptance and original-byte preservation, not only decoded values. | Added that requirement to `v0.54.0 - Carrier Parser Fuzzing Program`. |
| Offline profile prerequisites were scheduled after the offline profile itself. | Added `v0.47.2 - Pre-Offline Profile Specification Gate`, `v0.47.3 - Core Object And Bundle Parser Fuzzing`, and `v0.47.4 - Pre-Offline no_std And Zero-Copy Evidence` before `v0.48.0`. |
| no-std feature tiers and zero-copy benchmarks need evidence before profile work and a refresh before carriers. | Added `v0.47.4` before the offline profile, kept `v0.54.1 - Pre-Carrier no_std Feature Tiers And Zero-Copy Evidence` before HTTP/profile implementation, and kept `v0.94.0` as the final no-std audit. |
| Threshold witness signer-set rotation and epoch binding were underspecified. | Expanded `v0.79.0 - Threshold Proof Crate` with signer-set epoch, membership, rotation, and threshold-change binding. |
| Cryptographic conformance was described as planning rather than executable proof. | Updated `v0.37.0` to require executable imported-vector harnesses, pinned vector provenance and hashes, complete KAT execution, provider/oracle differential tests, cross-product signing and verification tests, negative-vector execution, and reproducible reports. |
| Multi-attestation duplicate handling could reject valid key rollover or inflate thresholds. | Updated `v0.35.0` to reject exact duplicate proofs, count stable trust principals once per role, retain rollover evidence without threshold inflation, and define key-rotation overlap behavior. |
| Pruned graph proofs needed enough ancestry data to reject cycles. | Expanded `v0.19.0` and `v0.20.0` with authenticated reachability summaries, unsafe finalized-epoch edge rejection, retained transitive metadata, and cycle-through-pruned-ancestor tests. |
| Conformance and testkit crates were too late for shared vectors and adversarial fixtures. | Added `v0.16.1 - Conformance Vector Scaffold` and `v0.16.2 - Testkit Fixture Scaffold`; `v0.87.0` and `v0.88.0` now complete those crates. |
| Resource exhaustion must not be cached or reported as semantic invalidity. | Added `v0.17.1 - Verification Outcome And Receipt Model` with valid, invalid, and indeterminate outcomes, receipt completion state, cost schedule recording, and conformance vectors. |
| Threshold witness finality needs Byzantine quorum safety, not only `t-of-n` counting. | Expanded `v0.79.0 - Threshold Proof Crate` with fault assumptions, quorum-intersection rules, equivocation evidence, joint-consensus signer-set transitions, and conflict-finalization tests. |
| Composite key lifecycle needed atomic component validity rules. | Expanded `v0.31.0 - Composite Key Record` with component epoch binding, partial rotation semantics, checkpoint-relative historical verification, cross-suite reuse rejection, and fail-closed downgrade recovery. |
| Production signing providers needed side-channel and entropy admission rules. | Expanded `v0.29.0`, `v0.36.1`, and `v0.36.2` with provider side-channel declarations, zeroization, entropy health, hedged-signing fallback policy, fault-injection behavior, platform capability declarations, and external-provider guarantee boundaries. |
| Graph semantics needed an early executable reference model before profile and persistent adapter work. | Added `v0.20.1 - Minimal Graph Reference Model` for atomic insertion, competing inserts, missing-parent promotion, pruning/finalization boundaries, duplicate delivery, and cycles through pruned ancestors. |
| Security controls needed to mirror the new outcome, budget, commitment, hybrid, and quorum rules. | Updated `docs/security-controls.md` and attached ongoing security-control traceability to `v0.17.1`, `v0.31.0`, `v0.47.0`, and `v0.79.0`. |
| Unsupported-suite outcomes could become a downgrade escape hatch. | Tightened `v0.17.1` to distinguish unknown critical suites, policy-forbidden suites, locally unsupported recognized suites, and temporarily unavailable providers. |
| Verification receipts needed trust semantics. | Added receipt trust rules to `v0.17.1` and tied receipt classification into `v0.41.0`. Sender-provided receipts cannot suppress required local verification unless policy admits the receipt signer role. |
| Historical key validity cannot be selected by an unsigned claimed timestamp. | Expanded `v0.27.0` and `v0.31.0` with authenticated evaluation points such as admission receipts, checkpoint sequence/time, transparency inclusion, or profile trusted-clock evidence. |
| Threshold quorum safety needed exact mathematics. | Expanded `v0.79.0` with supported fault model, exact quorum formula, weighted-quorum policy, overflow-safe arithmetic, and policy-time rejection of impossible configurations. |
| Provider side-channel wording could admit variable-time signing into high-assurance profiles. | Added provider assurance classes in `v0.28.1` and required high-assurance profiles to admit only constant-time software or appropriately isolated hardware signing providers. |
| Invalid-result caching needed a complete semantic cache key. | Added cache-key requirements to `v0.17.1` covering statement, policy, trust, revocation, conflict, checkpoint, suite, and profile roots. |
| Replay stores could be poisoned before authentication. | Expanded `v0.23.0 - Validity And Atomic Replay Policy` with authenticate-before-commit ordering, bounded reservation semantics, advisory early duplicate checks, and invalid-signature/crash fixtures. |
| Provider threat claims needed a precise trust boundary. | Expanded `v0.28.1 - Provider Assurance Classes` so admitted primitive providers are part of the trusted computing base; provider crashes, format errors, resource abuse, and capability misreporting are defended, but arbitrary false cryptographic success from an admitted compromised provider is a trusted-boundary failure through `v1.0.0`. |
| Verification cacheability needed per-outcome rules. | Expanded `v0.17.1 - Verification Outcome And Receipt Model` with a cacheability matrix for decoding, crypto, policy, revocation, validity time, replay, missing evidence, provider availability, resource exhaustion, capability, delegation, and authority outcomes. |
| Verification receipts needed a nonrecursive signature domain. | Expanded `v0.41.0 - Receipt Model Split` with a distinct receipt-signature domain and direct receipt verification path that does not recursively require another verification receipt. |
| Missing-parent quotas could be bypassed with unauthenticated issuer claims. | Expanded `v0.21.0 - Missing Parent Reconciliation` with unauthenticated global/source quotas and authenticated per-issuer quotas only after issuer authentication. |
| Repeated replay/provider/cache/receipt/orphan/quorum gap review needed one traceable closure line. | Confirmed the closure remains versioned in `v0.17.1`, `v0.21.0`, `v0.23.0`, `v0.28.1`, `v0.41.0`, and `v0.79.0`; no extra milestone is introduced for the duplicate review. |

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
- explicit absence/completeness and redacted evidence markers,
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
- adjacent duplicate-parent rejection by parent ID, including the same parent
  ID under different relationship kinds,
- canonical role-set encoding for a parent that carries several semantic roles,
- sorted and duplicate-free role-set encoding,
- relationship and role compatibility validation,
- statement ID derivation from canonical statement bytes,
- sealed `CanonicalStatementBytes` or `StatementCommitment` producer,
- rule that `StatementId` is excluded from its own hash preimage,
- rule that constructors do not accept a caller-derived ID,
- exclusion of attestations, native bindings, local availability, and transport
  metadata from the statement preimage,
- inclusion of schema version and security-relevant extensions,
- canonical edge semantics limited to parent ID, relationship, and optional
  role set; profile hooks may narrow this but cannot weaken parent-ID
  uniqueness,
- mutation fixtures.

Verification:

- canonical test vectors,
- malformed fixture tests,
- no-default-features pass,
- detached-byte substitution fixtures.

Exit criteria:

- the same logical statement produces the same `StatementId` everywhere,
- caller-supplied statement IDs cannot bypass canonical identity checks,
- one canonical statement cannot contain two edges to the same parent ID.

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

### v0.16.1 - Conformance Vector Scaffold

Goal: establish shared interoperability vector formats immediately after the
first canonical encoding milestones.

Deliverables:

- `bcx-conformance` crate skeleton,
- vector manifest schema,
- vector provenance fields,
- expected version and digest fields,
- canonical statement, attestation, binding, and checkpoint vector directories,
- verification outcome vector directories,
- cost-schedule and resource-exhaustion vector directories,
- fixture regeneration guard,
- pass/fail report format.

Verification:

- `cargo test -p bcx-conformance`,
- manifest digest mismatch tests,
- fixture regeneration check.

Exit criteria:

- every new canonical object vector has one shared home before profile-specific
  vector formats appear.

### v0.16.2 - Testkit Fixture Scaffold

Goal: provide shared adversarial fixtures before graph, parser, crypto, and
bundle milestones each create local copies.

Deliverables:

- `bcx-testkit` crate skeleton,
- deterministic test-only key material,
- conspicuous test-only key and signer types,
- compile-time guards preventing deterministic test keys from being enabled
  through production facade features,
- statement and attestation fixture builders,
- malformed byte fixture helpers,
- graph edge fixture helpers,
- parser corpus seed helpers,
- bundle dependency fixture helpers.

Verification:

- `cargo test -p bcx-testkit`,
- no root dependency regression,
- deterministic fixture repeatability tests.

Exit criteria:

- graph, crypto, parser, and bundle tests can depend on one shared fixture
  vocabulary,
- deterministic test keys cannot enter production facade builds.

## Phase 3: Limits, Graph Admission, Replay, And Authority

### v0.17.0 - Core Limit And Budget Split

Goal: remove transport-named limit coupling from semantic model and crypto
verification.

Deliverables:

- general graph limits in core or policy,
- verification budgets in core or policy,
- checked `VerificationBudget` type,
- versioned `VerificationCostSchedule` type,
- `CostScheduleId` in the verification context,
- suite-specific and profile-specific worst-case debit values,
- registry-defined costs charged before proof-suite or provider dispatch,
- policy-controlled verification limits that untrusted statements cannot
  increase,
- key-resolution debits,
- classical and post-quantum verification debits,
- hybrid component debits,
- optional-attestation debits,
- transparency, membership, and non-inclusion proof debits,
- graph node and edge visit debits,
- policy evaluation debits,
- receipt and disclosure proof-check debits,
- rule that exhaustion is detected before starting the next expensive
  operation,
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
- tests that budgeted parser failures happen before allocation or crypto work,
- tests that verification-budget exhaustion happens before the next key
  resolution, signature verification, graph traversal, policy evaluation, or
  proof check,
- tests that different providers for the same suite debit identically,
- tests that untrusted statements cannot increase policy-controlled limits.

Exit criteria:

- semantic model and cryptographic verification do not depend on a
  transport-named crate for non-wire budgets,
- verification cost accounting is deterministic from registry, profile, and
  local policy data before any provider-specific code runs.

### v0.17.1 - Verification Outcome And Receipt Model

Goal: separate local resource exhaustion and missing evidence from
cryptographic or semantic invalidity.

Deliverables:

- `VerificationOutcome` model with `Valid`, `Invalid`, and indeterminate
  states,
- `Indeterminate(ResourceExhausted)` outcome,
- `Indeterminate(MissingEvidence)` outcome,
- unknown critical suite or algorithm is structurally invalid,
- suite forbidden by exact policy is invalid for admission,
- protocol-recognized and policy-allowed suite missing in the local
  implementation is `Indeterminate(UnsupportedLocally)`,
- temporarily unavailable external provider is
  `Indeterminate(ProviderUnavailable)`,
- missing required authorization evidence remains indeterminate as a truth
  claim but causes fail-closed denial for consequential admission,
- separate result dimensions for verification truth, evidence completeness,
  and authorization usability,
- rule that budget exhaustion cannot be cached as an invalid result,
- rule that invalid outcomes may be cached only when the cache key includes
  every relevant statement, policy, trust, revocation, conflict, checkpoint,
  suite, and profile root,
- cacheability matrix by outcome class:
  canonical decoding failures are permanently cacheable by object bytes,
  cryptographic failures are cacheable by statement, signature, key, suite, and
  verifier profile,
  policy or revocation failures are cacheable only with the exact policy,
  trust, and root snapshot,
  not-yet-valid results include evaluation time or expire at the validity
  boundary,
  expired results are cacheable only under the applicable clock or
  evaluation-point policy,
  replay or sequence results require replay-store generation or state binding,
  missing evidence, provider unavailable, and resource exhausted outcomes are
  never cached as invalid, and capability or delegation failures include
  capability, delegation, and authority-state roots,
- rule that retrying with a larger locally permitted budget may complete
  verification without changing the underlying statement validity,
- unsigned local verification receipts are diagnostics only,
- attestable verification receipt binding statement ID, verification context,
  roots, policy epoch, `CostScheduleId`, outcome, and completion state,
- explicit trusted role requirement for receipt signers,
- consumer rule to either re-execute verification or accept an attestable
  receipt under policy,
- rule that sender-provided receipts cannot suppress required local
  verification,
- rejection of receipt replay across different roots, policies, or
  checkpoints,
- offline-bundle field identifying the cost schedule used when claiming
  completed verification,
- documentation that local resource policies affect completion, not the
  statement's validity,
- security-controls update for indeterminate resource exhaustion.

Verification:

- outcome classification tests,
- cache-behavior tests proving resource exhaustion is not stored as invalid,
- complete invalid-result cache-key tests,
- per-outcome cacheability matrix tests,
- stale contextual cache fixtures,
- retry-with-larger-budget tests,
- verification receipt fixtures,
- unsigned receipt diagnostic-only fixtures,
- attestable receipt signer-role fixtures,
- receipt replay rejection fixtures,
- offline-bundle cost-schedule fixtures,
- conformance vectors for valid, invalid, resource-exhausted,
  missing-evidence, unsupported-locally, provider-unavailable, and
  policy-forbidden-suite outcomes.

Exit criteria:

- two implementations with different local budgets can disagree about
  completion without disagreeing about semantic validity,
- verification receipts are trusted only through local policy or re-execution.

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
- linearization or serialization point for validation plus insertion,
- failure atomicity: no partial edges, indexes, or reachability cache changes
  remain after failure,
- isolation expectations for database adapters,
- conflict and retry behavior,
- crash recovery requirements,
- transaction, compare-and-swap, or equivalent atomicity mechanism
  requirements,
- reachability check requirements,
- compact `CauseCapsule` normalization into causal edges or explicit
  deprecation behavior,
- adjacent duplicate checks over canonical parent order,
- rejection of the same parent ID under two relationship kinds; multiple roles
  for one parent use one canonical edge with a role set,
- rule that profile hooks cannot weaken parent-ID uniqueness,
- pruning safety rules using authenticated reachability or ancestry summaries,
  tombstones only as existence markers, and checkpoint-finalization boundaries,
- prohibition of unsafe edges from newer graph epochs into finalized ancestry
  where the ancestry summary cannot prove acyclicity,
- transitive metadata retention requirements for pruned ancestors,
- public identifier indexing policy hooks.

Verification:

- contract tests with deterministic in-memory graph fixtures,
- duplicate, self-parent, and two-node cycle fixtures,
- cycle-through-pruned-ancestor fixtures.

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
- transaction or compare-and-swap equivalent for the in-memory implementation,
- failure-injection tests for partial insert rollback,
- iterative reachability implementation,
- no recursion in traversal,
- explicit node, edge, and depth limits,
- pruning boundary representation,
- authenticated reachability or ancestry summary fixture,
- tombstone test fixture proving existence only,
- retained transitive metadata fixture for pruned ancestors.

Verification:

- arbitrary insertion-order tests,
- property tests for atomic admission,
- bounded acyclicity checks,
- concurrent insertion tests for check-then-insert races,
- crash and partial-failure recovery tests,
- pruning safety tests,
- cycle-through-pruned-ancestor tests,
- cycle and duplicate-delivery tests,
- no-default-features workspace pass.

Exit criteria:

- every accepted insertion leaves the bounded in-memory graph acyclic.

### v0.20.1 - Minimal Graph Reference Model

Goal: add executable graph semantics evidence before profile crates or
persistent adapters build on graph behavior.

Deliverables:

- minimal executable graph reference model,
- atomic insertion model,
- concurrent competing-insert model,
- missing-parent promotion model,
- pruning and finalization boundary model,
- duplicate-delivery model,
- cycle-through-pruned-ancestor model,
- operation-history generator,
- state-machine differential test harness comparing the reference model and
  bounded graph implementation after every operation,
- trace fixtures shared with `bcx-testkit`.

Verification:

- reference-model property test run,
- model-to-implementation fixture replay,
- generated operation-history differential tests,
- competing insert and missing-parent promotion tests,
- pruning/finalization cycle tests.

Exit criteria:

- profile and persistent adapter work depend on graph semantics with executable
  model evidence, not only example fixtures.

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
- unauthenticated global orphan count bound,
- unauthenticated transport-peer or source orphan count bound,
- per-issuer orphan count bound only after issuer authentication,
- rule that unauthenticated issuer claims cannot consume authenticated issuer
  quota,
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
- unauthenticated orphan-quota bypass tests,
- explanation bundle tests for incomplete graphs.

Exit criteria:

- two objects staged while unresolved cannot form an accepted cycle when both
  become available,
- unresolved objects are staged and not causally usable until deterministic
  promotion succeeds,
- validly shaped missing-parent references cannot fill storage without bounds,
- unauthenticated missing-parent submissions cannot block authenticated issuer
  capacity.

### v0.22.0 - Relationship Semantics And Edge Roles

Goal: enforce semantic constraints that require resolved parent metadata or
profile policy.

Deliverables:

- relationship cardinality rules,
- canonical dependency roles such as required authorization, required input,
  and observational context,
- role-set sorting and duplicate-free validation reused from `v0.15.0`,
- relationship and role compatibility matrix,
- target-kind checks for delegation, retry, scheduling, derivation, and joins,
- profile hook for domain-specific relationship constraints,
- rule that profile hooks may only narrow or supplement core edge invariants,
  never weaken them,
- fail-closed behavior for missing required parent metadata.

Verification:

- one-parent `JoinedFrom` rejection where policy requires several parents,
- multiple `RetryOf` rejection where policy requires one retry source,
- wrong-target-kind fixtures,
- required versus observational dependency fixtures,
- duplicate role fixtures,
- incompatible relationship and role fixtures.

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
- ordering rule: verify signature, key, audience, and basic authority before
  permanent replay-state commit,
- optional reservation model only with reserve, verify, commit, and abort
  states, opaque reservation tokens, TTLs, crash recovery, and strict
  reservation bounds,
- rule that failed authentication never permanently consumes nonce or sequence
  replay state,
- rule that two concurrent valid requests using the same nonce may both perform
  cryptographic work, but only one can atomically commit,
- rule that early duplicate lookup is advisory only and authoritative replay
  rejection happens during atomic commit,
- rule that reservations from unauthenticated sources cannot block
  authenticated traffic,
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
- invalid-signature replay poisoning fixtures,
- crash-during-reservation fixtures,
- advisory early duplicate lookup fixtures,
- cache saturation tests.

Exit criteria:

- consequential statements cannot be accepted without atomic replay and
  freshness policy,
- failed authentication cannot permanently consume replay state.

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
- hiding commitment construction for private values using non-public blinding
  material; public salts alone are not sufficient for low-entropy values,
- commitment suite identifier,
- domain-separated commitment preimage format,
- minimum blinding entropy requirement,
- prohibition on blinding reuse across fields or statements,
- canonical opening format and verification operation,
- binding of field path, statement context, and disclosure audience where
  applicable,
- explicit computational-hiding and binding assumptions,
- public evidence marker,
- encrypted field disclosure contract,
- AEAD suite identifier,
- crash-safe AEAD nonce generation or derivation rules across key epochs,
- rule that AEAD nonce derivation cannot depend on a statement ID when the
  ciphertext contributes to that statement ID,
- recipient and audience binding,
- ciphertext domain separation,
- padding or explicit length-leakage policy,
- disclosure key distribution and rotation behavior,
- explicit missing-evidence or completeness marker.

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
- attested decision mode: trust an authorized evaluator signature,
- reproducible decision mode: policy artifact and inputs are available for
  deterministic re-execution,
- proven decision mode: an admitted proof demonstrates correct evaluation,
- executable-policy metering, sandboxing, host-function, determinism, and
  evaluator resource-limit rules,
- distinction between `references policy X` and `verified as evaluated under
  policy X`,
- security-controls update for policy evaluation evidence.

Verification:

- policy commitment fixtures,
- wrong evaluator, version, input, and resolver fixtures,
- admission evidence mutation tests,
- fixtures for attested, reproducible, and proven decision modes.

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
- authenticated evaluation point for historical key and revocation selection,
- admitted evaluation points: trusted admission receipt time, checkpoint
  sequence and authenticated checkpoint time, transparency inclusion point, or
  profile-defined trusted clock evidence,
- explicit rejection of unsigned self-declared timestamps for selecting a
  historical key, validity interval, or revocation state,
- trust-anchor selection,
- deterministic failure for ambiguous `kid` matches,
- no network I/O during primitive verification,
- security-controls update for trust snapshots.

Verification:

- ambiguous key fixtures,
- wrong realm, audience, usage, and interval fixtures,
- unsigned self-declared timestamp rejection fixtures,
- checkpoint-relative historical key fixtures,
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

### v0.28.1 - Provider Assurance Classes

Goal: classify provider side-channel assurance before production provider
crates are admitted.

Deliverables:

- `ConstantTimeSoftware` provider assurance class,
- `HardwareIsolated` provider assurance class,
- `SideChannelUnassessed` provider assurance class,
- `TestOnly` provider assurance class,
- high-assurance policy rule requiring admitted constant-time software or
  appropriately isolated hardware providers for consequential signing,
- rule that documented variable-time behavior does not qualify for
  high-assurance consequential signing,
- verifier-provider adversary model for malicious or compromised providers,
- admitted primitive providers are part of the trusted computing base through
  `v1.0.0`,
- defended provider failures: format errors, crashes, resource abuse, and
  capability misreporting,
- trusted-boundary failure: an admitted compromised provider that returns
  arbitrary false cryptographic success,
- rule that provider assurance classes are assigned by local policy or
  admission records, never trusted directly from provider self-reported
  metadata,
- multi-provider runtime agreement is not required through `v1.0.0` and is
  reserved for `v1.1.0 - Multi-Provider Verification Agreement` if BCX admits
  that model after `v1.0.0`,
- resource-amplification input handling requirements for provider dispatch.

Verification:

- assurance-class policy tests,
- high-assurance rejection tests for side-channel-unassessed providers,
- test-only provider feature-guard tests,
- malicious verifier provider fixtures,
- provider self-reported metadata rejection fixtures,
- trusted-boundary failure documentation check,
- resource-amplification fixtures.

Exit criteria:

- provider admission distinguishes documentation from assurance, and
  high-assurance profiles cannot accidentally admit variable-time signing,
- BCX's `v1.0.0` security claim does not tolerate arbitrary Byzantine false
  success from an admitted primitive provider.

### v0.29.0 - Signing Provider Boundary

Goal: define how BCX creates attestations without exposing private key material
or partial hybrid signatures.

Deliverables:

- opaque private-key handles,
- signer capability metadata,
- provider assurance class from `v0.28.1`,
- injected RNG or entropy interface for no-std,
- ML-DSA and SLH-DSA deterministic versus hedged mode policy,
- rule that providers cannot fall back from hedged to deterministic signing
  unless local policy explicitly permits it,
- algorithm context selection,
- secret scratch zeroization,
- secret-key and seed zeroization requirements,
- entropy-source health checks and failure propagation,
- fault-injection behavior for malformed internal state,
- platform capability declaration for embedded and hardware-backed signing,
- distinction between BCX-enforced guarantees and external-provider asserted
  guarantees,
- atomic hybrid signing,
- no partial hybrid signature release when one component fails,
- key generation and import boundaries,
- security-controls update for signer entropy and private-key handling.

Verification:

- fake signer tests,
- provider assurance-class tests,
- entropy failure tests,
- hedged-signing fallback rejection tests,
- malformed internal-state rejection tests,
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
- component key epoch binding,
- rule that revocation or expiry of either mandatory component invalidates the
  composite key for new acceptance,
- rule that components from different key epochs cannot be mixed,
- partial component rotation creates a new composite commitment,
- historical verification uses the composite record valid at an authenticated
  evaluation point, not at an unsigned claimed signing timestamp,
- admitted composite-key evaluation points: trusted admission receipt time,
  checkpoint sequence and authenticated checkpoint time, transparency inclusion
  point, or profile-defined trusted clock evidence,
- rule that one component cannot be silently reused in incompatible composite
  suites,
- compromise recovery and emergency downgrade behavior is policy-bound and
  fail-closed,
- domain-separated composite key commitment.

Verification:

- component substitution tests,
- old-Ed25519/new-ML-DSA component mixing tests,
- expired-component and revoked-component tests,
- unsigned claimed signing timestamp rejection tests,
- authenticated evaluation point fixtures,
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
- panic and unwind wipe behavior where the platform supports unwinding,
- explicit abort limitation,
- provider-owned internal memory and secret-copy responsibility,
- hardware keystore, HSM, DMA, and external-provider limitation notes,
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
- exact duplicate-proof rejection,
- duplicate key rejection for threshold counting,
- stable trust-principal identity for unique signer counting,
- retention of multiple keys from one principal as evidence while counting
  that principal once per applicable role,
- key-rotation overlap behavior,
- role-specific unique-signer counting rules,
- canonical signer ordering,
- unique signer counting,
- signer roles and trust domains,
- all-of, any-of, threshold, and named-role policies,
- optional-attestation failure semantics,
- assurance aggregation rules,
- unknown proof-suite handling.

Verification:

- valid and invalid proof fixtures,
- exact duplicate proof fixtures,
- duplicate key threshold-counting fixtures,
- key-rotation overlap fixtures,
- stable-principal multi-key fixtures,
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
- provider assurance class from `v0.28.1`,
- constant-time software or hardware-isolated admission requirement for
  high-assurance consequential signing,
- side-channel-unassessed provider admission limited to non-high-assurance or
  test-only policy where explicitly allowed,
- secret-key and seed zeroization guarantees,
- entropy-source health and failure propagation,
- hedged-signing fallback policy where the provider supports hedged signing,
- fault-injection behavior and malformed internal-state rejection,
- platform capability declaration for embedded and hardware-backed
  implementations,
- clear distinction between guarantees enforced by BCX and guarantees asserted
  by the external provider,
- signing known-answer tests,
- verification known-answer tests,
- malformed key and signature vectors,
- cross-provider-ready test interface.

Verification:

- `cargo test -p <ed25519-provider-crate>`,
- RFC 8032 vector smoke tests,
- provider assurance-class review,
- high-assurance provider admission tests,
- entropy failure tests,
- malformed internal-state tests,
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
- provider assurance class from `v0.28.1`,
- constant-time software or hardware-isolated admission requirement for
  high-assurance consequential signing,
- side-channel-unassessed provider admission limited to non-high-assurance or
  test-only policy where explicitly allowed,
- secret-key and seed zeroization guarantees,
- entropy-source health and failure propagation,
- hedged-versus-deterministic signing mode declaration,
- rule that hedged signing cannot silently fall back to deterministic signing
  unless local policy explicitly permits it,
- fault-injection behavior and malformed internal-state rejection,
- platform capability declaration for embedded and hardware-backed
  implementations,
- clear distinction between guarantees enforced by BCX and guarantees asserted
  by the external provider,
- signing known-answer tests,
- verification known-answer tests,
- malformed key and signature vectors,
- cross-provider signature generation and verification fixtures.

Verification:

- `cargo test -p <ml-dsa-provider-crate>`,
- NIST KAT or ACVP-vector smoke tests where available,
- provider assurance-class review,
- high-assurance provider admission tests,
- entropy failure tests,
- malformed internal-state tests,
- hedged fallback policy tests,
- no root dependency regression.

Exit criteria:

- BCX has one admitted post-quantum provider for signing and verification
  before hybrid and COSE conformance are claimed.

### v0.36.3 - Provider Oracle And SLH-DSA Scope Decision

Goal: make conformance possible with independent verification evidence and
settle whether SLH-DSA is mandatory for the `1.0.0` suite.

Deliverables:

- second dev/test-only provider or independent reference oracle for Ed25519,
- second dev/test-only provider or independent reference oracle for ML-DSA-65,
- cross-provider or oracle-backed signature generation and verification
  fixtures,
- SLH-DSA decision: admitted mandatory provider milestone or explicitly
  reserved outside the initial mandatory `1.0.0` suite,
- malformed key and signature corpus shared across providers and oracles.

Verification:

- provider/oracle differential smoke tests,
- SLH-DSA scope documentation check,
- no root dependency regression.

Exit criteria:

- the `v0.37.0` conformance program can compare against independent evidence
  instead of trusting one implementation per algorithm.

### v0.37.0 - Cryptographic Conformance Program

Goal: prove primitive and hybrid verification against external vectors and
adversarial suite mutations before profiles depend on these surfaces.

Deliverables:

- executable imported-vector harnesses,
- pinned vector provenance with expected version and digest for every corpus,
- complete RFC 8032 Ed25519 KAT execution,
- complete NIST ACVP or KAT execution for mandatory ML-DSA parameter sets,
- RFC 9964 JOSE/COSE vector execution where applicable to admitted suites,
- automated provider-oracle differential tests,
- signing and verification cross-product tests across admitted providers,
- negative-vector execution for malformed keys, signatures, headers, and
  suite metadata,
- reproducible pass/fail report generation,
- SLH-DSA mandatory-or-reserved result from `v0.36.3`,
- hybrid component swap and strip corpus,
- downgrade and cross-suite reuse corpus.

Verification:

- complete imported-vector harness run,
- provider-oracle differential test run,
- cross-product signing and verification run,
- negative corpus run,
- generated conformance report check,
- hybrid negative corpus run.

Exit criteria:

- cryptographic acceptance is measured against standards vectors and
  adversarial suite mutations through executable, reproducible harnesses.

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
- non-inclusion interface fixtures,
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
- verification receipt vocabulary from `v0.17.1`,
- distinction between unsigned local diagnostic receipts and attestable
  verification receipts,
- distinct verification-receipt signature domain,
- direct receipt verification path that does not recursively require another
  verification receipt,
- trusted verifier role requirements for receipt acceptance,
- policy rule that consumers re-execute verification unless they accept a
  receipt under an explicit trusted verifier role,
- transparency receipt vocabulary for inclusion, consistency, disclosure, and
  non-inclusion,
- receipt-to-statement commitment rules,
- receipt preimage binding to statement ID, receipt signer identity,
  verification context, verification profile and version, authenticated
  evaluation point, roots, policy epoch, `CostScheduleId`, outcome, and
  completion state where applicable,
- receipt assurance classification.

Verification:

- receipt validation fixtures,
- forged verification receipt fixtures,
- recursive receipt-verification rejection fixtures,
- wrong receipt-signature domain fixtures,
- sender-provided receipt cannot suppress local verification fixtures,
- cross-root, cross-policy, and cross-checkpoint receipt replay fixtures,
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

### v0.45.0 - Explanation Bundle Skeleton

Goal: define the internal bounded proof-bundle structure before final
contradiction and privacy fields are frozen.

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
- placeholder extension points for contradiction and disclosure data scheduled
  in `v0.46.0`, `v0.47.0`, and `v0.47.1`.

Verification:

- bundle validation tests,
- missing-parent fixture tests.

Exit criteria:

- an offline verifier can see what is proven, missing, redacted, stale,
  withheld, truncated, or unknown without claiming final contradiction or
  privacy semantics.

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
- hiding commitment guidance for low-entropy values using non-public blinding
  material; public salts alone are not sufficient,
- commitment suite identifier and domain-separated preimage rules,
- minimum blinding entropy and no-reuse rules across fields and statements,
- canonical opening format and verification operation,
- field path, statement context, and disclosure audience binding rules,
- explicit computational-hiding and binding assumptions,
- selective disclosure binding,
- encrypted field disclosure contract,
- AEAD suite identification,
- crash-safe AEAD nonce generation or derivation rules across key epochs,
- AEAD nonce derivation rule that avoids circular dependence on statement IDs
  when ciphertext contributes to those IDs,
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

### v0.47.1 - Final Explanation Bundle Closure

Goal: finalize the offline WHY bundle schema after contradiction semantics and
privacy/disclosure cryptography are defined.

Deliverables:

- final contradiction fields using `v0.46.0` semantics,
- disclosure map binding fields to plaintext, ciphertext, commitments, or
  predicate proofs,
- self-contained and thin/referenced bundle modes,
- deterministic dependency-closure and deduplication algorithm,
- inclusion of all required key records, policy records, checkpoints, registry
  entries, attestations, and proofs in self-contained mode,
- explicit incompleteness marker for every unresolved reference,
- clear separation between included untrusted material and externally supplied
  trust anchors,
- rule that a bundle cannot manufacture its own trust root by embedding it,
- verification rule that self-contained mode performs no network lookup,
- final bundle commitment or signature.

Verification:

- self-contained closure fixtures,
- thin/referenced bundle fixtures,
- missing dependency marker tests,
- no-network-lookup tests for self-contained mode,
- trust-anchor substitution tests.

Exit criteria:

- an offline verifier can verify bundle closure deterministically and can see
  exactly what is proven, missing, redacted, stale, contradicted, withheld,
  truncated, or externally trusted.

### v0.47.2 - Pre-Offline Profile Specification Gate

Goal: define the minimum normative profile contract before the first profile
crate is implemented.

Deliverables:

- profile normative specification template,
- required profile security-contract sections,
- object and bundle schema requirements,
- registry and critical-extension requirements,
- downgrade behavior requirements,
- finality and trust-anchor requirements,
- completeness and unresolved-reference requirements.

Verification:

- documentation lint for required profile sections,
- offline-profile spec stub review.

Exit criteria:

- `bcx-offline` can start with the same normative template every later profile
  must follow.

### v0.47.3 - Core Object And Bundle Parser Fuzzing

Goal: fuzz core object and WHY bundle parsers before offline bundles parse
untrusted files.

Deliverables:

- cargo-fuzz targets for statements, attestations, bindings, checkpoints, and
  WHY bundles,
- malformed seed corpus for canonical CBOR objects,
- truncation, nonminimal integer, duplicate key, trailing byte, deep nesting,
  unknown critical field, and oversized proof seeds,
- accepted-object re-encode identity assertion,
- differential CBOR checks for canonical acceptance and original-byte
  preservation,
- corpus regression tests.

Verification:

- local core parser fuzz smoke,
- malformed corpus regression tests,
- no-default-features parser tests.

Exit criteria:

- offline profile parsing depends on already-fuzzed core object and bundle
  parser boundaries.

### v0.47.4 - Pre-Offline no_std And Zero-Copy Evidence

Goal: define feature tiers and prove zero-copy behavior before offline profile
implementation depends on core APIs.

Deliverables:

- no-std feature tier policy,
- core-only tier without allocation,
- alloc-enabled borrowed/owned conversion tier,
- optional std adapter tier,
- end-to-end zero-copy allocation benchmarks for canonical parse, verify, and
  bundle-read paths,
- feature-leakage guard for offline dependencies.

Verification:

- `cargo test --workspace --no-default-features`,
- feature matrix checks,
- allocation benchmark run,
- zero-copy evidence report.

Exit criteria:

- offline profile work can start knowing which BCX APIs are core-only,
  alloc-enabled, or std-adapter APIs.

### v0.48.0 - Offline Profile

Goal: add `bcx-offline` for air-gapped bundles after final explanation,
contradiction, privacy, profile-specification, parser-fuzzing, and no-std
evidence gates exist.

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

### v0.50.0 - Core And Codec Specification Consolidation

Goal: consolidate the living normative security specifications that began in
`v0.11.0` through `v0.17.1` before carrier profiles start.

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

- implementation behavior is traceable to consolidated written protocol
  requirements before carrier profiles start.

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
- differential CBOR checks for canonical acceptance and original-byte
  preservation, not only decoded values,
- differential CBOR decode plan.

Verification:

- local fuzz smoke,
- corpus regression tests,
- per-profile fuzz target added for every profile milestone from `v0.55.0`
  onward that introduces parser surface.

Exit criteria:

- every parser has a reproducible malformed-input corpus and a fuzz entry
  point before HTTP implementation starts.

### v0.54.1 - Pre-Carrier no_std Feature Tiers And Zero-Copy Evidence

Goal: refresh feature-tier and zero-copy evidence before network carrier
implementations depend on core APIs.

Deliverables:

- no-std feature tier policy refresh from `v0.47.4`,
- core-only tier without allocation refresh,
- alloc-enabled borrowed/owned conversion tier refresh,
- optional std adapter tier refresh,
- end-to-end zero-copy allocation benchmarks,
- throughput benchmarks for canonical parse, verify, and bundle-read paths,
- feature-leakage guard for profile dependencies.

Verification:

- `cargo test --workspace --no-default-features`,
- feature matrix checks,
- allocation benchmark run,
- throughput benchmark run.

Exit criteria:

- carrier profile implementation can start with refreshed evidence for
  core-only, alloc-enabled, and std-adapter API boundaries.

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
- supported fault model declaration,
- exact quorum formula; for the unweighted Byzantine model, quorum size `q`
  from `n` members must satisfy `2q > n + f` to intersect in more than `f`
  members,
- weighted quorum forbidden through `v1.0.0`,
- overflow-safe arithmetic for threshold and quorum calculations,
- rejection of impossible threshold, quorum, and fault configurations at
  policy construction time,
- signer-set epoch binding,
- signer membership commitment,
- fault assumption and maximum tolerated faulty witnesses,
- quorum-intersection requirement for finality claims,
- rules preventing two disjoint quorums from finalizing conflicting roots,
- double-signing and equivocation evidence,
- membership rotation rules,
- threshold change binding,
- behavior when the threshold or signer set changes,
- joint-consensus or overlapping-epoch transition rules,
- old-and-new quorum requirements during joint transitions,
- proof-of-possession for witness keys where required by the admitted suite,
- signer set commitment,
- threshold count validation,
- witness signature bundle model,
- duplicate signer rejection.

Verification:

- `cargo test -p bcx-proof-threshold`,
- signer-set epoch and rotation fixtures,
- exact quorum-formula fixtures,
- boundary fixtures for `q = 0`, `q > n`, `f >= n`, and arithmetic at integer
  maxima,
- overflow-safe arithmetic fixtures,
- impossible-configuration rejection tests,
- quorum-intersection fixtures,
- insufficient-intersection conflicting checkpoint tests,
- sufficient-intersection conflict rejection tests,
- equivocation evidence fixtures,
- joint-consensus transition tests,
- old-and-new quorum transition tests,
- proof-of-possession fixtures,
- threshold mutation tests.

Exit criteria:

- a checkpoint can be witnessed by a private or federated group without using a
  public blockchain,
- threshold finality cannot be claimed by two disjoint quorums under the
  configured fault assumption.

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

### v0.87.0 - Conformance Crate Completion

Goal: complete the `bcx-conformance` crate scaffold from `v0.16.1` with all
mandatory interoperability vectors.

Deliverables:

- canonical encoding vectors,
- signature vectors,
- verification outcome classification vectors,
- cost-schedule and resource-exhaustion vectors,
- replay cases,
- unknown extension cases,
- binding mutation cases,
- checkpoint cases.

Verification:

- conformance test suite.

Exit criteria:

- independent implementations can validate compatibility using the full
  mandatory vector suite.

### v0.88.0 - Testkit Crate Expansion

Goal: expand the `bcx-testkit` crate scaffold from `v0.16.2` for the complete
release-candidate fixture set.

Deliverables:

- expanded statement, attestation, and binding scenario catalog,
- advanced tamper helper catalog,
- additional deterministic test-only key sets behind the `v0.16.2`
  production-facade guards,
- adversarial receipt, graph, provider, and profile fixture packs,
- fixture coverage report.

Verification:

- `cargo test -p bcx-testkit`,
- production facade feature-guard check,
- fixture coverage report check.

Exit criteria:

- releases from `v0.89.0` through `v1.0.0` can add tests faster without
  weakening production crates.

### v0.89.0 - Graph And State Modeling

Goal: model graph and semantic-state invariants under reordering,
duplication, missing parents, and concurrent insertion.

Deliverables:

- expansion of the minimal graph reference model from `v0.20.1`,
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
- alloc/std feature documentation,
- review that the `v0.54.1` feature tier and zero-copy evidence still holds.

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
