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
