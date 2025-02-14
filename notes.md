# Transaction Lifecycle

## High-Level Explanation
1. **Transaction Submission**
   - A user sends a transaction using a wallet
   - Specifies inputs (accounts, instructions, recent blockhash)

2. **Leader Selection & Block Building**
   - The leader receives transactions
   - Organizes them into a block
   - Executes them

3. **Transaction Propagation**
   - The block is propagated to validators using Turbine

4. **Validation & Consensus**
   - Validators execute transactions independently
   - Verify correctness
   - Vote using Tower BFT

5. **Finalization**
   - Once a majority agrees
   - The block is finalized and recorded on-chain

## Low-Level Explanation (With Inputs & Outputs)
### User Interaction:
- Input: Signed transaction (sender, receiver, amount, instructions, recent blockhash)
- Output: Encoded transaction sent to RPC node

### Leader Processing:
- Input: Pool of transactions
- Output: Proposed block containing executed transactions

### Block Propagation:
- Input: Block created by the leader
- Output: Block transmitted via Turbine to validators

### Validation & Voting:
- Input: Proposed block received by validators
- Output: Validators vote using Tower BFT after re-executing transactions

### Finalization:
- Input: Block reaching consensus
- Output: Block permanently recorded on-chain


NOTES
-----

Introduction
-----

Solana is a high-performance blockchain optimized for speed, efficiency, and user experience.
Achieves high throughput with low transaction fees.
Uses integrated architecture for scalability.
Composability ensures all applications can interact seamlessly.

Transaction lifecycle: Users send transactions → Leader processes them → Validators confirm them.
- Users initiate transactions, all of which are sent to the current lead block producer (known as the leader). The leader compiles these transactions into a block, executing them and thereby updating the blockchain's state.
- This block of transactions is then propagated throughout the network for other validators to execute and confirm.

Governance changes follow Solana Improvement Documents (SIMDs).

Scaling and Throughput: Solana processes up to 65,000 transactions per second (TPS), whereas Ethereum (pre-rollups) can only handle 15 TPS. With rollups, Ethereum can scale to 1,000–4,000 TPS, but Solana remains significantly faster.

Scaling Challenges: The blockchain trilemma (decentralization, security, scalability) makes it difficult to achieve all three optimally. Solana prioritizes scalability and low latency but works to maintain decentralization.

Future Scaling Solutions: SVM Rollups and ZK Compression are being explored to improve scalability further.

ZK Compression: Helps scale Solana by reducing the amount of on-chain data storage required. Instead of storing full transaction history or state updates directly on-chain, zero-knowledge proofs (ZKPs) allow for compressed representations of large data sets, making validation faster and reducing storage bloat. This enhances Solana's ability to handle more transactions while maintaining security and decentralization.

Users
-----
Account Basics:
- Public key = account identifier
- Private key = access control
- Transactions are atomic (all or nothing execution) and use Ed25519 for cryptography
- Transactions consist of a header, account list, blockhash, and instructions

Transaction Message Components:
- Header: Specifies the number of required signatures, indicates which accounts need to sign, and contains a reference to the transaction's structure
- Account Addresses: Lists all accounts that will be read from or written to during the transaction. Knowing this in advance enables optimizations that improve efficiency
- Recent Blockhash: Prevents duplicate and stale transactions. A blockhash expires after 151 blocks (~60.4 seconds), ensuring timely transaction execution
- Instructions: Represent specific operations (e.g., transfer, mint, burn, create account, close account). Each instruction includes the program to execute, the required accounts, and necessary data

Transaction Limits:
- Size: Transactions can be up to 1,232 bytes
- Account References: Limited by transaction structure
- Compute Units (CUs): Measure transaction complexity and execution cost

Fee Structure:
- Fees: Base fee + Prioritization fee (burn mechanism applied to fees)
- Priority Fees: Calculated as compute unit price × compute unit limit and paid in addition to the base fee
- Distribution: Only the leader receives a portion of the priority fees from transactions included in their block
- Fee Burning: 50% of base fees are burned to manage SOL supply

Validator Rewards:
Non-leader validators earn rewards through:
- Staking rewards: Earned based on the amount of SOL staked and voting performance
- Voting rewards: Validators vote on finalized blocks, and successful votes accumulate credits, determining their share of staking rewards
- Transaction fees: Validators do not receive priority fees, but they may benefit from base fees depending on future SIMD changes


