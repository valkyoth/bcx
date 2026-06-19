# BCX Profiles

Profiles define normative mappings from BCX objects to an underlying protocol
or evidence system.

Future examples:

- `bcx-http`
- `bcx-quic`
- `bcx-ethereum`
- `bcx-cardano`
- `bcx-offline`
- `bcx-scitt`
- `bcx-opentelemetry`
- possible future `bcx-bitcoin` or `bcx-xrp`

A profile crate is added only after its security contract states what native
fields are committed, what replay protection exists, what finality or
observation means, what data becomes public, and what the profile cannot prove.
