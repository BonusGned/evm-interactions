use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub name: String,
    pub alias: String,
    pub rpc_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub default_network: Option<String>,
    #[serde(default)]
    pub networks: Vec<Network>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            default_network: Some("eth".to_string()),
            networks: default_networks(),
        }
    }
}

impl AppConfig {
    pub fn load(path: &Path) -> Result<Self, String> {
        let content =
            std::fs::read_to_string(path).map_err(|e| format!("failed to read config: {e}"))?;
        toml::from_str(&content).map_err(|e| format!("failed to parse config: {e}"))
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("failed to create config directory: {e}"))?;
        }
        let content =
            toml::to_string_pretty(self).map_err(|e| format!("failed to serialize config: {e}"))?;
        std::fs::write(path, content).map_err(|e| format!("failed to write config: {e}"))
    }

    pub fn find_network(&self, identifier: &str) -> Option<&Network> {
        let lower = identifier.to_lowercase();
        self.networks
            .iter()
            .find(|n| n.name.to_lowercase() == lower || n.alias.to_lowercase() == lower)
    }

    pub fn remove_network(&mut self, identifier: &str) -> bool {
        let lower = identifier.to_lowercase();
        let before = self.networks.len();
        self.networks
            .retain(|n| n.name.to_lowercase() != lower && n.alias.to_lowercase() != lower);
        self.networks.len() < before
    }

    pub fn add_network(
        &mut self,
        name: String,
        alias: String,
        rpc_url: String,
    ) -> Result<(), String> {
        if self.find_network(&alias).is_some() || self.find_network(&name).is_some() {
            return Err(format!(
                "network with name '{}' or alias '{}' already exists",
                name, alias
            ));
        }
        self.networks.push(Network {
            name,
            alias,
            rpc_url,
        });
        Ok(())
    }
}

pub fn default_networks() -> Vec<Network> {
    vec![
        Network {
            name: "Ethereum".into(),
            alias: "eth".into(),
            rpc_url: "https://ethereum-rpc.publicnode.com".into(),
        },
        Network {
            name: "BNB Smart Chain".into(),
            alias: "bsc".into(),
            rpc_url: "https://bsc-dataseed1.binance.org".into(),
        },
        Network {
            name: "Polygon".into(),
            alias: "pol".into(),
            rpc_url: "https://polygon-bor-rpc.publicnode.com".into(),
        },
        Network {
            name: "Avalanche".into(),
            alias: "avax".into(),
            rpc_url: "https://api.avax.network/ext/bc/C/rpc".into(),
        },
        Network {
            name: "Sonic".into(),
            alias: "sonic".into(),
            rpc_url: "https://rpc.soniclabs.com".into(),
        },
        Network {
            name: "Arbitrum".into(),
            alias: "arb".into(),
            rpc_url: "https://arb1.arbitrum.io/rpc".into(),
        },
    ]
}

pub fn config_path(override_path: Option<&Path>) -> PathBuf {
    if let Some(p) = override_path {
        return p.to_path_buf();
    }
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("evm-interactions")
        .join("config.toml")
}

pub fn load_or_default(path: &Path) -> AppConfig {
    if path.exists() {
        AppConfig::load(path).unwrap_or_else(|e| {
            eprintln!("Warning: {e}, using defaults");
            AppConfig::default()
        })
    } else {
        AppConfig::default()
    }
}

pub fn resolve_networks(
    cfg: &AppConfig,
    aliases: Vec<String>,
    all: bool,
    rpc: Option<String>,
) -> Vec<Network> {
    if all {
        return cfg.networks.clone();
    } else if !aliases.is_empty() {
        return aliases
            .into_iter()
            .map(|identifier| {
                if let Some(rpc_url) = &rpc {
                    Network {
                        name: identifier.clone(),
                        alias: identifier,
                        rpc_url: rpc_url.clone(),
                    }
                } else {
                    cfg.find_network(&identifier).cloned().unwrap_or_else(|| {
                        eprintln!("Network '{}' not found in config", identifier);
                        std::process::exit(1);
                    })
                }
            })
            .collect();
    }

    if let Some(default_alias) = &cfg.default_network {
        if let Some(net) = cfg.find_network(default_alias) {
            return vec![net.clone()];
        }
        eprintln!("Default network '{}' not found in config", default_alias);
        std::process::exit(1);
    }

    cfg.networks.clone()
}

