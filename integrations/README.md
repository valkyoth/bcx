# BCX Integrations

Integrations connect BCX profiles to concrete libraries, frameworks, products,
or services. They are intentionally separate from profiles so the core and
normative mappings do not inherit large dependency trees.

Future examples:

- `bcx-http-hyper`
- `bcx-http-axum`
- `bcx-http-h3`
- `bcx-quic-quinn`
- `bcx-ethereum-alloy`
- `bcx-ethereum-contracts`
- `bcx-cardano-pallas`
- `bcx-skrifheim`
- `bcx-fluxheim`

Integrations may use `std`, async runtimes, network stacks, or framework
dependencies. The root `bcx` crate must not.
