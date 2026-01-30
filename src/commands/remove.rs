use colored::Colorize;
use dialoguer::Confirm;

use crate::config::ConfigManager;
use crate::core::{FileOps, FileOperations, NginxConfig, WebServerConfig};
use crate::error::Result;
use crate::utils::{check_root, log_info, log_success, log_warning};

pub fn run(domain: String, skip_confirm: bool, remove_certs: bool) -> Result<()> {
    check_root()?;

    let mut config_manager = ConfigManager::new()?;

    // Check if domain exists
    let domain_config = config_manager
        .get_domain(&domain)
        .ok_or_else(|| crate::error::Error::NotFound(format!("Domain '{}' not found", domain)))?
        .clone();

    log_info(&format!("Found configuration for {}", domain));

    // Confirm removal unless skipped
    if !skip_confirm {
        let mut prompt = format!("Remove SSL configuration for {}?", domain);
        if remove_certs {
            prompt.push_str("\n  This will also remove SSL certificates.");
        }

        let confirm = Confirm::new()
            .with_prompt(prompt)
            .default(false)
            .interact()
            .unwrap_or(false);

        if !confirm {
            log_warning("Removal cancelled by user");
            return Ok(());
        }
    }

    let web_server = NginxConfig::new();
    let file_ops = FileOps::new();

    // Disable site
    web_server.disable_site(&domain)?;
    log_success("Site disabled");

    // Remove nginx config
    if file_ops.file_exists(&domain_config.nginx_config_path) {
        file_ops.remove_file(&domain_config.nginx_config_path)?;
        log_success("Nginx configuration removed");
    }

    // Remove certificates if requested
    if remove_certs {
        if file_ops.file_exists(&domain_config.ssl_cert_path) {
            file_ops.remove_file(&domain_config.ssl_cert_path)?;
        }
        if file_ops.file_exists(&domain_config.ssl_key_path) {
            file_ops.remove_file(&domain_config.ssl_key_path)?;
        }
        log_success("SSL certificates removed");
    } else {
        log_info("SSL certificates kept (use --remove-certs to delete them)");
    }

    // Test and reload nginx
    web_server.test_config()?;
    web_server.reload()?;
    log_success("Nginx reloaded");

    // Remove from config
    config_manager.remove_domain(&domain)?;
    log_success("Configuration removed");

    println!();
    println!("{} {}", "✓".green().bold(), format!("Successfully removed {}", domain).green());
    println!();

    Ok(())
}