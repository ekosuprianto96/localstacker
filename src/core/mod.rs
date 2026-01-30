// Core traits following SOLID principles

use crate::error::Result;

/// Single Responsibility: Each service handles one specific aspect
/// Interface Segregation: Small, focused interfaces

/// Certificate provider interface
pub trait CertificateProvider {
    fn is_installed(&self) -> Result<bool>;
    fn install(&self) -> Result<()>;
    fn install_ca(&self) -> Result<()>;
    fn generate_cert(&self, domain: &str) -> Result<()>;
    fn get_cert_paths(&self, domain: &str) -> (String, String);
}

/// Web server configuration interface
pub trait WebServerConfig {
    fn generate_config(&self, domain: &str, port: u16, template: Option<&str>) -> Result<String>;
    fn write_config(&self, domain: &str, config: &str) -> Result<()>;
    fn enable_site(&self, domain: &str) -> Result<()>;
    fn disable_site(&self, domain: &str) -> Result<()>;
    fn test_config(&self) -> Result<()>;
    fn reload(&self) -> Result<()>;
}

/// System service interface
pub trait SystemService {
    fn exists(&self, service: &str) -> Result<bool>;
    fn is_running(&self, service: &str) -> Result<bool>;
    // fn start(&self, service: &str) -> Result<()>;
    // fn stop(&self, service: &str) -> Result<()>;
    fn restart(&self, service: &str) -> Result<()>;
    // fn enable(&self, service: &str) -> Result<()>;
}

/// File operations interface
pub trait FileOperations {
    fn ensure_directory(&self, path: &str) -> Result<()>;
    fn copy_file(&self, from: &str, to: &str) -> Result<()>;
    fn remove_file(&self, path: &str) -> Result<()>;
    fn file_exists(&self, path: &str) -> bool;
}

// Implementations
mod mkcert;
mod nginx;
mod systemd;
mod file_ops;

pub use mkcert::MkcertProvider;
pub use nginx::NginxConfig;
pub use systemd::SystemdService;
pub use file_ops::FileOps;