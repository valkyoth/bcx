# BCX Crate Version Matrix

Status: active release metadata

BCX will eventually contain many crates. They must not all be published for
every milestone unless their package contents changed. Crate package versions
are tracked separately from protocol specification versions such as
`BCX-CORE/1` or `BCX-HTTP/1`.

Update this table whenever a crate package version changes or a new workspace
crate is added. The local release metadata gate verifies that the table matches
Cargo metadata.

| Crate | Manifest path | Package version | Protocol/API line | Publish policy |
| --- | --- | --- | --- | --- |
| `bcx` | `.` | `0.3.0` | facade API | publish when facade exports, docs embedded in the crate, or dependency pins change |
| `bcx-core` | `crates/bcx-core` | `0.3.0` | BCX-CORE foundation | publish when identifiers, digest/nonce semantics, validation errors, or core dependency policy change |
| `bcx-crypto` | `crates/bcx-crypto` | `0.3.0` | BCX proof-envelope foundation | publish when signature envelope, verifier traits, algorithm policy, or crypto boundary semantics change |
| `bcx-model` | `crates/bcx-model` | `0.3.0` | BCX statement/model foundation | publish when causal model, admission/effect vocabulary, truth, or assurance semantics change |
| `bcx-policy` | `crates/bcx-policy` | `0.2.0` | BCX policy foundation | publish when profile, proof-level, disclosure, replay, or settlement policy vocabulary changes |
| `bcx-wire` | `crates/bcx-wire` | `0.3.0` | BCX wire-limit foundation | publish when protocol version, message bounds, or profile-carried header behavior changes |

## Rules

- A crate with source or manifest changes after the latest release tag must
  bump its package version before the next release.
- A crate without package-content changes should usually keep its previous
  package version and should not be republished.
- Path dependency `version = "..."`
  requirements must match the referenced local package version.
- If a crate package version changes, every published workspace crate with a
  local path dependency on it must update that dependency pin and bump its own
  package version. Repeat this through the local dependency graph until no
  dependent crate needs a changed pin.
- The root `bcx` facade may move more often than leaf crates because it exposes
  dependency pins and user-facing re-exports.
- Protocol spec versions are independent. A crate can publish several patch
  versions while still implementing the same protocol line.

Run:

```bash
scripts/validate-crate-version-matrix.py
```
