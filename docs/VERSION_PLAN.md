# BCX Version Plan

Status: planning document

This plan is intentionally granular. BCX is a security-sensitive protocol
crate, so each tag should be a small, testable step with a clean stop before
pentest and tagging.

Tags use:

```text
v0.N.0      milestone release
v0.N.P      patch/fix release for milestone N
v1.0.0      first serious production-ready BCX crate
```

The list below is not a maximum. Add patch tags or split milestones further
whenever a release would otherwise mix too many security surfaces.

## Release Principles

Every release must have:

- a clear definition of done,
- a local verification command,
- security review notes,
- known limitations,
- release notes,
- a completed pentest report for the release,
- no hidden dependency on one developer machine.

Every release should prefer:

- small protocol increments,
- no-std model tests before transport integration,
- deterministic behavior before provider-specific behavior,
- bounded parsing and graph traversal before federation,
- explicit capability-aware APIs even when enforcement is still simple.

No production claim is allowed before `1.0.0`.

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
- the pentest report records `Input-Digest: sha256:<digest>` for the scratch
  `PENTEST.md` input,
- the pentest report has `Status: PASS`,
- the pentest report has non-blank `Tester:` and `Scope:` fields,
- the pentest report has a `Date: YYYY-MM-DD` field,
- root `PENTEST.md` does not exist,
- the tag does not already exist locally,
- `scripts/validate-release-readiness.sh <tag>` passes.

When a version's implementation criteria are done, stop before tagging and say:

```text
vX.Y.Z implementation stop reached. Run pentest for this release.
```

Do not tag until the pentest has been completed, findings have been fixed, and
the permanent report is committed.

### Pentest Handoff Flow

Use this loop for every version:

1. The implementation owner finishes the criteria and reports the release is
   ready for review.
2. The maintainer runs pentest and writes temporary findings to root
   `PENTEST.md`.
3. Findings are reviewed, release-scope issues are fixed, documentation or
   release notes are updated, and `PENTEST.md` is deleted.
4. Local gates run again.
5. The maintainer reruns pentest if needed.
6. When GitHub CI and CodeQL default setup are green, the preferred automated
   flow is:

```bash
scripts/finalize_release.py \
  --version X.Y.Z \
  --tester "<tester>" \
  --scope "<scope>" \
  --date YYYY-MM-DD
```

This records the scratch report, deletes root `PENTEST.md`, commits the
permanent report, runs the version release gate, and creates the local tag.
The pentest report is a release approval artifact. It records the digest of the
scratch `PENTEST.md` that the maintainer supplied, but it does not bind the
approval to a single git commit. After the maintainer confirms pentest and
GitHub are green, the release may be finalized without restarting pentest for
release-process-only commits.

The lower-level report-only command is:

```bash
scripts/record_pentest_report.py \
  --version X.Y.Z \
  --tester "<tester>" \
  --scope "<scope>" \
  --date YYYY-MM-DD
```

7. Review `security/pentest/<tag>.md`. The final tag commit is allowed to be
   the permanent report commit.
8. `scripts/validate-release-readiness.sh <tag>` passes.
9. Tagging and pushing tags happen only when explicitly requested.
10. Publishing uses `scripts/release_crate.py --version X.Y.Z --require-tag`;
    publish-time checks must accept that the tag already exists and must verify
    the tag points at the report commit while the pentest report points at the
    audited implementation commit.

Never commit root `PENTEST.md`; it is scratch input and is ignored by git.

## Phase 0: Repository Foundation

### v0.1.0 - Repository Foundation

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
- `scripts/release_0_1_gate.sh` after pentest report exists
- `scripts/validate-release-readiness.sh v0.1.0` after pentest report exists

Exit criteria:

- a contributor can understand the 1.0 target from the README and plans,
- all non-generated Rust files stay under 500 lines,
- implementation stop is reported before any tag.

### v0.2.0 - Toolchain And Release Gate

Goal: make release readiness mechanically checkable before protocol expansion.

Deliverables:

