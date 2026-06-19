# BCX Toolchain Policy

Status: active policy

BCX uses stable Rust.

Baseline:

- Preferred Rust: `1.96.0`
- MSRV: `1.90.0`
- Edition: `2024`
- Resolver: `3`
- Default build: `no_std`
- CI security tools: `cargo-deny 0.19.9`, `cargo-audit 0.22.2`

Compatibility target:

| Rust | Expected gate |
| --- | --- |
| `1.90.0` | full release gate |
| `1.91.0` | `cargo check --all-features` |
| `1.92.0` | `cargo check --all-features` |
| `1.93.0` | `cargo check --all-features` |
| `1.94.0` | `cargo check --all-features` |
| `1.95.0` | `cargo check --all-features` |
| `1.96.0` | full release gate |

Before changing Rust versions, dependency versions, CI actions, or security
tools, re-check current upstream status and document the reason in release
notes.
