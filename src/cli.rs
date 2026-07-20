use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "rfb-cli", author = "Elsuraj Hussaini", version = "1.0")]
#[command(about = "Rust for Bitcoin 2.0 CLI Tool - Interfaces with Bitcoin Core RPC", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Display general blockchain info (Chain, blocks, headers, difficulty, progress)
    BlockchainInfo,

    /// Display wallet details (Name, balance, unconfirmed balance, tx count)
    WalletInfo,

    /// Print current wallet balance
    Balance,

    /// Generate and print a new receiving address
    NewAddress,

    /// Execute dynamic/arbitrary Bitcoin Core JSON-RPC command
    Rpc {
        /// RPC method name (e.g., getblockcount, getblockhash)
        method: String,
        /// Optional arguments to pass
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        params: Vec<String>,
    },
}