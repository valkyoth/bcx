# BCX Unsafe Policy

Status: active policy

Unsafe Rust is forbidden in the current BCX workspace.

If unsafe ever becomes necessary:

- isolate it in a dedicated boundary crate,
- document the reason before implementation,
- add a crate-level safety document,
- require `SAFETY:` comments for every unsafe block,
- add tests that exercise the boundary,
- keep the safe facade separate from the unsafe implementation.

The root workspace lint currently sets `unsafe_code = "forbid"`.
