use crate::cli::ConfigCommands;
use crate::config::{config_path, AppConfig};
use colored::Colorize;
use std::path::Path;

pub fn execute(cmd: ConfigCommands, cfg_path: &Path) {
    match cmd {
        ConfigCommands::Init => init(cfg_path),
        ConfigCommands::List => list(cfg_path),
        ConfigCommands::Add { name, alias, rpc } => add(cfg_path, name, alias, rpc),
        ConfigCommands::Remove { alias } => remove(cfg_path, alias),
        ConfigCommands::Default { alias } => default_network(cfg_path, alias),
        ConfigCommands::Path => println!("{}", config_path(Some(cfg_path)).display()),
    }
}

fn init(cfg_path: &Path) {
    if cfg_path.exists() {
        eprintln!("Config already exists at {}", cfg_path.display());
        eprintln!("Delete it first to reinitialize.");
        std::process::exit(1);
    }

    let cfg = AppConfig::default();
    cfg.save(cfg_path).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });

    println!(
        "{} Config created at {}",
        "✓".bright_green(),
        cfg_path.display()
    );
    println!("  {} networks configured", cfg.networks.len());
}

fn list(cfg_path: &Path) {
    let cfg = load_config(cfg_path);
    let default = cfg.default_network.as_deref().unwrap_or("");

    if cfg.networks.is_empty() {
        println!("No networks configured. Run `evm-interactions config init` to set up defaults.");
        return;
    }

    println!("\n{}", "Configured networks:".bright_white().bold());
    println!("{}", "─".repeat(56).dimmed());

    for net in &cfg.networks {
        let marker = if net.alias == default || net.name == default {
            " (default)".bright_green().to_string()
        } else {
            String::new()
        };
        println!(
            "  {} [{}] {}{}",
            net.name.bright_yellow(),
            net.alias.bright_magenta(),
            net.rpc_url.dimmed(),
            marker
        );
    }
    println!();
}

fn add(cfg_path: &Path, name: String, alias: String, rpc: String) {
    if let Err(e) = reqwest::Url::parse(&rpc) {
        eprintln!("{} Invalid RPC URL '{}': {}", "✗".bright_red(), rpc, e);
        std::process::exit(1);
    }

    let mut cfg = load_config(cfg_path);

    cfg.add_network(name.clone(), alias.clone(), rpc).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });

    cfg.save(cfg_path).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });

    println!("{} Added network '{}' with alias '{}'", "✓".bright_green(), name, alias);
}

fn remove(cfg_path: &Path, identifier: String) {
    let mut cfg = load_config(cfg_path);

    if !cfg.remove_network(&identifier) {
        eprintln!("Network '{}' not found", identifier);
        std::process::exit(1);
    } else if cfg.default_network.as_deref() == Some(&identifier) {
        cfg.default_network = None;
    }

    cfg.save(cfg_path).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });

    println!("{} Removed network '{}'", "✓".bright_green(), identifier);
}

fn default_network(cfg_path: &Path, identifier: Option<String>) {
    let mut cfg = load_config(cfg_path);

    match identifier {
        Some(id) => {
            let found = cfg.find_network(&id).map(|n| (n.alias.clone(), n.name.clone()));
            if let Some((alias, name)) = found {
                cfg.default_network = Some(alias.clone());
                cfg.save(cfg_path).unwrap_or_else(|e| {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                });
                println!("{} Default network set to '{}' ({})", "✓".bright_green(), name, alias);
            } else {
                eprintln!("Network '{}' not found in config", id);
                std::process::exit(1);
            }
        }
        None => match &cfg.default_network {
            Some(alias) => {
                if let Some(net) = cfg.find_network(alias) {
                    println!("Default network: {} [{}]", net.name.bright_yellow(), net.alias.bright_magenta());
                } else {
                    println!("Default network alias '{}' not found", alias);
                }
            }
            None => println!("No default network set"),
        },
    }
}

fn load_config(cfg_path: &Path) -> AppConfig {
    AppConfig::load(cfg_path).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        eprintln!("Run `evm-interactions config init` to create a config file.");
        std::process::exit(1);
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::ConfigCommands;
    use crate::config::AppConfig;
    use tempfile::TempDir;

    fn setup() -> (TempDir, std::path::PathBuf) {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("config.toml");
        (dir, path)
    }

    #[test]
    fn test_cli_config_init() {
        let (_dir, path) = setup();
        execute(ConfigCommands::Init, &path);

        assert!(path.exists());
        let cfg = AppConfig::load(&path).unwrap();
        assert_eq!(cfg.default_network.as_deref(), Some("eth"));
        assert_eq!(cfg.networks.len(), 5);
    }

    #[test]
    fn test_cli_config_add() {
        let (_dir, path) = setup();
        execute(ConfigCommands::Init, &path);

        execute(
            ConfigCommands::Add {
                name: "Arbitrum".into(),
                alias: "arb".into(),
                rpc: "https://arb1.arbitrum.io/rpc".into(),
            },
            &path,
        );

        let cfg = AppConfig::load(&path).unwrap();
        assert_eq!(cfg.networks.len(), 6);
        let arb = cfg.find_network("arb").unwrap();
        assert_eq!(arb.name, "Arbitrum");
        assert_eq!(arb.rpc_url, "https://arb1.arbitrum.io/rpc");
    }

    #[test]
    fn test_cli_config_remove() {
        let (_dir, path) = setup();
        execute(ConfigCommands::Init, &path);

        execute(
            ConfigCommands::Remove {
                alias: "bsc".into(),
            },
            &path,
        );

        let cfg = AppConfig::load(&path).unwrap();
        assert_eq!(cfg.networks.len(), 4);
        assert!(cfg.find_network("bsc").is_none());
    }

    #[test]
    fn test_cli_config_default_set() {
        let (_dir, path) = setup();
        execute(ConfigCommands::Init, &path);

        execute(
            ConfigCommands::Default {
                alias: Some("bsc".into()),
            },
            &path,
        );

        let cfg = AppConfig::load(&path).unwrap();
        assert_eq!(cfg.default_network.as_deref(), Some("bsc"));
    }

    #[test]
    fn test_cli_config_default_show() {
        let (_dir, path) = setup();
        execute(ConfigCommands::Init, &path);
        execute(ConfigCommands::Default { alias: None }, &path);
    }

    #[test]
    fn test_cli_config_list() {
        let (_dir, path) = setup();
        execute(ConfigCommands::Init, &path);
        execute(ConfigCommands::List, &path);
    }

    #[test]
    fn test_cli_config_path() {
        let (_dir, path) = setup();
        execute(ConfigCommands::Path, &path);
    }
}
