# BCX Foundation Crates

These crates are the no-std foundation of BCX. They are published separately
and should remain small.

Current crates:

- `bcx-core`
- `bcx-model`
- `bcx-crypto`
- `bcx-policy`
- `bcx-wire`

Future core crates such as `bcx-codec`, `bcx-state`, `bcx-explain`,
`bcx-registry`, `bcx-conformance`, and `bcx-testkit` should be added here only
when their release milestone is reached.

Profiles, integrations, proof providers, domain profiles, and services belong
in their top-level family directories, not in the core crate set.
