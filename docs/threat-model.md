# BCX Threat Model

Status: planning document

BCX models verifiable causal accountability for consequential operations. Its
goal is to prove what participants observed, declared, verified, enforced,
acknowledged, and receipted. It does not prove a human or organization had an
honest internal motive.

## In Scope

- Forged causal parentage.
- Replay of valid operations.
- Tampering with signed invocation metadata.
- False broadening of delegated authority.
- Silent HTTP downgrade.
- Recursive WHY query abuse.
- Cross-domain disclosure mistakes.
- Confusing declared purpose with verified fact.
- Gateway claims about effects it cannot observe.
- Missing, redacted, contradictory, or unreachable evidence.

## Out Of Scope

- Proving internal human intent.
- Revoking information already copied into uncontrolled systems.
- Making a compromised endpoint honest.
- Replacing TLS, QUIC, OS access controls, HSMs, or database authorization.
- Proving legal compliance without reviewed policy and legal packs.
- Providing anonymity when deployments require real identity by policy.

## Security Claims

BCX may claim:

- this participant signed this declaration,
- this receiver observed this invocation,
- this policy digest governed this decision,
- this effect receipt was signed by this executor,
- this edge has a named assurance level.

BCX must not claim:

- the declared purpose was psychologically true,
- a gateway observed a database commit without integration evidence,
- a missing parent does not exist,
- a redacted explanation is complete,
- an HTTP wrapper is proof by itself.

## Privacy Risks

BCX can become a surveillance graph if implemented carelessly. The protocol must
support:

- no universal identity,
- pairwise event identifiers,
- hash commitments,
- selective disclosure,
- encrypted evidence,
- predicate proofs,
- bounded WHY queries,
- public anonymous reads outside consequential profiles.

## Abuse Controls

Every WHY query must be authenticated and bounded by:

- audit capability,
- query purpose,
- maximum depth,
- maximum nodes,
- maximum response bytes,
- disclosure policy,
- trust-domain rules.
