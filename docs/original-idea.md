Yes—this is the strongest formulation of the idea so far

I would spell it Bifröst Causal Exchange—“causal,” because the protocol models why operations happened and how effects followed from them.

BCX would not be a replacement for HTTP, Ethereum, Cardano, QUIC, or any other system. It would be a semantic overlay protocol that can be bound to those systems.

The original HTTP-NG thought experiment mixed transport, wire encoding, authorization, execution, browsers, and identity into one new stack. The better idea is to extract the durable part—the signed contract, causality, authority, evidence, and receipts—and make that independent of the underlying technology.

The central rule becomes:

BCX defines what an operation means and how its history is proven. Other protocols carry, execute, store, or settle it.

Application or institution
            │
            ▼
    BCX semantic layer
 intent, authority, causality,
 decisions, effects, receipts
            │
            ▼
       BCX profile
 HTTP / QUIC / Ethereum /
 Cardano / Kafka / offline
            │
            ▼
     Underlying protocol

This is not without precedent as a design pattern. W3C Trace Context defines one context model that can be propagated using bindings for multiple protocols, including HTTP, AMQP, and MQTT. BCX would take the same general idea much further by making the propagated objects signed, policy-aware, causal, and independently verifiable.

What BCX itself would contain

BCX should contain only the concepts that remain true regardless of whether the operation travels through HTTP or settles on a blockchain.

The fundamental lifecycle would be:

Intent
   → Admission
   → Execution
   → Effect
   → Checkpoint
   → Explanation

A banking transfer, software deployment, AI-agent tool call, medical export, HTTP request, or Ethereum transaction could all participate in that lifecycle.

The central object model

I would separate three things that are often wrongly combined:

Statement
    What is being claimed?

Attestation
    Who signed or proved that claim?

Native binding
    Which HTTP request, Ethereum transaction,
    Cardano UTXO, or other event does it refer to?

Conceptually:

pub struct Statement<T> {
    pub version: ProtocolVersion,
    pub realm: RealmId,
    pub kind: StatementKind,

    pub subject: SubjectRef,
    pub audience: Option<SubjectRef>,

    pub parents: BoundedVec<CausalEdge, MAX_PARENTS>,
    pub policy_refs: BoundedVec<ObjectId, MAX_POLICIES>,

    pub validity: ValidityWindow,
    pub replay: ReplayPolicy,
    pub disclosure: DisclosurePolicy,

    pub payload: T,
}

pub struct Attestation {
    pub statement_id: StatementId,
    pub issuer: SubjectId,
    pub key_id: KeyId,
    pub proof_suite: ProofSuiteId,
    pub proof: BoundedBytes<MAX_PROOF_SIZE>,
}

pub struct NativeBinding<E> {
    pub statement_id: StatementId,
    pub profile: ProfileId,
    pub native_commitment: Digest,
    pub evidence: E,
}

The statement ID would be derived from the canonical statement bytes:

statement_id =
    HASH(
        "BCX-STATEMENT/1"
        || canonical_statement
    )

The signature is not necessarily part of the statement ID. That permits several institutions to attest to the same logical statement without producing different identities for it.

For example:

Statement S91:
    Bank A intends to transfer asset X under mandate M17.

Attestation A1:
    Bank A signed S91.

Attestation A2:
    Bank A's risk system signed approval of S91.

HTTP binding H1:
    S91 was carried in HTTP request R71.

Ethereum binding E1:
    S91 authorized Ethereum transaction 0x83...

Cardano binding C1:
    Checkpoint containing S91 was also anchored in UTXO 9f...

The logical statement remains S91 everywhere.

Core BCX statement types

BCX/1 should stay deliberately small.

pub enum StatementBody {
    Intent(Intent),
    Admission(Admission),
    Effect(Effect),
    Delegation(Delegation),
    Revocation(Revocation),
    Checkpoint(Checkpoint),
    Contradiction(Contradiction),
}
Intent

Describes what someone proposes:

action
target commitment
declared purpose
requested scope
authority or capability
expected effect
constraints
causal parents
Admission

