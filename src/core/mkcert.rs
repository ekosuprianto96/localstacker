use std::collections::HashMap;

use crate::core::CertificateProvider;
use crate::error::{Error, Result};
use crate::utils::{
    command_exists, execute_command, execute_command_with_env,
    get_mkcert_caroot, log_info, log_verbose, log_warning,
};

pub struct MkcertProvider {
    caroot: Option<String>,
}

impl MkcertProvider {
    pub fn new() -> Self {
        let caroot = get_mkcert_caroot();
        if let Some(ref path) = caroot {
            log_verbose(&format!("Using CAROOT: {}", path));
        }
        MkcertProvider { caroot }
    }

    /// Get environment variables for mkcert commands
    fn get_env(&self) -> HashMap<&str, &str> {
        let mut env = HashMap::new();
        if let Some(ref caroot) = self.caroot {
            env.insert("CAROOT", caroot.as_str());
        }
        env
    }
}

impl CertificateProvider for MkcertProvider {
    fn is_installed(&self) -> Result<bool> {
        Ok(command_exists("mkcert"))
    }

    fn install(&self) -> Result<()> {
        log_info("Installing mkcert...");

        // Try to detect package manager and install
        if command_exists("apt-get") {
            log_verbose("Using apt-get to install mkcert");
            execute_command("apt-get", &["update"], "Update package list")?;
            execute_command(
                "apt-get",
                &["install", "-y", "mkcert"],
                "Install mkcert",
            )?;
        } else if command_exists("yum") {
            log_verbose("Using yum to install mkcert");
            execute_command("yum", &["install", "-y", "mkcert"], "Install mkcert")?;
        } else if command_exists("brew") {
            log_verbose("Using homebrew to install mkcert");
            execute_command("brew", &["install", "mkcert"], "Install mkcert")?;
        } else {
            return Err(Error::NotFound(
                "No supported package manager found. Please install mkcert manually.".to_string(),
            ));
        }

        Ok(())
    }

    fn install_ca(&self) -> Result<()> {
        log_info("Installing local CA...");
        
        if let Some(ref caroot) = self.caroot {
            log_info(&format!("Using CA from: {}", caroot));
        } else {
            log_warning("CAROOT not detected, using default mkcert location");
        }

        // Use CAROOT environment variable to ensure correct CA is used
        execute_command_with_env(
            "mkcert",
            &["-install"],
            self.get_env(),
            "Install local CA",
        )?;
        
        Ok(())
    }

    fn generate_cert(&self, domain: &str) -> Result<()> {
        log_info(&format!("Generating certificate for {}...", domain));
        
        if let Some(ref caroot) = self.caroot {
            log_verbose(&format!("Certificate will be signed by CA in: {}", caroot));
        }

        // Use CAROOT environment variable to ensure certificate is signed by correct CA
        execute_command_with_env(
            "mkcert",
            &[domain],
            self.get_env(),
            "Generate certificate",
        )?;
        
        Ok(())
    }

    fn get_cert_paths(&self, domain: &str) -> (String, String) {
        (
            format!("{}.pem", domain),
            format!("{}-key.pem", domain),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cert_paths() {
        let provider = MkcertProvider::new();
        let (cert, key) = provider.get_cert_paths("example.com");
        assert_eq!(cert, "example.com.pem");
        assert_eq!(key, "example.com-key.pem");
    }
}