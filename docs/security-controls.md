# BCX Security Controls

Status: planning document

Required controls from the first production profile:

- canonical statement signing for consequential operations,
- audience binding,
- nonce and expiry,
- issuer sequence numbers,
- atomic replay `check_and_record` with scoped nonce and sequence policy after
  signature, key, audience, and basic authority verification, or bounded
  reserve/verify/commit/abort semantics,
- failed authentication never permanently consumes replay state,
- separate operation and effect-attempt lifecycle tracking for reserved,
  authenticated, admitted, active, completed, reservation-expired, aborted,
  rejected, pending, observed, receipted, failed, and indeterminate states,
- canonical operation key, one-statement binding, effect-attempt identifiers,
  deterministic transition table, transition authority, and atomic revision or
  transaction checks for lifecycle updates,
- unique, non-reusable, overflow-safe `EffectAttemptId` allocation or
  derivation bound into every native binding and effect receipt,
- append-only operation transition journal or authenticated transition log with
  derived current status,
- authorized `start_attempt`, attempt-limit, and cumulative effect-work budget
  controls for retryable effect execution,
- admission-gated `start_attempt` with policy and revocation root rechecks
  before retry when roots changed since admission,
- distinct retry and reconciliation semantics for indeterminate attempts,
- phase-specific indeterminate resolution that prevents admission-level
  uncertainty from jumping directly to effect execution and prevents
  effect-level uncertainty from manufacturing admission,
- profile-bound completion rules with exact roots, evaluation point,
  finality/checkpoint policy, pending-attempt policy, and
  compensation/reorg/receipt-invalidation behavior,
- canonical completion-rule records and domain-separated completion-rule IDs
  bound into admission evidence, completion transitions, completion-relevant
  receipts, journal-head evidence, and verification cache keys,
- completion-rule policy epochs that prevent silent reinterpretation of
  historical completion evidence,
- unknown or unavailable completion rules producing indeterminate results and
  policy-forbidden completion rules failing admission,
- complete time-varying authority re-evaluation before every `start_attempt`,
  including trusted time, statement validity, key validity, capability and
  delegation validity, policy/trust/revocation/conflict roots, audience,
  operation scope, budgets, recovery rules, and completion-rule epoch,
- attempt-start transition records containing the authenticated evaluation
  point and roots used for retry authorization,
- canonical transition-event commitments before lifecycle history is exported
  as portable evidence,
- transition-chain commitments with previous-transition links, strict
  no-gap/no-duplicate sequences, journal-head commitments or Merkle roots, and
  checkpoint, trusted-receipt, or witness binding for exported lifecycle
  history,
- stale or incomplete lifecycle-history markers when the latest journal head
  cannot be established,
- preserved effect evidence when subsequent reorg, rollback, compensation,
  contradiction, or receipt invalidation evidence changes derived finality,
- profile-selected recovery model for replay/effect crash windows: atomic local
  transaction, durable journal/outbox, native carrier idempotency key, or
  external effect reconciliation,
- duplicate statement and nonce returning stored operation status or receipt
  without re-execution,
- deterministic duplicate response for every lifecycle state,
- duplicate delivery never creating a new effect attempt,
- same nonce with different statement commitment treated as conflict,
- no generic exactly-once execution claim across native carriers,
- proof-of-possession capabilities,
- canonical policy records and signed policy-evaluation evidence,
- immutable trust snapshots for key, policy, and revocation resolution,
- no silent downgrade,
- signer entropy policy and opaque private-key handles,
- production provider admission with provider assurance classes, secret
  zeroization, entropy-source health, fault-injection behavior, and external
  guarantee boundaries,
- admitted primitive providers treated as trusted computing base for
  cryptographic truth through `v1.0.0`,
- hybrid all-component acceptance with composite key lifecycle, epoch,
  revocation, expiry, and fail-closed downgrade rules,
- explicit native binding for each consequential carrier, ledger, or storage
  operation,
- bounded message sizes,
- checked aggregate decode budget before allocation, hashing, key lookup, or
  cryptographic verification,
- checked `VerificationBudget` and versioned `VerificationCostSchedule`,
- indeterminate resource-exhaustion outcomes that are not cached as invalid,
- cacheability matrix by outcome class with replay-store generation/state
  binding for replay results,
- locally unsupported recognized suites and temporarily unavailable providers
  separated from structurally invalid or policy-forbidden suites,
- verification receipts recording cost schedule, consumed units, completion
  state, signer role, roots, and policy epoch,
- verification receipts using a distinct receipt-signature domain and direct
  receipt verification path,
- sender-provided verification receipts accepted only through explicit local
  policy or re-execution,
- bounded WHY depth and node count,
- bounded unresolved-parent staging, orphan retention, fetch attempts,
  referenced bytes, unauthenticated source quotas, and authenticated per-issuer
  quotas only after issuer authentication,
- explicit distinction between declared, observed, verified, enforced,
  acknowledged, witnessed, settled, contradicted, and unknown or incomplete
  evaluation outcomes,
- hiding commitments for low-entropy private values,
- fail-closed unknown algorithm policy,
- checkpoint issuer, monotonic sequence, fork/equivocation, rollback, and
  consistency proof handling,
- exact witness quorum formula, overflow-safe arithmetic,
  quorum-intersection, and equivocation rules for threshold finality,
- provider admission for crypto, carrier, settlement, proof, and storage
  backends.

High-assurance deployments should additionally require:

- mutual peer authentication,
- no state-changing 0-RTT,
- append-only receipt storage,
- key rotation and revocation evidence,
- remote attestation where admitted,
- constant-time software or appropriately isolated hardware signing providers,
- independent witness commitments for critical events.
