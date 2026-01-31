use clap::{Parser, Subcommand};
use colored::Colorize;

mod commands;
mod config;
mod core;
mod error;
mod utils;

use commands::{setup, list, remove, status};

#[derive(Parser)]
#[command(
    name = "localstacker",
    author = "NusaCloud Team",
    version = env!("CARGO_PKG_VERSION"),
    about = "Automated SSL setup tool for Nginx with mkcert",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Dry run mode (don't execute, just show what would happen)
    #[arg(short = 'n', long, global = true)]
    dry_run: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Setup SSL for a new domain
    Setup {
        /// Domain name to setup
        #[arg(long)]
        domain: String,

        /// Backend port to proxy to
        #[arg(long)]
        port: u16,

        /// Systemd service name (optional)
        #[arg(long)]
        service: Option<String>,

        /// Skip confirmation prompts
        #[arg(short = 'y', long)]
        yes: bool,

        /// Custom nginx config template path
        #[arg(long)]
        template: Option<String>,
    },

    /// List all managed SSL configurations
    List {
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },

    /// Remove SSL configuration for a domain
    Remove {
        /// Domain name to remove
        domain: String,

        /// Skip confirmation prompts
        #[arg(short = 'y', long)]
        yes: bool,

        /// Also remove certificates
        #[arg(long)]
        remove_certs: bool,
    },

    /// Check status of SSL setup
    Status {
        /// Domain name to check (optional, checks all if not provided)
        domain: Option<String>,
    },

    /// Install mkcert if not present
    InstallMkcert {
        /// Force reinstall
        #[arg(short, long)]
        force: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // Set global verbose flag
    if cli.verbose {
        std::env::set_var("NUSACLOUD_VERBOSE", "1");
    }

    if cli.dry_run {
        std::env::set_var("NUSACLOUD_DRY_RUN", "1");
        println!("{}", "🔍 DRY RUN MODE - No changes will be made".yellow());
        println!();
    }

    let result = match cli.command {
        Commands::Setup {
            domain,
            port,
            service,
            yes,
            template,
        } => setup::run(domain, port, service, yes, template),

        Commands::List { detailed } => list::run(detailed),

        Commands::Remove {
            domain,
            yes,
            remove_certs,
        } => remove::run(domain, yes, remove_certs),

        Commands::Status { domain } => status::run(domain),

        Commands::InstallMkcert { force } => commands::install_mkcert::run(if force { Some(String::new()) } else { None }),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "✗".red().bold(), e.to_string().red());
        std::process::exit(1);
    }
}