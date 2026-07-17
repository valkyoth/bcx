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
- operation lifecycle tracking for reserved, authenticated, admitted,
  effect-pending, effect-observed, effect-receipted, failed, and indeterminate
  states,
- canonical operation key, one-statement binding, effect-attempt identifiers,
  deterministic transition table, transition authority, and atomic revision or
  transaction checks for lifecycle updates,
- append-only operation transition journal or authenticated transition log with
  derived current status,
- preserved effect evidence when later reorg, rollback, compensation,
  contradiction, or receipt invalidation evidence changes derived finality,
- profile-selected recovery model for replay/effect crash windows: atomic local
  transaction, durable journal/outbox, native carrier idempotency key, or
  external effect reconciliation,
- duplicate statement and nonce returning stored operation status or receipt
  without re-execution,
- deterministic duplicate response for every lifecycle state,
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
