# BCX Security Controls

Status: planning document

Required controls from the first production profile:

- canonical object signing for consequential invocations,
- audience binding,
- nonce and expiry,
- issuer sequence numbers,
- replay cache hook,
- proof-of-possession capabilities,
- no silent downgrade,
- bounded message sizes,
- bounded WHY depth and node count,
- explicit distinction between declared, observed, verified, enforced,
  acknowledged, witnessed, and unknown claims,
- fail-closed unknown algorithm policy,
- provider admission for crypto and transport backends.

High-assurance deployments should additionally require:

- mutual peer authentication,
- no state-changing 0-RTT,
- append-only receipt storage,
- key rotation and revocation evidence,
- remote attestation where admitted,
- independent witness commitments for critical events.
