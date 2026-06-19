# BCX Proof Crates

Proof crates implement or integrate concrete proof suites. The core owns proof
semantics and verification boundaries; proof crates own provider-specific
logic.

Future examples:

- `bcx-proof-cose`
- `bcx-proof-threshold`
- `bcx-proof-sp1`
- `bcx-proof-risc0`

No proof crate may silently broaden what a statement claims. A proof verifies a
specific statement, attestation, binding, checkpoint, or policy claim.
