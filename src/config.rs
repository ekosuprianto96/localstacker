use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{PathBuf};

use crate::error::{Error, Result};

const CONFIG_DIR: &str = "/etc/localstacker";
const CONFIG_FILE: &str = "domains.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DomainConfig {
    pub domain: String,
    pub port: u16,
    pub service: Option<String>,
    pub ssl_cert_path: String,
    pub ssl_key_path: String,
    pub nginx_config_path: String,
    pub created_at: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigManager {
    pub domains: HashMap<String, DomainConfig>,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let config_path = Self::config_path();
        
        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            let config: ConfigManager = serde_json::from_str(&contents)
                .map_err(|e| Error::Config(format!("Failed to parse config: {}", e)))?;
            Ok(config)
        } else {
            Ok(ConfigManager::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_dir = PathBuf::from(CONFIG_DIR);
        
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        let config_path = Self::config_path();
        let contents = serde_json::to_string_pretty(self)
            .map_err(|e| Error::Config(format!("Failed to serialize config: {}", e)))?;
        
        fs::write(config_path, contents)?;
        Ok(())
    }

    pub fn add_domain(&mut self, config: DomainConfig) -> Result<()> {
        if self.domains.contains_key(&config.domain) {
            return Err(Error::Already(format!(
                "Domain '{}' already exists in configuration",
                config.domain
            )));
        }
        
        self.domains.insert(config.domain.clone(), config);
        self.save()?;
        Ok(())
    }

    pub fn remove_domain(&mut self, domain: &str) -> Result<DomainConfig> {
        let config = self.domains.remove(domain).ok_or_else(|| {
            Error::NotFound(format!("Domain '{}' not found in configuration", domain))
        })?;
        
        self.save()?;
        Ok(config)
    }

    pub fn get_domain(&self, domain: &str) -> Option<&DomainConfig> {
        self.domains.get(domain)
    }

    pub fn list_domains(&self) -> Vec<&DomainConfig> {
        self.domains.values().collect()
    }

    fn config_path() -> PathBuf {
        PathBuf::from(CONFIG_DIR).join(CONFIG_FILE)
    }
}

pub struct Paths;

impl Paths {
    pub fn ssl_dir() -> &'static str {
        "/etc/nginx/ssl"
    }

    pub fn nginx_sites_available() -> &'static str {
        "/etc/nginx/sites-available"
    }

    pub fn nginx_sites_enabled() -> &'static str {
        "/etc/nginx/sites-enabled"
    }

    pub fn ssl_cert(domain: &str) -> String {
        format!("{}/{}.pem", Self::ssl_dir(), domain)
    }

    pub fn ssl_key(domain: &str) -> String {
        format!("{}/{}-key.pem", Self::ssl_dir(), domain)
    }

    pub fn nginx_config(domain: &str) -> String {
        format!("{}/{}", Self::nginx_sites_available(), domain)
    }

    pub fn nginx_enabled(domain: &str) -> String {
        format!("{}/{}", Self::nginx_sites_enabled(), domain)
    }
}