Describes what the receiver decided:

accepted
rejected
narrowed
quarantined
additional approval required
policy applied
scope granted
obligations assumed
Effect

Describes an observation after execution:

response returned
state changed
object created
ledger committed
transaction finalized
data transferred
operation failed

An effect must state who observed it.

Fluxheim may truthfully say:

I received a 200 response from the upstream service.

It may not automatically say:

The upstream database permanently committed the transaction.

That stronger claim requires a database or application receipt.

Delegation

Describes a narrowing transfer of authority:

Bank A allowed Agent B to perform operation class C
with limit L until time T.

A child delegation may narrow authority but must not silently broaden it.

Revocation

Revokes:

key
credential
capability
mandate
policy version
attestation

Revocation should not erase history. It changes what remains valid.

Checkpoint

Commits a collection of statements, attestations, and bindings:

statement root
attestation root
binding root
policy root
revocation root
previous checkpoint

A checkpoint may be stored locally, threshold-signed, anchored on Ethereum, anchored on Cardano, registered through a transparency service, or all of those.

Contradiction

Conflicting claims should coexist:

Bank A:
    I sent request X.

Bank B:
    I never received request X.

BCX should preserve both statements and show their assurance levels. It should never overwrite one with the other.

Claims need explicit truth levels

BCX must not treat every signed statement as proven reality.

I would standardize assurance categories such as:

pub enum ClaimStatus {
    Declared,
    Observed,
    CryptographicallyVerified,
    PolicyEnforced,
    CounterpartyAcknowledged,
    IndependentlyWitnessed,
    SettlementFinalized,
    Contradicted,
    Unknown,
}

For example:

Purpose:
    "fraud prevention"

Status:
    Declared and signed by payments-service

Not proven:
    Whether fraud prevention was the actual business motivation

This distinction is essential. A signature proves authorship and integrity, not truth.

BCX profile categories

HTTP and Ethereum should not implement the same generic adapter interface because they perform different jobs.

BCX should define several extension roles.

Role	Purpose	Examples
Carrier	Moves BCX objects	HTTP, QUIC, Kafka, NATS
Binding	Binds a BCX statement to a native operation	HTTP request, Ethereum call, Cardano transaction
Observer	Produces evidence about native effects	web gateway, database SDK, chain indexer
Settlement	Anchors checkpoints and reports finality	Ethereum, Cardano, threshold ledger
Proof	Creates or verifies evidence	COSE, ZK proof, TEE attestation
Identity	Resolves keys and subjects	X.509, SPIFFE, Ethereum account
Storage	Preserves detailed evidence	Skrifheim, object store, offline bundle

This prevents a meaningless “one adapter trait for everything.”

Conceptual Rust interfaces could look like:

pub trait BindingProfile {
    const PROFILE_ID: ProfileId;

    type NativeOperation;
    type BindingEvidence;

    fn commit_native(
        operation: &Self::NativeOperation,
    ) -> Result<Digest, BindingError>;

    fn verify_binding(
        statement: &StatementId,
        evidence: &Self::BindingEvidence,
    ) -> Result<VerifiedBinding, BindingError>;
}

pub trait SettlementBackend {
    type Receipt;

    async fn anchor(
        &self,
        checkpoint: &SignedCheckpoint,
    ) -> Result<Self::Receipt, SettlementError>;

    async fn verify(
        &self,
        checkpoint: &SignedCheckpoint,
        receipt: &Self::Receipt,
    ) -> Result<FinalityStatus, SettlementError>;
}
The Rust crate ecosystem

The bcx crate should be a small façade over stable core crates. It should not pull Ethereum, Cardano, Tokio, HTTP, and ZK dependencies into every build.

