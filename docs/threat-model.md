# BCX Threat Model

Status: planning document

BCX models verifiable causal accountability for consequential operations across
many carriers, ledgers, storage systems, and offline bundles. Its goal is to
prove what participants observed, declared, verified, enforced, acknowledged,
settled, and receipted. It does not prove a human or organization had an honest
internal motive.

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
- Native binding confusion between HTTP, blockchain, storage, and offline
  evidence.
- Settlement finality overclaims.
- Cross-profile downgrade or replay.
- Provider format errors, crashes, resource abuse, and capability
  misreporting.
- Compromised admitted providers as trusted-boundary failures.
- Resource-amplification inputs that force expensive verification paths.
- Threshold witness equivocation or disjoint-quorum finality claims.
- Forged or replayed verification receipts.
- Replay-store poisoning before authentication.
- Stale contextual verification-cache reuse.
- Unauthenticated orphan-quota bypass.

## Out Of Scope

- Proving internal human intent.
- Revoking information already copied into uncontrolled systems.
- Making a compromised endpoint honest.
- Replacing TLS, QUIC, OS access controls, HSMs, or database authorization.
- Proving legal compliance without reviewed policy and legal packs.
- Providing anonymity when deployments require real identity by policy.
- Replacing Ethereum, Cardano, Bitcoin, XRP, SCITT, OpenTelemetry, or other
  underlying systems.
- Proving that an underlying chain or service is honest beyond its configured
  finality and evidence model.
- Tolerating arbitrary false cryptographic success from a compromised admitted
  primitive provider through `v1.0.0`.

## Security Claims

BCX may claim:

- this participant signed this declaration,
- this receiver observed this invocation,
- an authorized evaluator attested that this policy digest governed this
  decision,
- a reproducible or proven policy-decision mode demonstrates stronger
  evaluation evidence,
- this effect receipt was signed by this executor,
- this checkpoint was witnessed or settled under a named profile policy,
- this edge has a named assurance level.

BCX must not claim:

- the declared purpose was psychologically true,
- a gateway observed a database commit without integration evidence,
- an Ethereum, Cardano, Bitcoin, XRP, or other settlement receipt proves an
  offchain business purpose was honest,
- a missing parent does not exist,
- a redacted explanation is complete,
- an HTTP wrapper or blockchain transaction is proof by itself,
- a sender-provided verification receipt suppresses required local verification
  unless local policy explicitly trusts that verifier role.

## Privacy Risks

BCX can become a surveillance graph if implemented carelessly. The protocol must
support:

- no universal identity,
- pairwise event identifiers,
- hiding commitments for low-entropy private values,
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
- trust-domain rules,
- verification budget and cost-schedule rules,
- provider assurance policy,
- receipt replay boundaries,
- replay-store authentication-before-commit policy,
- cacheability-by-outcome policy,
- unauthenticated source quota policy,
- threshold quorum-intersection policy.
