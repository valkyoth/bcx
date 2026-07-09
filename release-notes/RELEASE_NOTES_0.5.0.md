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

## Known Limitations

- No canonical encoding yet.
- No statement envelope yet.
- No full graph traversal crate yet.
- `CausalCycleGuard` is a hook; BCX does not store or traverse a complete graph
  in this release.
- Missing parents are represented, but recovery and WHY bundle behavior remain
  future `bcx-explain` work.