bcx/
├── crates/
│   ├── bcx/
│   ├── bcx-types/
│   ├── bcx-codec/
│   ├── bcx-crypto/
│   ├── bcx-core/
│   ├── bcx-policy/
│   ├── bcx-state/
│   ├── bcx-explain/
│   ├── bcx-registry/
│   ├── bcx-conformance/
│   └── bcx-testkit/
│
├── profiles/
│   ├── bcx-http/
│   ├── bcx-quic/
│   ├── bcx-ethereum/
│   ├── bcx-cardano/
│   ├── bcx-offline/
│   ├── bcx-scitt/
│   └── bcx-opentelemetry/
│
├── integrations/
│   ├── bcx-http-hyper/
│   ├── bcx-http-axum/
│   ├── bcx-http-h3/
│   ├── bcx-quic-quinn/
│   ├── bcx-ethereum-alloy/
│   ├── bcx-ethereum-contracts/
│   ├── bcx-cardano-pallas/
│   ├── bcx-skrifheim/
│   └── bcx-fluxheim/
│
├── proofs/
│   ├── bcx-proof-cose/
│   ├── bcx-proof-sp1/
│   ├── bcx-proof-risc0/
│   └── bcx-proof-threshold/
│
├── domains/
│   ├── bcx-profile-banking/
│   ├── bcx-profile-ai-agent/
│   ├── bcx-profile-supply-chain/
│   └── bcx-profile-healthcare/
│
└── services/
    ├── bcx-node/
    ├── bcx-witness/
    ├── bcx-prover/
    ├── bcx-query/
    └── bcx-cli/

Cargo workspaces are well suited to a set of separately published packages that share development and testing infrastructure.

bcx

The developer-friendly façade:

use bcx::{
    IntentBuilder,
    StatementId,
    Attestation,
    ExplanationBundle,
};

It should depend on the stable protocol core only.

bcx-types

Consensus-visible types.

Preferably:

#![no_std]

No networking, async runtime, database, HTTP, or blockchain types.

bcx-codec

Defines the canonical bytes used for:

object IDs
signatures
Merkle trees
settlement commitments
proof inputs
test vectors

I would make deterministic CBOR the first mandatory binary profile and use COSE for the first standard signing/encryption profile. CBOR is designed for compact, extensible binary messages, while COSE defines signatures, MACs, encryption, and key representations around CBOR.

JSON could be used for inspection and debugging, but never as the canonical signed representation.

bcx-core

Implements:

statement validation
causal-link validation
audience and realm binding
expiry
replay rules
delegation narrowing
attestation verification
extension handling
bcx-state

The optional deterministic World state machine:

pub fn execute_batch<S: StateBackend>(
    state: &mut S,
    context: &BatchContext,
    batch: &Batch,
) -> Result<TransitionOutput, TransitionError>;

This is used when BCX operates as a rollup, consortium ledger, or synchronized causal state.

Ordinary BCX-over-HTTP usage need not run the full state machine.

bcx-explain

Implements:

why queries
causal traversal
selective disclosure
unknowns
contradictions
proof-bundle verification
bcx-conformance

Contains mandatory interoperability vectors:

canonical encoding vectors
signature vectors
replay cases
unknown-extension cases
binding mutation cases
multi-parent causality
partial disclosure
cross-platform checkpoint tests
bcx-http

bcx-http would define the normative mapping between a BCX statement and an HTTP exchange.

It could support two modes.

Attached mode

An existing request remains recognizable:

POST /payments
Content-Type: application/json
Content-Digest: ...
BCX-Context: ...
BCX-Statement: ...

The BCX native commitment would cover:

HTTP method
scheme
authority
target
selected request fields
content digest
BCX statement ID
expiry

HTTP Message Signatures already define canonical signing of selected HTTP components and account for legitimate transformations by intermediaries. BCX HTTP can reuse that machinery or bind a separately signed BCX envelope to the same components. RFC 9421 also makes clear that message signatures are a tool within a larger application security model, not a complete security system by themselves.

Encapsulated mode

The complete exchange is a BCX object:

POST /.well-known/bcx/exchange
Content-Type: application/bcx+cbor

<signed BCX envelope>

The response contains:

Admission
Effect receipt
or asynchronous receipt reference
HTTP integration crates
bcx-http
    protocol mapping only

