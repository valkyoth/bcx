# BCX 0.5.0 Release Notes

Status: unreleased

## Summary

BCX 0.5.0 adds the first explicit causal edge model. The release keeps the
compact `CauseCapsule` API from 0.4.0 and adds per-parent edge semantics for
multi-parent graphs, offline explanation, and graph-aware cycle prevention.

## Added

- Add `CausalEdge`.
- Add `CausalEdgeSet`.
- Add `CausalEdgeSetParts`.
- Add `ParentStatus` with `Present` and `Missing` variants.
- Add `CausalCycleGuard` for caller-provided graph cycle checks.
- Add `NoKnownCycles` for contexts that only need local shape validation.
- Add multi-parent fixture tests.
- Add missing-parent representation tests.
- Add duplicate-parent, self-parent, too-many-parent, and cycle-guard
  rejection tests.

## Changed

- Re-export causal edge types from `bcx-model`.
- Re-export causal edge types from the root `bcx::prelude`.
- Move all current workspace packages to version `0.5.0`. `bcx-model` and
  root `bcx` changed code/API, and the other foundation crates carry packaged
  README header updates made after `v0.4.0`.
- Count crate README files as package content in the crate version matrix
  guard.
- Update the preferred pinned Rust toolchain to `1.96.1`.
- Expand release-gate compatibility checks to cover Rust `1.90.0` through
  `1.96.1`.
- Require every deferral in roadmap, release notes, pentest responses, and
  limitations to name the exact scheduled version or version range.

## Known Limitations

- No canonical encoding yet.
- No statement envelope yet.
- No full graph traversal crate yet.
- `CausalCycleGuard` is a hook; BCX does not store or traverse a complete graph
  in this release.
- Missing parents are represented; recovery and WHY bundle behavior are
  scheduled for `v0.26.0` (`bcx-explain` skeleton) and `v0.27.0`
  (explanation bundles).
