use colored::Colorize;
use std::process::Command;

use crate::config::ConfigManager;
use crate::core::{FileOps, FileOperations, SystemdService, SystemService};
use crate::error::Result;

pub fn run(domain: Option<String>) -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let file_ops = FileOps::new();
    let system_service = SystemdService::new();

    let domains_to_check: Vec<_> = if let Some(ref d) = domain {
        if let Some(config) = config_manager.get_domain(d) {
            vec![config.clone()]
        } else {
            return Err(crate::error::Error::NotFound(format!(
                "Domain '{}' not found",
                d
            )));
        }
    } else {
        config_manager.list_domains().iter().map(|&c| c.clone()).collect()
    };

    if domains_to_check.is_empty() {
        println!("{}", "No domains configured.".yellow());
        return Ok(());
    }

    println!();
    println!("{}", "Domain Status Report".bold().underline());
    println!();

    for config in domains_to_check {
        println!("{} {}", "Domain:".bold(), config.domain.cyan());

        // Check SSL certificates
        let cert_exists = file_ops.file_exists(&config.ssl_cert_path);
        let key_exists = file_ops.file_exists(&config.ssl_key_path);

        print!("  SSL Certificate: ");
        if cert_exists && key_exists {
            println!("{}", "✓ Present".green());
        } else {
            println!("{}", "✗ Missing".red());
        }

        // Check Nginx config
        let nginx_exists = file_ops.file_exists(&config.nginx_config_path);
        print!("  Nginx Config: ");
        if nginx_exists {
            println!("{}", "✓ Present".green());
        } else {
            println!("{}", "✗ Missing".red());
        }

        // Check if site is enabled
        let enabled_path = format!("/etc/nginx/sites-enabled/{}", config.domain);
        let is_enabled = file_ops.file_exists(&enabled_path);
        print!("  Site Enabled: ");
        if is_enabled {
            println!("{}", "✓ Yes".green());
        } else {
            println!("{}", "✗ No".red());
        }

        // Check backend port
        print!("  Backend Port: ");
        if check_port_listening(config.port) {
            println!("{} {}", config.port.to_string().green(), "(listening)".green());
        } else {
            println!("{} {}", config.port.to_string().yellow(), "(not listening)".yellow());
        }

        // Check service if specified
        if let Some(ref service_name) = config.service {
            print!("  Service: ");
            match system_service.is_running(service_name) {
                Ok(true) => println!("{} {}", service_name.green(), "(running)".green()),
                Ok(false) => println!("{} {}", service_name.yellow(), "(stopped)".yellow()),
                Err(_) => println!("{} {}", service_name.red(), "(not found)".red()),
            }
        }

        // Check HTTPS connectivity
        print!("  HTTPS Check: ");
        if check_https_connectivity(&config.domain) {
            println!("{}", "✓ Accessible".green());
        } else {
            println!("{}", "✗ Not accessible".yellow());
        }

        println!();
    }

    Ok(())
}

fn check_port_listening(port: u16) -> bool {
    Command::new("ss")
        .args(&["-ln", &format!("sport = :{}", port)])
        .output()
        .map(|output| {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.contains(&format!(":{}", port))
        })
        .unwrap_or(false)
}

fn check_https_connectivity(domain: &str) -> bool {
    Command::new("curl")
        .args(&[
            "-k", // ignore cert validation
            "-s",
            "-o",
            "/dev/null",
            "-w",
            "%{http_code}",
            &format!("https://{}", domain),
            "--max-time",
            "5",
        ])
        .output()
        .map(|output| {
            let code = String::from_utf8_lossy(&output.stdout);
            code.starts_with('2') || code.starts_with('3')
        })
        .unwrap_or(false)
}