bcx-http-hyper
    Hyper request and response integration

bcx-http-axum
    Axum extractors and middleware

bcx-http-h3
    HTTP/3-specific integration

bcx-fluxheim
    Fluxheim policy, routing and receipt integration

The same bcx-http semantics can work over HTTP/1.1, HTTP/2, or HTTP/3. HTTP/3 is the carrier version, not a different BCX protocol.

bcx-quic

This would be the native peer protocol for environments that do not need HTTP semantics.

ALPN: bcx/1

Possible frames:

HELLO
PROFILE
STATEMENT
ATTESTATION
ADMISSION
EFFECT
WHY
EXPLANATION
CHECKPOINT
REVOCATION
CLOSE

Native QUIC would be attractive for:

Fluxheim-to-Fluxheim communication
high-security institutional networks
military systems
controlled AI-agent systems
long-lived federated services

The signed BCX objects would remain identical to those carried through HTTP.

bcx-ethereum

This should be a façade for several Ethereum-specific functions.

bcx-ethereum
├── binding
├── settlement
├── observation
├── proof verification
└── action gating
Ethereum native binding

A BCX statement could be bound to:

chain ID
contract address
function selector
calldata digest
ETH value
authorized sender
expiry
nullifier
statement ID

Conceptually:

native_commitment =
    keccak256(
        "BCX-ETHEREUM/1"
        || chain_id
        || contract
        || calldata_hash
        || value
        || statement_id
        || expiry
        || nullifier
    )
Ethereum settlement

A contract could anchor:

checkpoint ID
previous state root
next state root
statement root
policy root
revocation root
proof program ID

A basic implementation that only posts roots is an Ethereum anchoring profile.

It becomes a genuine Ethereum L2 or validium-like system only when Ethereum contracts control canonical state transitions, verify proofs, and participate in the system’s settlement and availability model. ZK rollups execute state transitions offchain and submit state summaries and validity proofs to Ethereum; validiums use similar validity proofs while retaining transaction data outside Ethereum.

Ethereum crates
bcx-ethereum
bcx-ethereum-alloy
bcx-ethereum-contracts
bcx-ethereum-settlement
bcx-ethereum-indexer
bcx-ethereum-gate
bcx-ethereum-rollup

Alloy would be a natural Rust integration layer for JSON-RPC, EVM primitives, transactions, and generated contract bindings.

bcx-cardano

The Cardano profile would preserve the same BCX statement and checkpoint IDs but map them into Cardano-native structures.

A checkpoint could be represented in a state UTXO:

Current BCX state UTXO:
    sequence
    current state root
    policy root
    verifier version

Next transaction:
    consumes current state UTXO
    validates BCX checkpoint
    creates next state UTXO

Cardano’s EUTXO model is fundamentally different from Ethereum’s account and contract-storage model: outputs carry state, scripts determine whether outputs may be consumed, and a transaction consumes old outputs to create new ones. Therefore bcx-cardano should implement BCX invariants natively rather than imitating the Ethereum contract design.

Possible packages:

bcx-cardano
bcx-cardano-pallas
bcx-cardano-settlement
bcx-cardano-indexer
bcx-cardano-validator

The offchain client can be Rust using Pallas, which provides Rust-native Cardano ledger, networking, transaction, and indexing building blocks. The onchain validator would likely use Aiken or Plutus rather than Rust.

The same checkpoint can use several settlement profiles

BCX should not require one blockchain.

Checkpoint C918
    statement root: R1
    policy root: R2
    revocation root: R3

Settlement receipt:
    Ethereum transaction E71

Settlement receipt:
    Cardano transaction C42

Settlement receipt:
    consortium threshold signature T19

There is one BCX checkpoint and several settlement receipts.

A realm could define:

pub enum SettlementPolicy {
    Primary(SettlementBackendId),

    PrimaryWithWitnesses {
        primary: SettlementBackendId,
        witnesses: Vec<SettlementBackendId>,
    },

    RequireAll(Vec<SettlementBackendId>),

