# BCX Supply-Chain Security

Status: active policy

The default BCX crates must remain zero-third-party-dependency and no-std until
a dependency is explicitly admitted.

Before admitting a dependency:

- check the latest crate version,
- review license compatibility with `EUPL-1.2`,
- review build scripts and proc macros,
- check RustSec advisories,
- check maintenance status,
- document why local code or a smaller subcrate is not better,
- add targeted tests for the dependency boundary.

Unknown registries and unknown git sources are denied by `deny.toml`.

## Admitted Dependencies

| Crate | Version | Used by | Reason |
| --- | --- | --- | --- |
| `zeroize` | `1.9.0` | `bcx-core` | Clears nonce memory on drop without adding local unsafe code. |