- release-readiness validator,
- pentest report metadata checks,
- version-specific release gate pattern,
- publish-readiness checks that are separate from pre-tag readiness checks,
- scratch pentest digest capture in permanent reports,
- release-note filename checks,
- pre-tag existing-tag rejection,
- root `PENTEST.md` rejection.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_2_gate.sh` after pentest report exists
- negative tests by temporarily omitting required release files locally

Exit criteria:

- the project cannot accidentally tag without release notes and pentest report
  metadata.

## Phase 1: Core Protocol Vocabulary

### v0.3.0 - Identifier Kernel

Goal: harden core IDs and commitments.

Deliverables:

- event ID validation,
- digest commitment types,
- nonce type,
- issuer sequence type,
- zero-value rejection tests.

Verification:

- `cargo test -p bcx-core`
- `cargo test --workspace --no-default-features`

Exit criteria:

- every public constructor rejects invalid zero or empty authority values where
  required.

### v0.4.0 - Causal Vocabulary

Goal: model operation causes and parent edges without receipts yet.

Deliverables:

- relationship kinds,
- cause kinds,
- operation actions,
- compact cause capsule,
- parent-count validation.

Verification:

- `cargo test -p bcx-model`
- model fixture tests for parent limit failures

Exit criteria:

- BCX can represent local causal parentage without relying on transport fields.

### v0.5.0 - Truth And Assurance Vocabulary

Goal: make explanation claims honest about evidence quality.

Deliverables:

- truth status model,
- assurance level model,
- ordering tests for assurance levels,
- docs describing declared versus observed versus verified claims.

Verification:

- `cargo test -p bcx-model`
- doc build with warnings denied

Exit criteria:

- no explanation API can collapse declared purpose into verified truth.

### v0.6.0 - Wire Version And Limits

Goal: enforce cheap bounds before expensive parsing or verification.

Deliverables:

- protocol version model,
- wire header validation,
- default development limits,
- negative tests for empty, oversized, and wrong-major messages.

Verification:

- `cargo test -p bcx-wire`
- `cargo clippy --workspace --all-targets -- -D warnings`

Exit criteria:

- every future decoder has a common bounded-entry policy.

## Phase 2: Canonical Encoding

### v0.7.0 - Encoding Format Decision

Goal: choose and document the canonical encoding boundary.

Deliverables:

- encoding decision record,
- accepted canonical format,
- rejected alternatives,
- no-std and alloc impact analysis,
- dependency admission notes if a third-party crate is needed.

Verification:

- docs checks,
- dependency policy review if a crate is admitted.

Exit criteria:

- implementation cannot proceed with ad hoc JSON or Rust layout encoding.

### v0.8.0 - Decoder Bounds

Goal: add bounded decode scaffolding before full object encoding.

Deliverables:

- maximum depth,
- maximum item counts,
- maximum byte lengths,
- malformed-input error model,
- tests for fail-fast bounds.

Verification:

- `cargo test --workspace --no-default-features`
- malformed fixture tests

Exit criteria:

- invalid payloads fail before signature or graph work begins.

### v0.9.0 - Canonical Core Encoding

Goal: encode core IDs and model enums deterministically.

Deliverables:

- canonical bytes for IDs,
- canonical bytes for cause vocabulary,
- stable test vectors,
- cross-platform byte-equivalence tests.

Verification:

- `cargo test --workspace`
- fixture regeneration check if fixtures are introduced

Exit criteria:

- two supported platforms produce identical canonical bytes for core values.

### v0.10.0 - Canonical Invocation Skeleton

Goal: make the first signed object payload shape stable enough for review.

Deliverables:

- invocation skeleton,
- issuer and audience fields,
- parent event fields,
- action and target commitment fields,
- nonce, expiry, and sequence fields.

Verification:

- canonical test vectors,
- invalid field tests,
- no-default-features test pass.

Exit criteria:

- an invocation can be encoded without any transport dependency.

## Phase 3: Crypto Envelopes

### v0.11.0 - Signature Envelope Bounds

Goal: validate signature metadata before provider dispatch.

Deliverables:

- signature envelope validation,
- key ID bounds,
- signature payload bounds,
- algorithm ID closed-list checks.

Verification:

- `cargo test -p bcx-crypto`
- negative tests for empty, oversized, and unknown signatures.

Exit criteria:

- malformed signature envelopes fail without invoking crypto providers.

### v0.12.0 - Verifier Provider Trait

Goal: define the crypto provider boundary without choosing a provider.

Deliverables:

- verifier trait,
- provider error model,
- admitted algorithm policy hook,
- test verifier for fixtures.

Verification:

- unit tests using a deterministic test verifier,
- clippy with warnings denied.

Exit criteria:

- real crypto crates can be added later without changing signed-object semantics.

### v0.13.0 - Signed Invocation Verification

Goal: verify invocation signatures over canonical bytes.

Deliverables:

- signed invocation envelope,
- audience binding check,
- expiry check hook,
- sequence and nonce presence checks,
- wrong-audience and changed-payload tests.

Verification:

- `cargo test --workspace`
- tamper fixture tests.

Exit criteria:

- changing any security-relevant invocation field invalidates verification.

## Phase 4: Capabilities And Replay

### v0.14.0 - Capability Metadata

Goal: define proof-of-possession capability metadata.

Deliverables:

- capability subject,
- holder key commitment,
- audience,
- scope,
- expiry,
- delegation caveat fields.

Verification:

- constructor and validation tests,
- docs for bearer-token rejection.

Exit criteria:

- BCX has no bearer-only capability path for consequential operations.

### v0.15.0 - Capability Attenuation

Goal: allow child capabilities to narrow but never silently broaden authority.

Deliverables:

- attenuation rules,
- scope narrowing checks,
- purpose and retention narrowing checks,
- delegation-depth bounds.

Verification:

- tests for valid narrowing,
- tests for scope, purpose, retention, and depth broadening failures.

Exit criteria:

- child operations cannot remove parent restrictions.

### v0.16.0 - Replay Protection Hooks

Goal: make replay resistance explicit before receipts exist.

Deliverables:

- replay cache trait,
- issuer sequence validation,
- nonce validation,
- expiry window validation,
- idempotency-key metadata.

Verification:

- replay fixture tests,
- expired invocation tests,
- duplicate sequence tests.

Exit criteria:

- repeated consequential invocations are rejected by the verification layer.

## Phase 5: Receipts

### v0.17.0 - Admission Receipt Skeleton

Goal: record why an operation was allowed, denied, narrowed, or quarantined.

Deliverables:

- admission receipt type,
- admission result,
- policy digest,
- configuration digest,
- identity and capability status.

Verification:

- required-field tests,
- policy mismatch tests.

Exit criteria:

- a verifier can distinguish admission from execution.

### v0.18.0 - Admission Obligations

Goal: attach enforceable obligations to admission decisions.

Deliverables:

- retention obligation fields,
- disclosure obligation fields,
- onward-sharing markers,
- human-review markers,
- obligation bounds.

Verification:

- obligation validation tests,
- malformed and oversized obligation tests.

Exit criteria:

- admission can narrow or condition an operation without executing it yet.

### v0.19.0 - Effect Receipt Skeleton

Goal: record what actually happened after execution.

Deliverables:

- effect receipt type,
- effect result,
- response commitment,
- state transition commitment,
- child invocation references.

Verification:

- receipt validation tests,
- child-reference bound tests.

Exit criteria:

- BCX can represent actual effects separately from requested intent.

### v0.20.0 - Effect Assurance Rules

Goal: prevent gateways from claiming effects they cannot observe.

Deliverables:

- effect source model,
- gateway-observed effect class,
- application-observed effect class,
- database/storage-observed effect class,
- negative tests for overclaiming.

Verification:

- gateway overclaim tests,
- application receipt fixture tests.

Exit criteria:

- effect receipts name what component had authority to observe the effect.

## Phase 6: Local Explanation

### v0.21.0 - Explanation Query Bounds

Goal: make local WHY queries bounded before graph traversal.

Deliverables:

- explanation query type,
- direction,
- maximum depth,
- maximum events,
- maximum response bytes,
- requested claim classes.

Verification:

- query validation tests,
- oversized query tests.

Exit criteria:

- no WHY query can request unbounded work.

### v0.22.0 - Explanation Bundle

Goal: return bounded proof bundles for local events.

Deliverables:

- explanation bundle type,
- signed event references,
- verified edge list,
- missing event list,
- redacted edge list,
- verification summary.

Verification:

- bundle validation tests,
- missing-parent fixture tests.

Exit criteria:

- missing and redacted information is explicit, never silently omitted.

### v0.23.0 - Contradiction Reporting

Goal: report conflicting claims without hiding disagreement.

Deliverables:

- contradictory claim model,
- assurance comparison,
- conflict categories,
- deterministic ordering.

Verification:

- conflicting-parent tests,
- mismatched-receipt tests.

Exit criteria:

- BCX can report contradiction instead of choosing a convenient story.

## Phase 7: Local Storage And CLI

### v0.24.0 - Append-Only Store Trait

Goal: define durable local event storage without selecting a database.

Deliverables:

- append-only store trait,
- event lookup trait,
- retention metadata,
- tamper-evidence hooks.

Verification:

- in-memory store tests,
- missing-parent tests.

Exit criteria:

- local event chains can be recorded and queried behind a stable trait.

### v0.25.0 - Hash Chain Or Merkle Commitments

Goal: make local storage tamper-evident.

Deliverables:

- chain or Merkle root metadata,
- batch commitment type,
- witness commitment hook,
- tamper fixture tests.

Verification:

- altered event tests,
- missing batch tests,
- deterministic commitment tests.

Exit criteria:

- local store corruption is detected instead of silently accepted.

### v0.26.0 - CLI Verify

Goal: provide offline verification before live federation.

Deliverables:

- `bcx-cli` crate,
- `bcx verify` command,
- bundle input format,
- human-readable verification summary.

Verification:

- CLI fixture tests,
- invalid bundle tests.

Exit criteria:

- signed proof bundles can be verified without Fluxheim running.

### v0.27.0 - CLI Why Local

Goal: provide a local WHY command over stored events.

Deliverables:

- `bcx why` command,
- ancestor traversal,
- descendant traversal,
- missing/redacted output,
- bounded query flags.

Verification:

- CLI local graph fixtures,
- depth and node limit tests.

Exit criteria:

- local WHY answers can be inspected without federation.

## Phase 8: Fluxheim Local Integration

### v0.28.0 - Fluxheim Ingress Events

Goal: make Fluxheim produce BCX events at request ingress.

Deliverables:

- ingress event mapping,
- authentication result mapping,
- route policy digest,
- request target commitment.

Verification:

- Fluxheim fixture tests,
- local BCX validation tests.

Exit criteria:

- Fluxheim can explain that a request arrived and which route policy handled it.

### v0.29.0 - Fluxheim Upstream And Response Events

Goal: link ingress events to upstream dispatch and response observations.

Deliverables:

- upstream dispatch event,
- upstream acknowledgement event,
- response commitment,
- local child edge.

Verification:

- Fluxheim local chain tests,
- tampered child edge tests.

Exit criteria:

- one Fluxheim request can produce an end-to-end local causal chain.

### v0.30.0 - Fluxheim Local Why

Goal: expose local WHY for Fluxheim operators.

Deliverables:

- `fluxheim why` prototype or integration hook,
- local explanation bundle,
- policy digest display,
- missing evidence display.

Verification:

- local WHY fixture tests,
- operator output snapshot tests.

Exit criteria:

- Fluxheim can answer local "why did this request happen?" without federation.

## Phase 9: HTTP Compatibility Binding

### v0.31.0 - HTTP Carrier Mapping

Goal: carry BCX objects over HTTP without trusting the HTTP wrapper.

Deliverables:

- `bcx-http` crate,
- request mapping,
- response mapping,
- content type,
- `.well-known/bcx` endpoint shape.

Verification:

- mapping tests,
- invalid content-type tests.

Exit criteria:

- HTTP can transport canonical BCX bytes without becoming the security source.

### v0.32.0 - HTTP Downgrade Rejection

Goal: make HTTP compatibility fail closed.

Deliverables:

- required-mode policy,
- optional-mode policy,
- unsigned request rejection,
- unsupported version rejection.

Verification:

- downgrade tests,
- unsigned request tests,
- wrong-version tests.

Exit criteria:

- an attacker cannot silently strip BCX and continue as ordinary HTTP.

### v0.33.0 - HTTP Tamper Fixtures

Goal: prove wrapper mutation cannot alter the signed operation.

Deliverables:

- tampered target fixture,
- tampered method fixture,
- tampered digest fixture,
- proxy rewrite fixture.

Verification:

- all tamper fixtures fail closed.

Exit criteria:

- BCX verification depends on canonical signed payloads, not mutable HTTP
  decoration.

## Phase 10: Federated Peer Model

### v0.34.0 - Peer Identity And Trust Domains

Goal: model peer trust without global identity.

Deliverables:

- peer identity type,
- trust-domain metadata,
- pairwise event identifier support,
- peer policy reference.

Verification:

- peer validation tests,
- wrong-domain tests.

Exit criteria:

- cross-domain events carry explicit peer and trust-domain context.

### v0.35.0 - Cross-Acknowledged Edges

Goal: prevent one peer from fabricating a federated graph alone.

Deliverables:

- sender observation,
- receiver observation,
- cross-acknowledgement receipt,
- edge assurance upgrade rules.

Verification:

- false-parent tests,
- sender-only claim tests,
- receiver mismatch tests.

Exit criteria:

- strong federated edges require both participant perspectives.

### v0.36.0 - Key Revocation Metadata

Goal: make key compromise and revocation visible to verification.

Deliverables:

- key epoch metadata,
- revocation reference,
- verification-time revocation status,
- affected-event markers.

Verification:

- revoked key tests,
- old valid receipt tests,
- compromised epoch tests.

Exit criteria:

- verification reports revocation impact instead of silently trusting old keys.

## Phase 11: Federated WHY

### v0.37.0 - WHY Query Authentication

Goal: require authorization for federated explanation queries.

Deliverables:

- audit capability reference,
- requester signature,
- query purpose,
- audience binding,
- nonce.

Verification:

- unauthenticated query tests,
- wrong-audience tests,
- replayed query tests.

Exit criteria:

- remote WHY cannot be used as an unauthenticated graph crawler.

### v0.38.0 - Selective Disclosure

Goal: control what a peer may disclose.

Deliverables:

- plaintext disclosure,
- encrypted disclosure,
- hash commitment,
- redacted presence,
- complete withholding.

Verification:

- disclosure policy tests,
- unauthorized field tests.

Exit criteria:

- sensitive fields can be withheld without making the explanation misleading.

### v0.39.0 - Federated Query Budgets

Goal: prevent recursive federation abuse.

Deliverables:

- maximum depth,
- maximum nodes,
- maximum response bytes,
- cycle detection,
- peer recursion policy.

Verification:

- cycle tests,
- amplification tests,
- over-budget query tests.

Exit criteria:

- federated WHY work is bounded and policy-controlled.

### v0.40.0 - Federated Explanation Bundle

Goal: return verifiable remote explanations.

Deliverables:

- remote signed facts,
- redacted remote edges,
- missing remote events,
- trust assumptions,
- verification summary.

Verification:

- two-peer fixture,
- redacted peer fixture,
- unreachable peer fixture.

Exit criteria:

- a verifier can distinguish local evidence, remote evidence, and unknowns.

## Phase 12: Native QUIC Transport

### v0.41.0 - QUIC Transport Decision

Goal: admit the initial QUIC provider and native transport shape.

Deliverables:

- provider review,
- ALPN string,
- frame family decision,
- dependency policy update,
- threat-model update.

Verification:

- dependency checks,
- docs and security review.

Exit criteria:

- native transport work has an admitted provider and documented security model.

### v0.42.0 - Native Frame Parser

Goal: parse native BCX frames with strict bounds.

Deliverables:

- frame header,
- frame type model,
- length checks,
- malformed frame tests.

Verification:

- parser tests,
- malformed corpus tests.

Exit criteria:

- native frames fail closed before invoking protocol logic.

### v0.43.0 - Native Invoke Flow

Goal: run invocation, admission, data, and effect over native streams.

Deliverables:

- invoke frame,
- admission frame,
- data frame,
- effect frame,
- stream state machine.

Verification:

- happy-path stream tests,
- wrong-order frame tests,
- cancelled stream tests.

Exit criteria:

- native peers can exchange one signed operation and receipt.

### v0.44.0 - No State-Changing 0-RTT

Goal: enforce replay-safe native transport behavior.

Deliverables:

- early-data policy,
- action classification,
- state-changing rejection,
- immutable-read exception tests if admitted.

Verification:

- 0-RTT rejection tests,
- replay fixture tests.

Exit criteria:

- consequential operations cannot execute as replayable early data.

## Phase 13: Sovereign Profile

### v0.45.0 - Sovereign Profile Rules

Goal: define strict high-assurance deployment behavior.

Deliverables:

- mandatory signatures,
- mutual peer authentication requirement,
- no bearer-only capability rule,
- no downgrade rule,
- receipt storage requirement.

Verification:

- profile conformance tests,
- negative config tests.

Exit criteria:

- Sovereign profile requirements are machine-checkable.

### v0.46.0 - Sovereign Verification Gate

Goal: enforce Sovereign profile decisions in verification.

Deliverables:

- profile-aware verifier,
- rejected unsigned invocation tests,
- rejected missing receipt tests,
- rejected downgraded transport tests.

Verification:

- `cargo test --workspace --all-features`
- Sovereign fixture suite.

Exit criteria:

- a Sovereign deployment fails closed when required evidence is missing.

### v0.47.0 - Remote Attestation Metadata

Goal: support attestation evidence without depending on one attestation stack.

Deliverables:

- attestation reference type,
- verifier result metadata,
- relying-party decision field,
- trust-assumption reporting.

Verification:

- attestation metadata tests,
- missing verifier result tests.

Exit criteria:

- BCX can carry attestation evidence as evidence, not as automatic truth.

## Phase 14: Provider Qualification And 1.0 Hardening

### v0.48.0 - Provider Qualification Hooks

Goal: prepare crypto, storage, and transport providers for review.

Deliverables:

- provider self-description trait,
- provider capability metadata,
- provider audit metadata,
- dependency review template.

Verification:

- provider fixture tests,
- docs review.

Exit criteria:

- providers can expose what they claim and what they do not claim.

### v0.49.0 - Interoperability Test Vectors

Goal: make independent implementations possible.

Deliverables:

- canonical encoding vectors,
- signature vectors,
- receipt vectors,
- WHY bundle vectors,
- tamper vectors.

Verification:

- all vectors verified by local test harness.

Exit criteria:

- another implementation can test compatibility without reading BCX internals.

### v0.50.0 - Security Documentation Freeze

Goal: close the 1.0 security documentation gap.

Deliverables:

- complete threat model,
- security controls,
- supply-chain security,
- unsafe policy,
- transport security notes,
- known limitations.

Verification:

- docs warnings denied,
- release metadata validator.

Exit criteria:

- security documentation matches the implemented behavior, not future intent.

### v0.51.0 - API Stabilization Candidate

Goal: freeze public API shape before 1.0.

Deliverables:

- public API review,
- deprecated experimental names removed or gated,
- feature matrix documented,
- migration notes from pre-1.0.

Verification:

- public API review checklist,
- full local gate.

Exit criteria:

- no known API rename is intentionally deferred past 1.0.

### v0.52.0 - Release Candidate

Goal: produce the final 1.0 candidate for pentest and integration testing.

Deliverables:

- release candidate notes,
- Fluxheim reference integration pass,
- HTTP compatibility pass,
- native QUIC pass,
- federated WHY pass,
- Sovereign profile pass.

Verification:

- `scripts/checks.sh`
- integration fixture suite,
- release readiness script after pentest report exists.

Exit criteria:

- all known release-blocking issues are fixed or the 1.0 release is postponed.

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
- pentest pass for the release.

Verification:

- `scripts/checks.sh`
- `cargo deny check`
- `cargo audit`
- `scripts/validate-release-readiness.sh v1.0.0`
- GitHub CI green
- GitHub CodeQL default setup green

Exit criteria:

- permanent pentest report has `Status: PASS`,
- no root `PENTEST.md`,
- no release-blocking findings,
- maintainer explicitly requests tag creation.
