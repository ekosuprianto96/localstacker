# Localsatcker CLI

🚀 **Automated SSL Setup Tool for Nginx with mkcert**

A production-ready Rust CLI tool that automates SSL certificate generation and Nginx configuration for local development using mkcert.

## ✨ Features

- ✅ **Automatic mkcert installation** - Detects and installs mkcert if not present
- 🔐 **SSL certificate generation** - Creates trusted local SSL certificates
- ⚙️ **Nginx auto-configuration** - Generates and deploys production-ready Nginx configs
- 🔄 **Service management** - Optionally manage systemd services
- 📋 **Configuration tracking** - Keeps track of all managed domains
- 🎨 **Beautiful CLI** - Colored output with progress indicators
- 🛡️ **SOLID principles** - Clean, maintainable, and extensible architecture
- 🔍 **Status monitoring** - Check health of all configured domains
- 🗑️ **Easy cleanup** - Remove domains and certificates with one command

## 🏗️ Architecture

Built with **SOLID principles** for maximum maintainability:

- **Single Responsibility**: Each module handles one specific concern
- **Open/Closed**: Easy to extend with new certificate providers or web servers
- **Liskov Substitution**: Trait-based design allows swapping implementations
- **Interface Segregation**: Small, focused interfaces
- **Dependency Inversion**: Depends on abstractions, not concrete implementations

### Project Structure

```
localstacker/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── error.rs             # Custom error types
│   ├── config.rs            # Configuration management
│   ├── utils.rs             # Utility functions
│   ├── core/
│   │   ├── mod.rs           # Core traits
│   │   ├── mkcert.rs        # Certificate provider impl
│   │   ├── nginx.rs         # Web server config impl
│   │   ├── systemd.rs       # System service impl
│   │   └── file_ops.rs      # File operations impl
│   └── commands/
│       ├── mod.rs
│       ├── setup.rs         # Setup command
│       ├── list.rs          # List domains
│       ├── remove.rs        # Remove domain
│       ├── status.rs        # Check status
│       └── install_mkcert.rs # Install mkcert
└── Cargo.toml
```

## 📦 Installation

### Prerequisites

- Nginx installed and running
- Root/sudo access
- Rust 1.70+ (will be auto-installed if missing)

### Option 1: Quick Install (Recommended)

```bash
git clone https://github.com/ekosuprianto96/localstacker.git
cd localstacker

# Run install script (builds and installs automatically)
chmod +x install.sh
./install.sh
```

The script will:
- ✅ Check/install Rust if needed
- ✅ Build optimized release binary
- ✅ Install to `/usr/local/bin/` (if running as root)

### Option 2: Using Makefile

```bash
git clone https://github.com/ekosuprianto96/localstacker.git
cd localstacker

# Build and install (requires sudo)
make install
```

### Option 3: Manual Build

```bash
git clone https://github.com/ekosuprianto96/localstacker.git
cd localstacker

# Build release binary
cargo build --release

# Install system-wide
sudo cp target/release/localstacker /usr/local/bin/

# Or install for current user only
mkdir -p ~/.local/bin
cp target/release/localstacker ~/.local/bin/
# Add ~/.local/bin to PATH in your shell config
```

### Verify Installation

```bash
localstacker --version
localstacker --help
```

## 🚀 Usage

### Setup SSL for a domain

```bash
sudo localstacker setup \
  --domain myapp.local \
  --port 3000
```

With systemd service integration:

```bash
sudo localstacker setup \
  --domain myapp.local \
  --port 3000 \
  --service myapp.service
```

Skip confirmation prompts:

```bash
sudo localstacker setup \
  --domain myapp.local \
  --port 3000 \
  --yes
```

Re-run setup to regenerate certificates or update configuration (auto-updates existing domain):

```bash
sudo localstacker setup \
  --domain myapp.local \
  --port 4000 \
  --yes
```

Use custom Nginx template:

```bash
sudo localstacker setup \
  --domain myapp.local \
  --port 3000 \
  --template ./custom-nginx.conf
```

