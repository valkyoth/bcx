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
- explicit native binding for each consequential carrier, ledger, or storage
  operation,
- bounded message sizes,
- checked aggregate decode budget before allocation, hashing, key lookup, or
  cryptographic verification,
- bounded WHY depth and node count,
- bounded unresolved-parent staging, orphan retention, fetch attempts, and
  referenced bytes,
- explicit distinction between declared, observed, verified, enforced,
  acknowledged, witnessed, settled, contradicted, and unknown claims,
- fail-closed unknown algorithm policy,
- checkpoint issuer, monotonic sequence, fork/equivocation, rollback, and
  consistency proof handling,
- provider admission for crypto, carrier, settlement, proof, and storage
  backends.

High-assurance deployments should additionally require:

- mutual peer authentication,
- no state-changing 0-RTT,
- append-only receipt storage,
- key rotation and revocation evidence,
- remote attestation where admitted,
- independent witness commitments for critical events.
