# Signature Verification Documentation

This document outlines the relevant code across various modules that participate in the Signature Verification stage in the Agave repository.

## Key Signature Verification Components

### perf/src/sigverify.rs

- **Role**: Core implementation of digital signature verification functions.
- **Key Functions**:
  - **Function: `verify_packet`**
    - **Description**: Verifies the signature(s) on a single packet.
    - **Inputs**: 
      - `packet: &mut Packet` - The packet containing transaction data.
      - `reject_non_vote: bool` - Flag to reject non-vote transactions.
    - **Outputs**: `bool` - Returns true if signatures are valid.
  - **Function: `ed25519_verify`**
    - **Description**: Verifies ED25519 signatures for batches of packets, with GPU acceleration if available.
    - **Inputs**: 
      - `batches: &mut [PacketBatch]` - Batches of packets to verify.
      - `recycler: &Recycler<TxOffset>` - Memory recycler for transaction offsets.
      - `recycler_out: &Recycler<PinnedVec<u8>>` - Memory recycler for verification results.
      - `reject_non_vote: bool` - Flag to reject non-vote transactions.
      - `valid_packet_count: usize` - Count of valid packets.
    - **Outputs**: None (marks packets as discard if invalid)
  - **Function: `generate_offsets`**
    - **Description**: Generates offset vectors for optimized batch verification.
    - **Inputs**:
      - `batches: &mut [PacketBatch]` - Batches to generate offsets for.
      - `recycler: &Recycler<TxOffset>` - Memory recycler.
      - `reject_non_vote: bool` - Flag to reject non-vote transactions.
    - **Outputs**: `TxOffsets` - Tuple of offset vectors for batch verification.

### core/src/sigverify_stage.rs

- **Role**: Implements the signature verification stage of the TPU (Transaction Processing Unit), which processes incoming transaction packets and verifies their signatures.
- **Key Components**:
  - **Struct: `SigVerifyStage`**
    - Manages a thread that receives packets, verifies their signatures, and forwards them to the next stage.
  - **Trait: `SigVerifier`**
    - Interface for signature verification implementers.
    - Key methods:
      - `verify_batches` - Verifies signatures on batches of packets.
      - `send_packets` - Sends verified packets to the next stage.
  - **Function: `verifier_service`**
    - **Description**: Runs a loop that receives packet batches, performs deduplication, verifies signatures, and forwards valid packets.
    - **Key Operations**:
      - Packet deduplication to eliminate redundant verification.
      - Batch signature verification (leveraging GPU when available).
      - Metrics collection for performance monitoring.

### turbine/src/sigverify_shreds.rs

- **Role**: Verifies signatures on shreds (fragments of the ledger) received from the network.
- **Key Functions**:
  - **Function: `spawn_shred_sigverify`**
    - **Description**: Spawns a thread to verify signatures on shreds before forwarding them.
    - **Inputs**: 
      - Cluster info, bank forks, leader schedule cache, receivers and senders.
    - **Outputs**: Thread handle for the verification service.
  - **Function: `verify_packets`**
    - **Description**: Verifies a batch of packets containing shreds.
    - **Inputs**: 
      - Thread pool, self public key, bank references, and packet batches.
    - **Outputs**: None (marks packets as discard if invalid)
  - **Function: `verify_retransmitter_signature`**
    - **Description**: Verifies the signature of a shred's retransmitter.
    - **Inputs**: 
      - Shred data, bank info, cluster info, and caches.
    - **Outputs**: `bool` - Whether the retransmitter signature is valid.

### core/src/sigverify.rs

- **Role**: Additional signature verification utilities specific to the core module.
- **Key Functions**:
  - Wrapper functions around the lower-level `perf/src/sigverify.rs` functions.
  - Integration with the banking stage for efficient verification.

### core/src/banking_stage/banking_stage.rs

- **Role**: Processes verified transactions, with integration with the signature verification stage.
- **Key Functions**:
  - Consumes verified transaction packets from the sigverify stage.
  - Manages parallel execution of transactions after signature verification.

## Cost Model

### cost-model/src/block_cost_limits.rs

- **Role**: Defines compute unit costs for different signature verification operations.
- **Key Constants**:
  - `SIGNATURE_COST` - Compute units for one signature verification.
  - `SECP256K1_COST` - Compute units for one secp256k1 signature verification.
  - `ED25519_COST` - Compute units for one ed25519 signature verification.
  - `ED25519_STRICT_COST` - Compute units for one ed25519 strict signature verification.
  - `SECP256R1_COST` - Compute units for one secp256r1 signature verification.

## RPC Interface

### rpc/src/rpc.rs

- **Role**: Provides JSON-RPC interface for transaction simulation with signature verification options.
- **Key Options**:
  - `sigVerify` parameter for the `simulateTransaction` RPC method to control whether signatures are verified during simulation.

## Summary

Signature verification in Agave is a multi-layered process:

1. The `sigverify_stage` receives transaction packets and verifies their signatures, optionally using GPU acceleration.
2. For shreds (ledger fragments), `sigverify_shreds` handles signature verification before adding them to the blockstore.
3. Core implementation of signature verification algorithms is in `perf/src/sigverify.rs`.
4. Verification is optimized for batches of transactions using memory recycling and GPU offloading where available.
5. Compute costs for different signature verification operations are defined in the cost model.

## Relevant File Paths

- `perf/src/sigverify.rs` - Core signature verification implementation
- `core/src/sigverify_stage.rs` - Transaction Processing Unit signature verification
- `turbine/src/sigverify_shreds.rs` - Shred signature verification
- `core/src/sigverify.rs` - Core module signature verification utilities
- `cost-model/src/block_cost_limits.rs` - Compute costs for signature operations
- `rpc/src/rpc.rs` - RPC interface with signature verification options
- `core/src/banking_stage/banking_stage.rs` - Processing of verified transactions