    Threshold {
        required: u16,
        backends: Vec<SettlementBackendId>,
    },
}

The underlying networks may have different finality semantics, so BCX should normalize status without discarding native evidence:

pub enum FinalityStatus {
    NotSubmitted,
    Submitted,
    Included,
    Confirmed,
    Finalized,
    Challenged,
    Reverted,
    Unavailable,
}
Domain profiles should be separate from protocol bindings

This is another important separation.

bcx-http:
    how BCX maps to HTTP

bcx-ethereum:
    how BCX maps to Ethereum

bcx-profile-banking:
    what mandates, approvals and settlement effects
    mean in banking

bcx-profile-ai-agent:
    what agent delegation, model identity and tool
    authorization mean

A deployment could combine:

bcx = "0.1"
bcx-http = "0.1"
bcx-ethereum = "0.1"
bcx-profile-banking = "0.1"

Another could use:

bcx = "0.1"
bcx-quic = "0.1"
bcx-skrifheim = "0.1"
bcx-profile-ai-agent = "0.1"

That makes BCX useful beyond blockchains without reducing everything to the lowest common denominator.

Normative specifications versus crates

The crates are implementations. The protocol specifications must exist separately.

I would define:

BCX-CORE/1
    common statements, attestations and causality

BCX-CODEC-CBOR/1
    canonical binary representation

BCX-PROOF-COSE/1
    standard signatures and encryption

BCX-HTTP/1
    HTTP binding

BCX-QUIC/1
    native QUIC exchange

BCX-ETHEREUM/1
    Ethereum binding and settlement profile

BCX-CARDANO/1
    Cardano binding and settlement profile

BCX-WHY/1
    explanation queries and proof bundles

BCX-OFFLINE/1
    air-gapped bundle format

BCX-ZK/1
    validity-proof public inputs

BCX-REGISTRY/1
    type, profile and algorithm registries

Crate and protocol versions must not be confused:

bcx-http crate 0.8.2
    implements BCX-HTTP/1

That permits bug fixes and API changes without changing the wire protocol.

Every deep profile needs a security contract

A BCX profile should be required to document:

What exactly is committed?

Which native fields are authenticated?

How is replay prevented?

What does finality mean?

What can intermediaries alter?

What information is public?

What can the adapter truthfully observe?

How are downgrades prevented?

What happens when native evidence disappears?

How are unknown extensions handled?

For example:

bcx-http can prove:
    this signed BCX statement was bound to this HTTP request

bcx-http cannot prove:
    the application permanently committed a database update

bcx-ethereum can prove:
    this transaction was included and finalized under
    the configured Ethereum finality policy

bcx-ethereum cannot prove:
    the human's declared business purpose was honest

bcx-cardano can prove:
    this UTXO was consumed and a specified output was created

bcx-cardano cannot prove:
    an offchain document contained truthful information
BCX should integrate with existing standards

BCX should reuse standards where they already solve part of the problem:

CBOR:
    canonical compact objects

COSE:
    signatures and encryption

HTTP Message Signatures:
    HTTP component binding

W3C Trace Context:
    operational correlation

SCITT:
    transparency-service integration

OpenTelemetry:
    observability bridge

SCITT is especially adjacent: its architecture concerns signed statements, transparency services, interoperability, and auditing. BCX would add the live causal lifecycle—intent, admission, effect, delegation, contradiction, and WHY—while potentially using a SCITT transparency service as one checkpoint or witness backend. The current SCITT architecture is in the RFC publication process as of May 2026.

BCX should not replace OpenTelemetry. bcx-opentelemetry could associate BCX statement IDs with trace and span IDs, but an observability trace remains operational telemetry rather than cryptographic evidence.

A complete cross-profile example

Suppose a bank customer requests a transfer.

1. HTTP arrival
Intent S100:
    transfer €1 million
    Bank A → Bank B
    purpose: commercial settlement
    authority: mandate M71

bcx-http binds S100 to:

POST /institutional-transfers
request body digest
Bank A authority
2. Admission

Bank B creates:

