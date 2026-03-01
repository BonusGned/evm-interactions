mod cli;
mod commands;
mod config;
mod display;
mod ens;
mod export;
mod model;
mod rpc;

use clap::Parser;
use cli::{Cli, Commands};
use config::{config_path, load_or_default};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let cfg_path = config_path(cli.config.as_deref());
    let output = cli.output;

    match cli.command {
        Commands::Block {
            alias,
            all,
            rpc,
            number,
            watch,
            interval,
        } => {
            let cfg = load_or_default(&cfg_path);
            commands::block::execute(&cfg, alias, all, rpc, number, watch, interval, output).await;
        }
        Commands::Tx { hash, alias, rpc } => {
            let cfg = load_or_default(&cfg_path);
            commands::tx::execute(&cfg, hash, alias, rpc, output).await;
        }
        Commands::Balance {
            address,
            alias,
            all,
            rpc,
        } => {
            let cfg = load_or_default(&cfg_path);
            commands::balance::execute(&cfg, address, alias, all, rpc, output).await;
        }
        Commands::Gas { alias, all, rpc } => {
            let cfg = load_or_default(&cfg_path);
            commands::gas::execute(&cfg, alias, all, rpc, output).await;
        }
        Commands::Call {
            address,
            data,
            alias,
            rpc,
            block,
        } => {
            let cfg = load_or_default(&cfg_path);
            commands::call::execute(&cfg, address, data, alias, rpc, block, output).await;
        }
        Commands::Logs {
            address,
            topic,
            from,
            to,
            alias,
            rpc,
        } => {
            let cfg = load_or_default(&cfg_path);
            commands::logs::execute(&cfg, address, topic, from, to, alias, rpc, output).await;
        }
        Commands::Ens { name, alias, rpc } => {
            let cfg = load_or_default(&cfg_path);
            commands::ens::execute(&cfg, name, alias, rpc, output).await;
        }
        Commands::Config(cmd) => {
            commands::config::execute(cmd, &cfg_path);
        }
    }
}
