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
- GitHub Actions: `actions/checkout v7.0.0` pinned by commit SHA.
- Release publishing helper: `scripts/release_crate.py`.
- Latest-tool validation runs in version-specific release gates, not normal CI.

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

Normal CI uses `scripts/checks.sh` and must not depend on crates.io or GitHub
latest-version lookups beyond normal cargo dependency resolution. Before a tag,
run the matching release gate, such as `scripts/release_0_1_gate.sh`; that gate
checks latest pinned cargo tools and GitHub Actions pins before compatibility
checks across Rust `1.90.0` through `1.96.0`.