### List all configured domains

```bash
sudo localstacker list
```

Show detailed information:

```bash
sudo localstacker list --detailed
```

### Check domain status

Check all domains:

```bash
sudo localstacker status
```

Check specific domain:

```bash
sudo localstacker status myapp.local
```

### Remove a domain

```bash
sudo localstacker remove myapp.local
```

Also remove SSL certificates:

```bash
sudo localstacker remove myapp.local --remove-certs
```

Skip confirmation:

```bash
sudo localstacker remove myapp.local --yes
```

### Install mkcert

```bash
sudo localstacker install-mkcert
```

Force reinstall:

```bash
sudo localstacker install-mkcert --force
```

### Global Options

- `--verbose` or `-v`: Enable verbose output
- `--dry-run` or `-n`: Show what would be done without executing

Example:

```bash
sudo localstacker setup --domain test.local --port 8080 --verbose --dry-run
```

## 🔧 What it does

When you run `setup`, the tool:

1. ✅ Checks if mkcert is installed (installs if missing)
2. 🔐 Installs local CA (if not already installed)
3. 📜 Generates SSL certificate for the domain
4. 📁 Copies certificates to `/etc/nginx/ssl/`
5. ⚙️ Generates Nginx configuration with:
   - HTTP to HTTPS redirect
   - SSL/TLS settings
   - Security headers
   - WebSocket support
   - Proxy configuration
6. 🔗 Enables the site (creates symlink)
7. ✔️ Tests Nginx configuration
8. 🔄 Reloads Nginx
9. 🎯 Optionally restarts specified systemd service
10. 💾 Saves configuration for future management

## 📝 Generated Nginx Configuration

The tool generates a production-ready Nginx config with:

- **HTTPS redirect** from HTTP
- **TLS 1.2 and 1.3** support
- **Security headers** (HSTS, X-Frame-Options, etc.)
- **Proxy headers** for backend compatibility
- **WebSocket support**
- **Optimized timeouts and buffering**
- **Access and error logging**

## 🎯 Use Cases

- **Local development** with HTTPS
- **Microservices** development environment
- **Testing** production-like SSL setup
- **DevOps automation** for team environments
- **CI/CD** local testing

## 🔐 Security Notes

- Uses **mkcert** for locally-trusted certificates
- Certificates are **only trusted on the local machine**
- **Not suitable for production** - use Let's Encrypt or similar for production
- Requires **root access** for Nginx configuration

## 🛠️ Extending the Tool

Thanks to SOLID design, you can easily:

### Add a new certificate provider

```rust
// Implement the CertificateProvider trait
pub struct LetsEncryptProvider;

impl CertificateProvider for LetsEncryptProvider {
    // ... implement methods
}
```

### Add a new web server

```rust
// Implement the WebServerConfig trait
pub struct ApacheConfig;

impl WebServerConfig for ApacheConfig {
    // ... implement methods
}
```

### Add a new command

1. Create a new file in `src/commands/`
2. Add command to `main.rs` enum
3. Implement the command logic

## 🐛 Troubleshooting

### Permission denied

Make sure you're running with `sudo`:

```bash
sudo localstacker setup --domain test.local --port 3000
```

### Nginx test failed

Check Nginx configuration manually:

```bash
sudo nginx -t
```

### Port already in use

Make sure another service isn't already using the port:

```bash
sudo ss -tlnp | grep :3000
```

### Certificate not trusted

Reinstall mkcert CA:

```bash
sudo localstacker install-mkcert --force
```

## 📄 License

MIT License - feel free to use in your projects!

## 🤝 Contributing

Contributions are welcome! The codebase follows SOLID principles to make it easy to:

- Add new features
- Fix bugs
- Improve documentation
- Add tests

## 🙏 Credits

Built with:
- [clap](https://github.com/clap-rs/clap) - CLI argument parsing
- [mkcert](https://github.com/FiloSottile/mkcert) - Local certificate generation
- Nginx - Web server / reverse proxy

---

Made with ❤️ for developers who want HTTPS in local development