Gulf Stream
----------
#### Core Functionality
- Gulf Stream is Solana's transaction forwarding protocol that eliminates the mempool by sending transactions directly to the leader before they are executed
- It improves efficiency by allowing leaders to pre-fetch and organize transactions before their slot arrives, ensuring faster execution
- Solana has no mempool; transactions are sent directly to the leader
- RPC nodes (4,000+) facilitate transaction processing

#### Transaction Flow
1. After the leader builds the block, it is propagated to validators using Turbine
2. Validators verify and execute the transactions to ensure correctness
3. Validators vote on the block using Tower BFT; if a majority agrees, the block is confirmed and finalized

#### Stake-Weighted Quality of Service (SWQoS)
- Prioritizes transactions from staked validators
- Capacity allocation:
  - 80% (2,000 connections) reserved for SWQoS
  - 20% (500 connections) allocated for non-staked nodes
- Benefits:
  - Prioritization Mechanism: Ensures transactions from higher-staked validators get priority
  - Network Performance: Reduces congestion and ensures efficient processing of critical transactions
  - Staking Incentive: Improves transaction speed and reliability for staked users
  - Dynamic Allocation: Resource allocation adjustable based on network conditions

#### Technical Components
- QUIC protocol manages transaction message transmission efficiently
- Tower BFT: 
  - Solana's consensus mechanism (adapted from PBFT)
  - Uses Proof of History (PoH) to reduce communication overhead
  - Validators lock votes with exponentially increasing delay
  - Makes fork-switching costly, ensuring stability and finality

#### Fork Management
- Causes: Network latency, leader failures, or conflicting block proposals
- Mitigation: Transactions must declare modifiable accounts beforehand
- Impact: Reduces conflicts and prevents unnecessary forks
- Parallel processing: Can contribute to inconsistencies but managed through account declaration

#### Timing and Scheduling
- Epoch Duration: ~2–3 days (432,000 slots, 400ms per slot)
- Leader Schedule:
  - Deterministic schedule based on stake weight
  - 4 consecutive slots per leader (~1.6s total)
  - Computed at epoch start, fixed until next epoch


Block Building
-----

Continuous block building vs. traditional discrete block building.
400ms slots, 4 slots per leader (1.6s per leader rotation).
Transaction Processing Unit (TPU) handles execution.
- Stages of the TPU:
  Fetch Stage: Collects incoming transactions from the network.
  - Listens for transaction messages sent by clients and other nodes.
  - Ensures a continuous stream of transactions for processing.
  SigVerify Stage: Verifies the signatures of the transactions to ensure authenticity.
  - Checks that each transaction is signed by the appropriate private key.
  - Confirms transaction authorization by the account holder.
  Banking Stage: Executes the transactions, updating the state of the blockchain.
  - Processes each transaction, applying changes to account balances and states.
  - Critical for making actual state changes in the blockchain.
  - Broadcast Stage: Prepares the block for distribution to validators.
  - Packages executed transactions into a block.
  - Sends the block to other validators for validation and consensus.
Transactions are batched into entries (64 per batch).
- These entries are then executed by the Solana Virtual Machine (SVM).
- The SVM processes the transaction instructions within each entry, updating account states in the bank (in-memory).
- After execution, the updated account states are flushed from the bank to the disk to ensure persistence.
Parallel execution: Transactions must declare accounts they modify to avoid conflicts.
Solana Virtual Machine (SVM) executes transactions using a modified rBPF runtime.
- Purpose: The SVM is responsible for executing smart contracts and transactions on the Solana blockchain.
- Architecture: Utilizes a modified version of the Berkeley Packet Filter (rBPF) runtime, optimized for high performance and low latency.
- Parallel Execution: Supports parallel execution of transactions, allowing multiple transactions to be processed simultaneously, provided they do not conflict.
- Input: 
  - Transaction instructions, which include the program to execute, required accounts, and any necessary data.
  - Account states that need to be read or modified during execution.
- Output:
  - Updated account states reflecting the changes made by the transaction.
  - Execution results, which may include logs or return values from smart contracts.
- Security: Ensures that only authorized programs can modify account data, maintaining the integrity of the blockchain.
- Efficiency: Designed to handle thousands of transactions per second, contributing to Solana's high throughput.
- In Solana's architecture, the Solana Virtual Machine (SVM) executes transactions and processes the transaction instructions within each entry. This execution updates the account states in memory. Once the transactions are executed and the account states are updated in the bank (which is an in-memory representation of the blockchain state), these changes are then flushed to disk to ensure persistence and durability.
- Block Acceptance: A block gains acceptance when it receives a supermajority of votes from validators using Tower BFT, ensuring it becomes part of the canonical chain.