pub fn resolve_network(cfg: &AppConfig, alias: Option<String>, rpc: Option<String>) -> Network {
    if let Some(rpc_url) = rpc {
        let name = alias.clone().unwrap_or_else(|| "Custom".to_string());
        return Network {
            alias: alias.unwrap_or_default(),
            name,
            rpc_url,
        };
    } else if let Some(id) = alias {
        return cfg.find_network(&id).cloned().unwrap_or_else(|| {
            eprintln!("Network '{}' not found in config", id);
            std::process::exit(1);
        });
    } else if let Some(default_alias) = &cfg.default_network {
        if let Some(net) = cfg.find_network(default_alias) {
            return net.clone();
        }
    }

    cfg.networks.first().cloned().unwrap_or_else(|| {
        eprintln!("No networks configured. Run `evm-interactions config init`.");
        std::process::exit(1);
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn make_config() -> AppConfig {
        AppConfig {
            default_network: Some("eth".to_string()),
            networks: vec![
                Network {
                    name: "Ethereum".into(),
                    alias: "eth".into(),
                    rpc_url: "https://eth.rpc".into(),
                },
                Network {
                    name: "BSC".into(),
                    alias: "bsc".into(),
                    rpc_url: "https://bsc.rpc".into(),
                },
            ],
        }
    }

    #[test]
    fn test_default_config_creation() {
        let cfg = AppConfig::default();
        assert_eq!(cfg.default_network.as_deref(), Some("eth"));
        assert_eq!(cfg.networks.len(), 6);

        let names: Vec<&str> = cfg.networks.iter().map(|n| n.name.as_str()).collect();
        assert!(names.contains(&"Ethereum"));
        assert!(names.contains(&"BNB Smart Chain"));
        assert!(names.contains(&"Polygon"));
        assert!(names.contains(&"Avalanche"));
        assert!(names.contains(&"Sonic"));
        assert!(names.contains(&"Arbitrum"));
    }

    #[test]
    fn test_config_load_save() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("config.toml");

        let cfg = AppConfig {
            default_network: Some("test".to_string()),
            networks: vec![Network {
                name: "TestNet".into(),
                alias: "test".into(),
                rpc_url: "https://test.rpc".into(),
            }],
        };

        cfg.save(&path).unwrap();
        let loaded = AppConfig::load(&path).unwrap();

        assert_eq!(loaded.default_network.as_deref(), Some("test"));
        assert_eq!(loaded.networks.len(), 1);
        assert_eq!(loaded.networks[0].name, "TestNet");
        assert_eq!(loaded.networks[0].alias, "test");
        assert_eq!(loaded.networks[0].rpc_url, "https://test.rpc");
    }

    #[test]
    fn test_find_network_by_name() {
        let cfg = make_config();
        let net = cfg.find_network("Ethereum").unwrap();
        assert_eq!(net.alias, "eth");
    }

    #[test]
    fn test_find_network_by_alias() {
        let cfg = make_config();
        let net = cfg.find_network("bsc").unwrap();
        assert_eq!(net.name, "BSC");
    }

    #[test]
    fn test_find_network_case_insensitive() {
        let cfg = make_config();
        assert!(cfg.find_network("ETHEREUM").is_some());
        assert!(cfg.find_network("Bsc").is_some());
        assert!(cfg.find_network("ETH").is_some());
    }

    #[test]
    fn test_find_network_not_found() {
        let cfg = make_config();
        assert!(cfg.find_network("nonexistent").is_none());
    }

    #[test]
    fn test_add_network_success() {
        let mut cfg = make_config();
        let result = cfg.add_network(
            "Polygon".into(),
            "matic".into(),
            "https://polygon.rpc".into(),
        );
        assert!(result.is_ok());
        assert_eq!(cfg.networks.len(), 3);
        assert!(cfg.find_network("matic").is_some());
    }

    #[test]
    fn test_add_network_duplicate_by_alias() {
        let mut cfg = make_config();
        let result = cfg.add_network("Other".into(), "eth".into(), "https://other.rpc".into());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already exists"));
    }

    #[test]
    fn test_add_network_duplicate_by_name() {
        let mut cfg = make_config();
        let result = cfg.add_network("Ethereum".into(), "eth2".into(), "https://other.rpc".into());
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_network_success() {
        let mut cfg = make_config();
        assert!(cfg.remove_network("bsc"));
        assert_eq!(cfg.networks.len(), 1);
        assert!(cfg.find_network("bsc").is_none());
    }

    #[test]
    fn test_remove_network_by_name() {
        let mut cfg = make_config();
        assert!(cfg.remove_network("Ethereum"));
        assert!(cfg.find_network("eth").is_none());
    }

    #[test]
    fn test_remove_network_not_found() {
        let mut cfg = make_config();
        assert!(!cfg.remove_network("nonexistent"));
        assert_eq!(cfg.networks.len(), 2);
    }

    #[test]
    fn test_resolve_networks_all() {
        let cfg = make_config();
        let result = resolve_networks(&cfg, vec![], true, None);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_resolve_networks_by_aliases() {
        let cfg = make_config();
        let result = resolve_networks(&cfg, vec!["bsc".to_string()], false, None);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "BSC");
    }

    #[test]
    fn test_resolve_networks_with_custom_rpc() {
        let cfg = make_config();
        let result = resolve_networks(
            &cfg,
            vec!["custom".to_string()],
            false,
            Some("https://custom.rpc".to_string()),
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].rpc_url, "https://custom.rpc");
        assert_eq!(result[0].name, "custom");
    }

    #[test]
    fn test_resolve_networks_default_fallback() {
        let cfg = make_config();
        let result = resolve_networks(&cfg, vec![], false, None);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].alias, "eth");
    }

    #[test]
    fn test_resolve_networks_no_default_returns_all() {
        let cfg = AppConfig {
            default_network: None,
            networks: make_config().networks,
        };
        let result = resolve_networks(&cfg, vec![], false, None);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_resolve_network_custom_rpc() {
        let cfg = make_config();
        let net = resolve_network(&cfg, None, Some("https://custom.rpc".to_string()));
        assert_eq!(net.name, "Custom");
        assert_eq!(net.rpc_url, "https://custom.rpc");
    }

    #[test]
    fn test_resolve_network_custom_rpc_with_alias() {
        let cfg = make_config();
        let net = resolve_network(
            &cfg,
            Some("mynet".to_string()),
            Some("https://custom.rpc".to_string()),
        );
        assert_eq!(net.name, "mynet");
        assert_eq!(net.alias, "mynet");
        assert_eq!(net.rpc_url, "https://custom.rpc");
    }

    #[test]
    fn test_resolve_network_by_alias() {
        let cfg = make_config();
        let net = resolve_network(&cfg, Some("bsc".to_string()), None);
        assert_eq!(net.name, "BSC");
    }

    #[test]
    fn test_resolve_network_default() {
        let cfg = make_config();
        let net = resolve_network(&cfg, None, None);
        assert_eq!(net.alias, "eth");
    }

    #[test]
    fn test_resolve_network_no_default_fallback_first() {
        let cfg = AppConfig {
            default_network: None,
            networks: make_config().networks,
        };
        let net = resolve_network(&cfg, None, None);
        assert_eq!(net.alias, "eth");
    }

    #[test]
    fn test_config_path_with_override() {
        let path = Path::new("/tmp/custom/config.toml");
        assert_eq!(config_path(Some(path)), path.to_path_buf());
    }

    #[test]
    fn test_config_path_default() {
        let path = config_path(None);
        assert!(path.ends_with("evm-interactions/config.toml"));
    }

    #[test]
    fn test_load_or_default_missing_file() {
        let cfg = load_or_default(Path::new("/tmp/nonexistent/config.toml"));
        assert_eq!(cfg.default_network.as_deref(), Some("eth"));
        assert_eq!(cfg.networks.len(), 6);
    }

    #[test]
    fn test_load_or_default_existing_file() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("config.toml");

        let custom = AppConfig {
            default_network: Some("test".to_string()),
            networks: vec![Network {
                name: "TestNet".into(),
                alias: "test".into(),
                rpc_url: "https://test.rpc".into(),
            }],
        };
        custom.save(&path).unwrap();

        let loaded = load_or_default(&path);
        assert_eq!(loaded.default_network.as_deref(), Some("test"));
        assert_eq!(loaded.networks.len(), 1);
    }

    #[test]
    fn test_load_nonexistent_file_returns_error() {
        let result = AppConfig::load(Path::new("/tmp/nonexistent/config.toml"));
        assert!(result.is_err());
    }

    #[test]
    fn test_save_creates_parent_dirs() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("nested").join("dir").join("config.toml");
        let cfg = AppConfig::default();
        assert!(cfg.save(&path).is_ok());
        assert!(path.exists());
    }
}
