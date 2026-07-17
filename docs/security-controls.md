# BCX Security Controls

Status: planning document

Required controls from the first production profile:

- canonical statement signing for consequential operations,
- audience binding,
- nonce and expiry,
- issuer sequence numbers,
- atomic replay `check_and_record` with scoped nonce and sequence policy,
- proof-of-possession capabilities,
- canonical policy records and signed policy-evaluation evidence,
- immutable trust snapshots for key, policy, and revocation resolution,
- no silent downgrade,
- signer entropy policy and opaque private-key handles,
- production provider admission for side-channel declarations, secret
  zeroization, entropy-source health, fault-injection behavior, and external
  guarantee boundaries,
- hybrid all-component acceptance with composite key lifecycle, epoch,
  revocation, expiry, and fail-closed downgrade rules,
- explicit native binding for each consequential carrier, ledger, or storage
  operation,
- bounded message sizes,
- checked aggregate decode budget before allocation, hashing, key lookup, or
  cryptographic verification,
- checked `VerificationBudget` and versioned `VerificationCostSchedule`,
- indeterminate resource-exhaustion outcomes that are not cached as invalid,
- verification receipts recording cost schedule, consumed units, and completion
  state,
- bounded WHY depth and node count,
- bounded unresolved-parent staging, orphan retention, fetch attempts, and
  referenced bytes,
- explicit distinction between declared, observed, verified, enforced,
  acknowledged, witnessed, settled, contradicted, and unknown or incomplete
  evaluation outcomes,
- hiding commitments for low-entropy private values,
- fail-closed unknown algorithm policy,
- checkpoint issuer, monotonic sequence, fork/equivocation, rollback, and
  consistency proof handling,
- witness quorum-intersection and equivocation rules for threshold finality,
- provider admission for crypto, carrier, settlement, proof, and storage
  backends.

High-assurance deployments should additionally require:

- mutual peer authentication,
- no state-changing 0-RTT,
- append-only receipt storage,
- key rotation and revocation evidence,
- remote attestation where admitted,
- independent witness commitments for critical events.
