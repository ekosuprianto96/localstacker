use colored::Colorize;
use dialoguer::Confirm;

use crate::config::{ConfigManager, DomainConfig, Paths};
use crate::core::{
    CertificateProvider, FileOperations, MkcertProvider, NginxConfig, SystemService,
    SystemdService, WebServerConfig, FileOps,
};
use crate::error::{Result};
use crate::utils::{
    check_root, get_timestamp, log_info, log_success, log_warning,
    validate_domain, validate_port,
};

pub fn run(
    domain: String,
    port: u16,
    service: Option<String>,
    skip_confirm: bool,
    template: Option<String>,
) -> Result<()> {
    // Check if running as root
    check_root()?;

    // Validate inputs
    validate_domain(&domain)?;
    validate_port(port)?;

    log_info(&format!("Setting up SSL for {} -> localhost:{}", domain, port));

    // Show confirmation unless skipped
    if !skip_confirm {
        let confirm = Confirm::new()
            .with_prompt(format!(
                "This will:\n  \
                • Generate SSL certificate for {}\n  \
                • Create Nginx configuration\n  \
                • Enable the site\n  \
                • Reload Nginx\n\n  \
                Continue?",
                domain
            ))
            .default(true)
            .interact()
            .unwrap_or(false);

        if !confirm {
            log_warning("Setup cancelled by user");
            return Ok(());
        }
    }

    // Initialize services (Dependency Injection following SOLID)
    let cert_provider = MkcertProvider::new();
    let web_server = NginxConfig::new();
    let system_service = SystemdService::new();
    let file_ops = FileOps::new();

    // Step 1: Ensure mkcert is installed
    log_info("Checking mkcert installation...");
    if !cert_provider.is_installed()? {
        log_warning("mkcert not found, attempting to install...");
        cert_provider.install()?;
    }
    log_success("mkcert is installed");

    // Step 2: Install local CA
    cert_provider.install_ca()?;
    log_success("Local CA installed");

    // Step 3: Generate certificate
    cert_provider.generate_cert(&domain)?;
    log_success(&format!("Certificate generated for {}", domain));

    // Step 4: Setup SSL directory and copy certificates
    file_ops.ensure_directory(Paths::ssl_dir())?;

    let (cert_src, key_src) = cert_provider.get_cert_paths(&domain);
    let cert_dest = Paths::ssl_cert(&domain);
    let key_dest = Paths::ssl_key(&domain);

    file_ops.copy_file(&cert_src, &cert_dest)?;
    file_ops.copy_file(&key_src, &key_dest)?;

    // Clean up temporary files
    file_ops.remove_file(&cert_src)?;
    file_ops.remove_file(&key_src)?;

    log_success("SSL certificates installed");

    // Step 5: Generate and write Nginx configuration
    let config = web_server.generate_config(&domain, port, template.as_deref())?;
    web_server.write_config(&domain, &config)?;
    log_success("Nginx configuration created");

    // Step 6: Enable site
    web_server.enable_site(&domain)?;
    log_success("Site enabled");

    // Step 7: Test configuration
    web_server.test_config()?;
    log_success("Nginx configuration test passed");

    // Step 8: Reload Nginx
    web_server.reload()?;
    log_success("Nginx reloaded");

    // Step 9: Optionally check and restart service
    if let Some(service_name) = &service {
        if system_service.exists(service_name)? {
            log_info(&format!("Restarting service {}...", service_name));
            system_service.restart(service_name)?;
            log_success(&format!("Service {} restarted", service_name));
        } else {
            log_warning(&format!("Service {} not found, skipping restart", service_name));
        }
    }

    // Step 10: Save configuration
    let mut config_manager = ConfigManager::new()?;
    let domain_config = DomainConfig {
        domain: domain.clone(),
        port,
        service,
        ssl_cert_path: cert_dest,
        ssl_key_path: key_dest,
        nginx_config_path: Paths::nginx_config(&domain),
        created_at: get_timestamp(),
        enabled: true,
    };

    let was_update = config_manager.upsert_domain(domain_config)?;
    if was_update {
        log_success("Configuration updated");
    } else {
        log_success("Configuration saved");
    }

    // Final success message
    println!();
    println!("{}", "═══════════════════════════════════════════".green());
    println!("{} {}", "✓".green().bold(), "Setup completed successfully!".green().bold());
    println!("{}", "═══════════════════════════════════════════".green());
    println!();
    println!("  {} https://{}", "URL:".bold(), domain);
    println!("  {} localhost:{}", "Backend:".bold(), port);
    println!();
    println!("  {}", "Next steps:".bold());
    println!("    • Make sure your backend is running on port {}", port);
    println!("    • Add {} to your /etc/hosts if needed", domain);
    println!("    • Visit https://{} in your browser", domain);
    println!();

    Ok(())
}