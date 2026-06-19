# BCX Protocol Family

Status: architecture note

BCX is one invariant causal protocol with many possible native bindings. The
core should be boring, dependency-light, no-std by default, and stable. The
ecosystem around it can be broad.

## Layer Boundaries

```text
Domain profile
    banking, AI agent, healthcare, supply chain

Integration
    Fluxheim, Skrifheim, Hyper, Axum, Alloy, Pallas

Profile
    HTTP, QUIC, Ethereum, Cardano, offline, SCITT, OpenTelemetry

Core
    statements, attestations, bindings, checkpoints, policy, verification
```

The dependency direction is always downward. Core crates never depend on
profiles, integrations, services, runtimes, networks, or blockchains.

## Core Responsibilities

The core defines:

- statement identity,
- causal parentage,
- claim status and assurance,
- attestations,
- native binding envelopes,
- checkpoints,
- replay and delegation policy vocabulary,
- disclosure and settlement policy vocabulary,
- verification boundaries.

The core does not define:

- HTTP request extraction,
- Ethereum RPC,
- Cardano indexing,
- Fluxheim routing,
- Skrifheim storage,
- CLI parsing,
- async runtime behavior.

## Profile Responsibilities

A profile maps BCX objects into one underlying system. Each profile must state
what it commits and what it can truthfully prove.

Examples:

- `bcx-http` binds statements to HTTP components.
- `bcx-ethereum` binds statements or checkpoints to Ethereum transactions and
  finality evidence.
- `bcx-cardano` binds checkpoints to EUTXO transitions.
- `bcx-offline` binds statements and evidence to an air-gapped bundle.

## Integration Responsibilities

An integration makes a profile usable with a real implementation stack.

Examples:

- `bcx-http-hyper` adapts `bcx-http` to Hyper request and response types.
- `bcx-ethereum-alloy` adapts `bcx-ethereum` to Alloy primitives and RPC.
- `bcx-fluxheim` adapts BCX receipts and WHY bundles to Fluxheim behavior.

## Extension Rule

New systems such as Bitcoin, XRP, future ledgers, specialized databases, or
institutional networks should start as profile security contracts. A crate is
added only after the contract defines:

- committed native fields,
- replay prevention,
- finality or observation semantics,
- public/private data exposure,
- downgrade behavior,
- evidence retention assumptions,
- unknown extension handling.

## Naming Rule

Published crates keep the `bcx-*` prefix. Protocol specs use names such as
`BCX-CORE/1`, `BCX-HTTP/1`, or `BCX-ETHEREUM/1`. Crate versions and protocol
versions are not the same thing.
