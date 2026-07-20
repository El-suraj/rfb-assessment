# Rust for Bitcoin Program 2.0 - Technical Assessment

A modular, production-ready Rust CLI application designed to interface with a local **Bitcoin Core** node running on **Regtest** (managed via **Polar** or standalone `bitcoind`).

---

## 🏗️ Architecture & Project Structure

The project strictly follows modular engineering practices and strong-typing patterns:

```text
src/
├── main.rs            # Application entry point & command router
├── cli.rs             # Command line argument parser (Clap)
├── config.rs          # Environment & configuration parser
├── error.rs           # Expressive error types (thiserror)
├── rpc.rs             # Reusable Bitcoin Core JSON-RPC Client abstraction
└── commands/          # Strongly-typed command implementations
    ├── mod.rs
    ├── blockchain.rs   # getblockchaininfo typed handler
    ├── wallet.rs       # getwalletinfo & balance typed handlers
    └── address.rs      # getnewaddress typed handler


 🚀 Setting Up Bitcoin Core with Polar
Install Polar: Download and install Polar (cross-platform desktop app for Bitcoin/Lightning local networks).

Create a Network:

Open Polar and click Create a Network.

Add 1 Bitcoin Core Node (Regtest).

Click Create Network.

Start Network: Click Start to spin up the containerized Bitcoin Core daemon.

Obtain Credentials:

Click on the Bitcoin node in Polar.

Go to the Connect tab.

Note the RPC Host, Port, Username, and Password.

⚙️ Configuration
Create a .env file in the project root with your credentials:

Code snippet
BITCOIN_RPC_HOST=127.0.0.1
BITCOIN_RPC_PORT=18443
BITCOIN_RPC_USER=polaruser
BITCOIN_RPC_PASS=polarpass
BITCOIN_RPC_WALLET=miner

 
 💻 Usage & Example Terminal Outputs

1. Blockchain Information
Bash
cargo run -- blockchain-info
Output:

JSON
{
  "chain": "regtest",
  "blocks": 101,
  "headers": 101,
  "difficulty": 4.6565423739069247e-10,
  "verificationprogress": 1.0
}

2. Wallet Information
Bash
cargo run -- wallet-info
Output:

JSON
{
  "walletname": "miner",
  "balance": 50.0,
  "unconfirmed_balance": 0.0,
  "txcount": 101
}

3. Check Wallet Balance
Bash
cargo run -- balance
Output:
Plaintext
Wallet Balance: 50.0 BTC

4. Generate New Address
Bash
cargo run -- new-address
Output
Plaintext
New Address: bcrt1qeygjwh77xyzflumtjsc3xg4hu9vek7r66x75m3

5. Dynamic Generic RPC Calls
Bash
cargo run -- rpc getblockcount
Output:
101
Bash 
cargo run -- rpc getblockhash 100
Output:
"604d68534c5d84331c601f829bb796656affc83cb1ac3cf095f27f5ae7f86116"

 🛡️ Error Handling
The application handles failures gracefully without panics:

Connection Error: Prints clear network/RPC connectivity hints if bitcoind is offline.

Authentication Error: Friendly output for incorrect RPC credentials.

RPC Faults: Captures and displays specific error codes and messages returned by Bitcoin Core.

Author: Surajo Hussaini (El-suraj)

Submited to: TheBuidl Rust for Bitcoin 2.0