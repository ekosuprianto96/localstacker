use colored::Colorize;

use crate::config::ConfigManager;
use crate::error::Result;

pub fn run(detailed: bool) -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let domains = config_manager.list_domains();

    if domains.is_empty() {
        println!("{}", "No domains configured yet.".yellow());
        println!();
        println!("Use {} to setup a new domain", "nusacloud setup --domain <domain> --port <port>".cyan());
        return Ok(());
    }

    println!();
    println!("{}", "Configured SSL Domains".bold().underline());
    println!();

    let domains_len = domains.len();
    
    for domain_config in domains {
        let status_icon = if domain_config.enabled {
            "✓".green()
        } else {
            "✗".red()
        };

        println!(
            "{} {} {} {}",
            status_icon,
            domain_config.domain.cyan().bold(),
            "→".dimmed(),
            format!("localhost:{}", domain_config.port).yellow()
        );

        if detailed {
            println!("  {} {}", "Created:".dimmed(), domain_config.created_at);
            println!("  {} {}", "SSL Cert:".dimmed(), domain_config.ssl_cert_path);
            println!("  {} {}", "SSL Key:".dimmed(), domain_config.ssl_key_path);
            println!("  {} {}", "Nginx Config:".dimmed(), domain_config.nginx_config_path);
            
            if let Some(ref service) = domain_config.service {
                println!("  {} {}", "Service:".dimmed(), service);
            }
            
            println!();
        }
    }

    if !detailed {
        println!();
        println!("{}", format!("Showing {} domain(s). Use --detailed for more info.", domains_len).dimmed());
    }

    println!();

    Ok(())
}