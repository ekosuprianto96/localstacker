use colored::Colorize;
use std::process::{Command, Output};

use crate::error::{Error, Result};

pub fn is_verbose() -> bool {
    std::env::var("NUSACLOUD_VERBOSE").is_ok()
}

pub fn is_dry_run() -> bool {
    std::env::var("NUSACLOUD_DRY_RUN").is_ok()
}

pub fn log_info(msg: &str) {
    println!("{} {}", "ℹ".blue().bold(), msg);
}

pub fn log_success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg);
}

pub fn log_warning(msg: &str) {
    println!("{} {}", "⚠".yellow().bold(), msg.yellow());
}

pub fn log_verbose(msg: &str) {
    if is_verbose() {
        println!("{} {}", "→".dimmed(), msg.dimmed());
    }
}

pub fn check_root() -> Result<()> {
    if !nix::unistd::Uid::effective().is_root() {
        return Err(Error::Permission(
            "This command requires root privileges. Run with sudo.".to_string(),
        ));
    }
    Ok(())
}

pub fn execute_command(program: &str, args: &[&str], description: &str) -> Result<Output> {
    log_verbose(&format!("Executing: {} {}", program, args.join(" ")));

    if is_dry_run() {
        log_info(&format!("[DRY RUN] Would execute: {} {}", program, args.join(" ")));
        return Ok(Output {
            status: std::process::ExitStatus::default(),
            stdout: vec![],
            stderr: vec![],
        });
    }

    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| Error::Command(format!("Failed to execute {}: {}", program, e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::Command(format!(
            "{} failed: {}",
            description, stderr
        )));
    }

    Ok(output)
}

pub fn validate_domain(domain: &str) -> Result<()> {
    if domain.is_empty() {
        return Err(Error::Validation("Domain cannot be empty".to_string()));
    }

    if !domain.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-') {
        return Err(Error::Validation(
            "Domain contains invalid characters".to_string(),
        ));
    }

    if domain.starts_with('.') || domain.ends_with('.') {
        return Err(Error::Validation(
            "Domain cannot start or end with a dot".to_string(),
        ));
    }

    Ok(())
}

pub fn validate_port(port: u16) -> Result<()> {
    if port == 0 {
        return Err(Error::Validation("Port cannot be 0".to_string()));
    }

    if port < 1024 && !nix::unistd::Uid::effective().is_root() {
        return Err(Error::Validation(
            "Ports below 1024 require root privileges".to_string(),
        ));
    }

    Ok(())
}

pub fn command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn get_timestamp() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub mod chrono {
    use std::time::{SystemTime, UNIX_EPOCH};

    pub struct Local;

    impl Local {
        pub fn now() -> DateTime {
            DateTime {
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            }
        }
    }

    pub struct DateTime {
        timestamp: u64,
    }

    impl DateTime {
        pub fn format(&self, _fmt: &str) -> FormattedDateTime {
            FormattedDateTime {
                timestamp: self.timestamp,
            }
        }
    }

    pub struct FormattedDateTime {
        timestamp: u64,
    }

    impl std::fmt::Display for FormattedDateTime {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let dt = std::time::UNIX_EPOCH + std::time::Duration::from_secs(self.timestamp);
            write!(f, "{:?}", dt)
        }
    }
}