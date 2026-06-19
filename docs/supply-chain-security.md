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

`cargo-deny 0.19.9` uses `all`, `workspace`, `transitive`, and `none` as the
valid values for `unmaintained` and `unsound` advisory coverage. BCX uses
`all` so those advisory classes are checked across every dependency kind.

## Admitted Dependencies

| Crate | Version | Used by | Reason |
| --- | --- | --- | --- |
| `subtle` | `2.6.1` | `bcx-core` | Provides optimizer-resistant constant-time equality for fixed-width identifiers and nonces. |
| `zeroize` | `1.9.0` | `bcx-core` | Clears nonce memory on drop without adding local unsafe code. |
