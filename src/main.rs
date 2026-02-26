mod cli;
mod commands;
mod config;
mod display;
mod model;
mod rpc;

use clap::Parser;
use cli::{Cli, Commands};
use config::{config_path, load_or_default};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let cfg_path = config_path(cli.config.as_deref());

    match cli.command {
        Commands::Block {
            alias,
            all,
            rpc,
            number,
        } => {
            let cfg = load_or_default(&cfg_path);
            commands::block::execute(&cfg, alias, all, rpc, number).await;
        }
        Commands::Tx { hash, alias, rpc } => {
            let cfg = load_or_default(&cfg_path);
            commands::tx::execute(&cfg, hash, alias, rpc).await;
        }
        Commands::Balance {
            address,
            alias,
            all,
            rpc,
        } => {
            let cfg = load_or_default(&cfg_path);
            commands::balance::execute(&cfg, address, alias, all, rpc).await;
        }
        Commands::Gas { alias, all, rpc } => {
            let cfg = load_or_default(&cfg_path);
            commands::gas::execute(&cfg, alias, all, rpc).await;
        }
        Commands::Config(cmd) => {
            commands::config::execute(cmd, &cfg_path);
        }
    }
}