Admission S101:
    parent: S100
    accepted under policy B-PAY-19
    scope: exact beneficiary and amount
3. Ethereum execution

bcx-ethereum binds S100 and S101 to an exact contract call.

The assurance gate verifies the authorization and consumes its nullifier.

4. Effects
Effect S102:
    Ethereum transaction finalized

Effect S103:
    Bank A core ledger debited

Effect S104:
    Bank B core ledger credited

Each effect is issued by the component that observed it.

5. Checkpoint
Checkpoint C20:
    includes S100-S104
6. Multiple settlement receipts
Ethereum:
    C20 anchored in transaction E1

Cardano:
    C20 anchored in transaction C1

Skrifheim:
    private evidence retained
7. Explanation
bcx why effect:S104 --view regulator

The result can show:

declared purpose
mandate
approvals
receiver admission
Ethereum settlement
both ledger receipts
policy versions
revocations checked
redacted evidence
unknown or unproven claims

That is one logical causal graph spanning HTTP, two institutions, Ethereum, Cardano, and private storage.

Rules that keep BCX from becoming another giant monolith

The architecture only works if several boundaries remain strict:

The core never depends on HTTP, Ethereum, or Cardano types.
Profiles depend on the core; the core never depends on profiles.
The same statement ID survives movement between profiles.
Every native operation receives an explicit cryptographic binding.
No profile may silently downgrade to unsigned or unaudited operation.
Private causal details remain off public blockchains by default.
An adapter may only attest to effects it can actually observe.
Unknown, redacted, missing, and contradictory evidence remain visible.
A blockchain is optional; BCX must still function bilaterally and offline.
The core remains small while innovation happens in profiles.

The most important design slogan would be:

The BCX core should be boring. The BCX ecosystem can be ambitious.

Recommended first implementation

I would not start with the Ethereum or Cardano rollup.

Release 0.1

Build:

bcx
bcx-types
bcx-codec
bcx-crypto
bcx-core
bcx-explain
bcx-offline
bcx-testkit
bcx-cli

Demonstrate signed intent, admission, effect, contradiction, and offline WHY.

Release 0.2

Add:

bcx-http
bcx-http-hyper
bcx-fluxheim

Run two Fluxheim servers and trace one operation across them.

Release 0.3

Add:

bcx-ethereum
bcx-ethereum-alloy
bcx-ethereum-contracts

Anchor the checkpoint and gate one mock banking transfer.

Release 0.4

Add:

bcx-cardano
bcx-cardano-pallas
bcx-cardano-validator

Anchor the same checkpoint on Cardano.

Release 0.5

Demonstrate:

bcx verify checkpoint:C20 \
    --ethereum-receipt eth.json \
    --cardano-receipt cardano.json

That demo would prove the central proposition:

BCX is not an Ethereum protocol or an HTTP extension. It is one causal assurance protocol with several deep native bindings.

Naming note

The BCX acronym is strong, and the Bifröst metaphor fits a protocol connecting otherwise separate technical “worlds.” However, “Bifrost” is already used by several blockchain and Rust communication projects, including an existing Rust crate and blockchain interoperability networks. Treat the name as a good internal name until you complete registry, domain, and trademark checks.

Final position

What you are describing is no longer primarily:

a future HTTP protocol

or:

an Ethereum L2

It is:

A universal, proof-carrying causal exchange layer that can travel over existing protocols, bind to native operations, preserve evidence privately, and settle checkpoints wherever a deployment chooses.

The ecosystem would look like:

bcx
    invariant semantics

bcx-http
    web compatibility

bcx-quic
    native secure exchange

bcx-ethereum
    Ethereum binding, enforcement and settlement

bcx-cardano
    Cardano binding, validation and settlement

bcx-skrifheim
    private evidence and World storage

bcx-fluxheim
    live gateway enforcement and causality

bcx-profile-banking
    institutional mandate semantics

bcx-profile-ai-agent
    delegated machine-action semantics

That is a substantially bigger and more coherent idea than reinventing HTTP or building one blockchain-specific application.
