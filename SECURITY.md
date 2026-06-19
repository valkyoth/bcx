# Security Policy

BCX is security-sensitive protocol infrastructure. Treat changes to causal
semantics, canonical encoding, signature envelopes, capabilities, replay
handling, receipt validation, transport bindings, and WHY queries as high-risk
until tested.

## Routine Checks

Run these regularly and before releases:

```bash
scripts/checks.sh
cargo deny check
cargo audit
```

GitHub Actions run CI, and GitHub CodeQL default setup should be enabled in the
repository security settings. Do not add an advanced CodeQL workflow while
default setup is active.

## Dependency Policy

BCX starts with zero third-party runtime dependencies. New dependencies require
review, license checks, RustSec checks, tests, and documentation. Prefer small
optional subcrates when an ecosystem dependency would break the root crate's
no-std default.

Unknown registries and git sources are denied by `deny.toml`.

## Reporting

Do not publish exploitable security details before a fix is available. Open a
private security advisory or contact the maintainers directly once public
repository security channels are configured.