Proof of History
-----

- Concept: Proof of History (PoH) is a cryptographic clock that provides a timestamp for each transaction, ensuring a consistent order of events.
- Functionality: PoH is not a consensus mechanism but a way to encode the passage of time and order of transactions.
- Benefits: Reduces the need for extensive communication between nodes, allowing for faster consensus and block production.
- Implementation: Uses SHA-256 hashing to create a verifiable sequence of hashes, acting as a verifiable delay function.
- Role in Solana: Ensures validators adhere to the leader schedule and prevents premature block production.

Accounts Model
-----

- Structure: Solana uses an account-based model where everything is an account, including user accounts, data accounts, and program accounts.
- Fields: Accounts have fields like owner, lamports (SOL balance), data, rent epoch, and executable flag.
- Ownership: Only the program that owns an account can modify its data, enhancing security.
- Rent: Accounts must maintain a minimum balance to remain active, incentivizing users to close unused accounts.
- Program Derived Addresses (PDAs): Special accounts used by programs to store state, ensuring only the owning program can modify them.

Turbine
-----

Purpose: Turbine is Solana's block propagation protocol, inspired by BitTorrent, designed to efficiently distribute blocks across the network.
Mechanism: Breaks blocks into smaller packets called "shreds" for transmission, using erasure coding to handle packet loss.
- Forward Error Correction (FEC) Batches: Used to enhance data reliability by allowing the recovery of lost data packets.
- Transmission Over UDP: Validators send data over the internet using the User Datagram Protocol (UDP), suitable for high-speed, low-latency transmission.
- Data Shred and Recovery Shred Structure:
  - Data Shred: Contains a portion of the block data, including transaction information.
  - Recovery Shred: Contains additional information to help reconstruct lost data shreds.
- Transactions and Entries in Shreds:
  - Shreds typically contain multiple entries, with each entry containing a batch of transactions.
  - Each entry can contain up to 64 transactions, depending on the size and complexity of the transactions.
- Structure of a Turbine Tree:
  - Validators are organized into a tree structure, with the leader at the root.
  - Data is propagated down the tree, with each node responsible for forwarding data to its children.
Efficiency: Reduces the data load on the leader by distributing the transmission responsibility across the network.
Structure: Validators are organized into a tree structure, with higher-stake validators closer to the root, optimizing data flow.

Consensus
-----

- Mechanism: Solana uses Tower BFT, a variant of PBFT, leveraging PoH to reduce communication overhead.
- Voting: Validators vote on blocks they consider valid, with votes incurring a transaction fee.
- Forks: Occur due to network latency or leader failures; validators must choose a fork and stick with it for a lockout period.
- Finalization: A block is finalized once it receives a supermajority of votes, ensuring it becomes part of the canonical chain.

Transaction Validation Unit (TVU):
- Purpose: The TVU is responsible for validating transactions and ensuring they adhere to the network's rules before they are added to the blockchain.
- Functionality: It processes incoming shreds, verifies transactions, and prepares them for consensus.

Stages of the TVU:
- Fetch Stage: Collects incoming shreds from the network.
  - Function: Ensures that all necessary data is available for validation.
- SigVerify Stage: Verifies the signatures of transactions within the shreds.
  - Function: Confirms that transactions are authorized by the account holders.
- Replay Stage: Re-executes transactions to ensure they produce the expected results.
  - Function: Validates the correctness of transactions by comparing the results with expected outcomes.
- Banking Stage: Updates the in-memory state of the blockchain with validated transactions.
  - Function: Applies changes to account balances and states.
- Broadcast Stage: Prepares validated transactions for consensus.
  - Function: Packages transactions into blocks for voting by validators.

Explanation of the TVU:
- The TVU is a critical component of Solana's consensus mechanism, ensuring that only valid transactions are included in the blockchain.
- It operates in parallel with the TPU, focusing on validation and consensus rather than transaction execution.

Economics & Jito
-----

- Inflation: Solana uses inflation to distribute staking rewards, with a decreasing rate over time.
- Staking: SOL holders can delegate tokens to validators to earn rewards, with returns influenced by validator performance and commission rates.
- Jito: A client that introduces blockspace auctions, providing additional incentives for validators through tips.

Gossip & Archive
-----

- Gossip Network: Solana's control plane for disseminating metadata about the network state, using a peer-to-peer protocol.
- Archive: Solana does not require full historical data for current state validation, relying on warehouse nodes for historical data storage.



