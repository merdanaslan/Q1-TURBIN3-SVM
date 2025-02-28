# New Fetch Documentation

This document outlines the relevant code across various modules that participate in the Fetch Stage of the TPU, with a focus on QUIC implementation and transaction scheduling.

## Relevant Files and Code

### agave/quic-client/src/nonblocking/quic_client.rs

- **Role**: Implements a nonblocking QUIC client that connects to UDP ports and provides an interface for sending data with flow control.
- **Key Components**:
  - **Class: `QuicLazyInitializedEndpoint`**
    - **Description**: A lazy-initialized QUIC endpoint for efficient connection management.
    - **Fields**:
      - `endpoint: OnceCell<Arc<Endpoint>>` - The lazily initialized endpoint.
      - `client_certificate: Arc<QuicClientCertificate>` - Certificate for secure connections.
      - `client_endpoint: Option<Endpoint>` - Optional client endpoint.
  - **Error Handling**:
    - `QuicError` enum for handling various QUIC-related errors.
  - **Connection Management**:
    - Handles connection timeouts, handshakes, and flow control.
    - Implements retry logic for connection failures.

### agave/quic-client/src/lib.rs

- **Role**: Provides the main interface for the QUIC client library, including connection pooling and management.
- **Key Components**:
  - **Struct: `QuicPool`**
    - **Description**: Manages a pool of QUIC connections.
    - **Methods**:
      - `add_connection` - Adds a new connection to the pool.
      - `get` - Retrieves a connection from the pool.
      - `create_pool_entry` - Creates a new connection for the pool.
  - **Struct: `QuicConfig`**
    - **Description**: Configuration for QUIC connections.
    - **Methods**:
      - `update_client_certificate` - Updates the client certificate.
      - `set_staked_nodes` - Sets staked nodes information.
  - **Function: `new_quic_connection_cache`**
    - **Description**: Creates a new QUIC connection cache.
    - **Inputs**:
      - `name: &'static str` - Name for the connection cache.
      - `keypair: &Keypair` - Keypair for authentication.
      - `ipaddr: IpAddr` - IP address for the client.
      - `staked_nodes: &Arc<RwLock<StakedNodes>>` - Information about staked nodes.
      - `connection_pool_size: usize` - Size of the connection pool.
    - **Outputs**: `Result<QuicConnectionCache, ClientError>`

### agave/quic-client/src/quic_client.rs

- **Role**: Implements a blocking QUIC client that connects to UDP ports and provides an interface for sending data.
- **Key Components**:
  - **Constant: `MAX_OUTSTANDING_TASK`**
    - **Value**: 2000
    - **Description**: Maximum number of outstanding asynchronous tasks.
  - **Struct: `AsyncTaskSemaphore`**
    - **Description**: Semaphore for limiting the number of asynchronous tasks.
    - **Methods**:
      - `acquire` - Acquires a permit before spawning a task.
      - `release` - Releases a permit after a task completes.
  - **Struct: `QuicClientConnection`**
    - **Description**: Manages a QUIC client connection.
    - **Methods**:
      - `send_data` - Sends data over the QUIC connection.
      - `new_with_client` - Creates a new connection with an existing client.

### agave/core/src/banking_stage/transaction_scheduler/scheduler_controller.rs

- **Role**: Controls the flow of packets and transactions into the scheduler and manages scheduling execution.
- **Key Components**:
  - **Struct: `SchedulerController`**
    - **Description**: Controls packet and transaction flow into the scheduler.
    - **Fields**:
      - `decision_maker: DecisionMaker` - Determines what to do with transactions.
      - `receive_and_buffer: R` - Receives and buffers packets.
      - `bank_forks: Arc<RwLock<BankForks>>` - Access to bank state.
      - `container: R::Container` - Container for transaction state.
      - `scheduler: S` - State for scheduling and communicating with worker threads.
    - **Methods**:
      - `run` - Main loop for processing transactions.
      - `process_transactions` - Processes transactions based on decisions.
      - `receive_and_buffer_packets` - Receives and buffers packets for processing.
      - `forward_packets` - Forwards packets to the leader if needed.

### agave/core/src/banking_stage/transaction_scheduler/receive_and_buffer.rs

- **Role**: Defines traits and implementations for receiving and buffering transactions.
- **Key Components**:
  - **Trait: `ReceiveAndBuffer`**
    - **Description**: Interface for receiving and buffering packets.
    - **Methods**:
      - `receive_and_buffer_packets` - Receives and buffers packets for processing.
  - **Struct: `SanitizedTransactionReceiveAndBuffer`**
    - **Description**: Implementation for sanitized transactions.
    - **Fields**:
      - `packet_receiver: PacketDeserializer` - Receives packets.
      - `bank_forks: Arc<RwLock<BankForks>>` - Access to bank state.
      - `forwarding_enabled: bool` - Whether forwarding is enabled.
  - **Struct: `TransactionViewReceiveAndBuffer`**
    - **Description**: Implementation for transaction views.
    - **Fields**:
      - `receiver: BankingPacketReceiver` - Receives banking packets.
      - `bank_forks: Arc<RwLock<BankForks>>` - Access to bank state.
    - **Methods**:
      - `handle_packet_batch_message` - Processes a batch of packets.
      - `try_handle_packet` - Attempts to handle a single packet.

### agave/streamer/src/nonblocking/quic.rs

- **Role**: Implements a nonblocking QUIC server for receiving transactions.
- **Key Components**:
  - **Constant: `ALPN_TPU_PROTOCOL_ID`**
    - **Value**: `b"solana-tpu"`
    - **Description**: Application-Layer Protocol Negotiation ID for TPU.
  - **Struct: `PacketAccumulator`**
    - **Description**: Accumulates packet chunks.
    - **Fields**:
      - `meta: Meta` - Metadata for the packet.
      - `chunks: SmallVec<[Bytes; 2]>` - Chunks of the packet.
      - `start_time: Instant` - When the accumulation started.
  - **Function: `spawn_server`**
    - **Description**: Spawns a QUIC server.
    - **Inputs**:
      - `name: &'static str` - Name for the server.
      - `sock: UdpSocket` - Socket for the server.
      - `keypair: &Keypair` - Keypair for authentication.
      - `packet_sender: Sender<PacketBatch>` - Channel for sending packets.
      - `exit: Arc<AtomicBool>` - Signal for exiting.
      - `staked_nodes: Arc<RwLock<StakedNodes>>` - Information about staked nodes.
      - `quic_server_params: QuicServerParams` - Parameters for the server.
    - **Outputs**: `Result<SpawnNonBlockingServerResult, QuicServerError>`
  - **Function: `run_server`**
    - **Description**: Main server loop.
    - **Key Operations**:
      - Accepts incoming connections.
      - Validates client certificates.
      - Manages connection tables for staked and unstaked nodes.
      - Processes incoming streams and packets.
      - Implements flow control and congestion management.

## Summary

The fetch stage in the TPU involves receiving transactions via QUIC, buffering them, and scheduling them for processing. The QUIC implementation provides efficient, secure, and flow-controlled data transfer, while the transaction scheduler manages the flow of transactions through the system.

The QUIC client connects to validators to send transactions, while the QUIC server receives transactions from clients and other validators. The transaction scheduler then processes these transactions, making decisions about which ones to process, forward, or drop based on various factors such as stake, load, and bank state.
