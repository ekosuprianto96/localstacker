use crate::core::SystemService;
use crate::error::{Result};
use crate::utils::{execute_command, log_info};

pub struct SystemdService;

impl SystemdService {
    pub fn new() -> Self {
        SystemdService
    }
}

impl SystemService for SystemdService {
    fn exists(&self, service: &str) -> Result<bool> {
        let output = execute_command(
            "systemctl",
            &["list-unit-files", service],
            "Check service exists",
        )?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        Ok(output_str.contains(service))
    }

    fn is_running(&self, service: &str) -> Result<bool> {
        let output = execute_command(
            "systemctl",
            &["is-active", service],
            "Check service status",
        )?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        Ok(output_str.trim() == "active")
    }

    // fn start(&self, service: &str) -> Result<()> {
    //     log_info(&format!("Starting service {}...", service));
    //     execute_command("systemctl", &["start", service], "Start service")?;
    //     Ok(())
    // }

    // fn stop(&self, service: &str) -> Result<()> {
    //     log_info(&format!("Stopping service {}...", service));
    //     execute_command("systemctl", &["stop", service], "Stop service")?;
    //     Ok(())
    // }

    fn restart(&self, service: &str) -> Result<()> {
        log_info(&format!("Restarting service {}...", service));
        execute_command("systemctl", &["restart", service], "Restart service")?;
        Ok(())
    }

    // fn enable(&self, service: &str) -> Result<()> {
    //     log_info(&format!("Enabling service {}...", service));
    //     execute_command("systemctl", &["enable", service], "Enable service")?;
    //     Ok(())
    // }
}