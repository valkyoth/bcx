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
- Crash windows between replay commitment, admission, native side effects, and
  effect receipts.
- Concurrent, skipped, backward, or unauthorized lifecycle transitions.
- Admission rejection mislabeled as effect failure.
- Duplicate delivery creating a new effect attempt.
- Retry amplification through unlimited effect attempts.
- Retrying after policy or revocation roots changed without rechecking
  admission authority.
- Retrying after trusted time, statement validity, key validity, capability,
  delegation, audience, scope, budget, recovery-rule, or completion-rule epoch
  changed without re-evaluating authority.
- Completion-rule substitution, weaker-rule downgrade, wrong policy epoch, or
  missing completion-rule evidence.
- Retry and reconciliation of an indeterminate attempt treated as the same
  operation.
- Admission-level indeterminate resolution jumping directly to effect
  execution.
- Effect-level indeterminate resolution manufacturing admission after the fact.
- Profile completion rules that overclaim finality or ignore pending attempts,
  compensation, reorg, or receipt invalidation.
- Effect-attempt state hiding operation-level evidence from another attempt.
- Reused, overflowed, or unbound effect-attempt identifiers.
- Portable lifecycle proof claims made from unauthenticated local journal data.
- Omitted, reordered, truncated, forked, or stale transition-history evidence
  presented as current lifecycle finality.
- Lost lifecycle history after effect evidence is reorged, contradicted,
  compensated, rolled back, or invalidated.
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
- Guaranteeing generic exactly-once execution across HTTP services, databases,
  blockchains, or other native carriers.
- Treating a mutable operation status alone as complete evidence history.
- Treating a portable transition event as the current lifecycle head without
  checkpoint, trusted receipt, witness, or equivalent profile head evidence.
- Treating `Completed` as irreversible rather than completion evidenced under
  a named profile policy and checkpoint or evaluation point.
- Evaluating completion without the canonical completion-rule ID, policy epoch,
  and committed rule preimage used by admission and completion evidence.

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
  unless local policy explicitly trusts that verifier role,
- missing effect evidence after admission proves that the effect failed or
  succeeded,
- subsequent reorg, rollback, compensation, contradiction, or receipt invalidation
  evidence erases the earlier observation,
- a failed latest attempt means an earlier observed or receipted attempt did
  not occur,
- a previously admitted operation authorizes new effect attempts after
  time-varying authority has expired or changed.

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
- operation lifecycle and recovery-model policy,
- append-only lifecycle journal and transition-authority policy,
- explicit attempt-creation, attempt-limit, and transition-event commitment
  policy,
- journal-head freshness and transition-chain completeness policy,
- completion-rule identity and retry-time authority re-evaluation policy,
- cacheability-by-outcome policy,
- unauthenticated source quota policy,
- threshold quorum-intersection policy.
