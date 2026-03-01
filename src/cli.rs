use crate::export::OutputFormat;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "evm-interactions",
    version,
    about = "CLI tool for interacting with EVM-compatible blockchains",
    long_about = "A command-line utility to fetch blockchain data from EVM networks.\n\
                  Supports multiple networks with configurable RPC endpoints."
)]
pub struct Cli {
    /// Path to config file
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,

    /// Output format: table, json, csv
    #[arg(short, long, global = true, default_value = "table")]
    pub output: OutputFormat,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Fetch block data from EVM networks
    Block {
        /// Network aliases to query (can be specified multiple times)
        #[arg(short, long)]
        alias: Vec<String>,

        /// Query all configured networks
        #[arg(long)]
        all: bool,

        /// One-off RPC URL for a custom network query
        #[arg(long, requires = "alias")]
        rpc: Option<String>,

        /// Block number (decimal or hex with 0x prefix; omit for latest)
        #[arg(short, long)]
        number: Option<String>,

        /// Watch for new blocks in real time
        #[arg(short, long)]
        watch: bool,

        /// Polling interval in seconds for watch mode (default: 12)
        #[arg(short, long, default_value = "12")]
        interval: u64,
    },

    /// Look up a transaction by hash
    Tx {
        /// Transaction hash (0x...)
        hash: String,

        /// Network alias to query
        #[arg(short, long)]
        alias: Option<String>,

        /// One-off RPC URL
        #[arg(long)]
        rpc: Option<String>,
    },

    /// Query account balance
    Balance {
        /// Account address (0x...)
        address: String,

        /// Network aliases to query (can be specified multiple times)
        #[arg(short, long)]
        alias: Vec<String>,

        /// Query all configured networks
        #[arg(long)]
        all: bool,

        /// One-off RPC URL
        #[arg(long, requires = "alias")]
        rpc: Option<String>,
    },

    /// Show current gas prices
    Gas {
        /// Network aliases to query (can be specified multiple times)
        #[arg(short, long)]
        alias: Vec<String>,

        /// Query all configured networks
        #[arg(long)]
        all: bool,

        /// One-off RPC URL
        #[arg(long, requires = "alias")]
        rpc: Option<String>,
    },

    /// Execute a read-only contract call (eth_call)
    Call {
        /// Contract address (0x...)
        address: String,

        /// Calldata (0x-prefixed hex)
        data: String,

        /// Network alias to query
        #[arg(short, long)]
        alias: Option<String>,

        /// One-off RPC URL
        #[arg(long)]
        rpc: Option<String>,

        /// Block tag (latest, earliest, pending, or number)
        #[arg(short, long, default_value = "latest")]
        block: String,
    },

    /// Filter and display event logs
    Logs {
        /// Contract address (0x...)
        address: String,

        /// Event topics to filter (can be specified multiple times)
        #[arg(short, long)]
        topic: Vec<String>,

        /// Start block (number, hex, or tag; default: latest)
        #[arg(long, default_value = "latest")]
        from: String,

        /// End block (number, hex, or tag; default: latest)
        #[arg(long, default_value = "latest")]
        to: String,

        /// Network alias to query
        #[arg(short, long)]
        alias: Option<String>,

        /// One-off RPC URL
        #[arg(long)]
        rpc: Option<String>,
    },

    /// Resolve an ENS name to an address
    Ens {
        /// ENS name (e.g. vitalik.eth)
        name: String,

        /// Network alias (must support ENS, e.g. eth)
        #[arg(short, long)]
        alias: Option<String>,

        /// One-off RPC URL
        #[arg(long)]
        rpc: Option<String>,
    },

    /// Manage network configuration
    #[command(subcommand)]
    Config(ConfigCommands),
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Initialize config with default networks
    Init,

    /// List all configured networks
    List,

    /// Add a new network
    Add {
        /// Network name
        #[arg(short, long)]
        name: String,

        /// Network alias (e.g. eth, bsc)
        #[arg(short, long)]
        alias: String,

        /// RPC endpoint URL
        #[arg(short, long)]
        rpc: String,
    },

    /// Remove a network
    Remove {
        /// Network alias to remove
        #[arg(short, long)]
        alias: String,
    },

    /// Get or set the default network
    Default {
        /// Network alias to set as default (omit to show current)
        #[arg(short, long)]
        alias: Option<String>,
    },

    /// Show config file path
    Path,
}
