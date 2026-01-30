# Deployment Guide

Guide untuk deployment dan distribusi NusaCloud CLI.

## Building untuk Production

### 1. Build Release Binary

```bash
cargo build --release
```

Binary akan tersedia di: `target/release/localstacker`

### 2. Strip Binary (Optional, untuk ukuran lebih kecil)

```bash
strip target/release/localstacker
```

### 3. Check Binary Size

```bash
ls -lh target/release/localstacker
```

## Cross-Compilation

### Build untuk Linux (dari macOS/Windows)

```bash
# Install target
rustup target add x86_64-unknown-linux-gnu

# Build
cargo build --release --target x86_64-unknown-linux-gnu
```

### Build Static Binary (untuk portabilitas maksimal)

```bash
# Install musl target
rustup target add x86_64-unknown-linux-musl

# Build
cargo build --release --target x86_64-unknown-linux-musl
```

## Package Distribution

### DEB Package (Debian/Ubuntu)

Install cargo-deb:

```bash
cargo install cargo-deb
```

Create package:

```bash
cargo deb
```

Install:

```bash
sudo dpkg -i target/debian/localstacker_*.deb
```

### RPM Package (RHEL/CentOS/Fedora)

Install cargo-rpm:

```bash
cargo install cargo-rpm
```

Build package:

```bash
cargo rpm build
```

### AUR Package (Arch Linux)

Create PKGBUILD file in separate repository.

### Homebrew (macOS)

Create formula file for homebrew-tap repository.

## Docker Container

### Dockerfile

```dockerfile
FROM rust:1.75-slim as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    nginx \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/localstacker /usr/local/bin/

ENTRYPOINT ["localstacker"]
CMD ["--help"]
```

### Build Docker Image

```bash
docker build -t localstacker:latest .
```

### Run in Docker

```bash
docker run -v /etc/nginx:/etc/nginx \
           -v /etc/nusacloud:/etc/nusacloud \
           localstacker:latest setup --domain test.local --port 3000
```

## CI/CD

### GitHub Actions

```yaml
name: Build and Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Build
        run: cargo build --release
        
      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: target/release/localstacker
```

### GitLab CI

```yaml
build:
  image: rust:latest
  script:
    - cargo build --release
  artifacts:
    paths:
      - target/release/localstacker
```

## Installation Methods

### Method 1: Direct Binary Install

```bash
# Download binary
curl -L -o localstacker https://github.com/user/localstacker/releases/download/v0.1.0/localstacker

# Make executable
chmod +x localstacker

# Move to system path
sudo mv localstacker /usr/local/bin/
```

### Method 2: Install Script

```bash
curl -sSL https://raw.githubusercontent.com/user/localstacker/main/install.sh | bash
```

### Method 3: Package Manager

```bash
# Debian/Ubuntu
sudo apt install localstacker

# RHEL/CentOS
sudo yum install localstacker

# Arch Linux
yay -S localstacker

# macOS
brew install localstacker
```

## Configuration Management

### System-wide Configuration

Lokasi: `/etc/nusacloud/`

```bash
sudo mkdir -p /etc/nusacloud
sudo chown root:root /etc/nusacloud
sudo chmod 755 /etc/nusacloud
```

### User Configuration (Future)

Lokasi: `~/.config/nusacloud/`

## Updating

### Manual Update

```bash
# Download new version
curl -L -o localstacker-new https://github.com/user/localstacker/releases/download/v0.2.0/localstacker

# Replace old version
sudo mv localstacker-new /usr/local/bin/localstacker
sudo chmod +x /usr/local/bin/localstacker
```

### Using Package Manager

```bash
sudo apt update && sudo apt upgrade localstacker
```

## Backup Before Update

```bash
# Backup configuration
sudo cp -r /etc/nusacloud /etc/nusacloud.backup

# Backup nginx configs
sudo tar czf nginx-configs-backup.tar.gz \
  /etc/nginx/sites-available/* \
  /etc/nginx/sites-enabled/* \
  /etc/nginx/ssl/*
```

## Monitoring

### Logging

Application logs ke stdout/stderr. Untuk persistent logging:

```bash
# Run with logging
localstacker setup --domain test.local --port 3000 2>&1 | tee -a /var/log/nusacloud.log
```

### Health Checks

```bash
# Check if binary is working
localstacker --version

# List all domains
localstacker list

# Check status
localstacker status
```

## Security Considerations

### File Permissions

```bash
# Binary
sudo chown root:root /usr/local/bin/localstacker
sudo chmod 755 /usr/local/bin/localstacker

# Configuration
sudo chown root:root /etc/nusacloud
sudo chmod 755 /etc/nusacloud
sudo chmod 644 /etc/nusacloud/domains.json

# SSL Directory
sudo chown root:root /etc/nginx/ssl
sudo chmod 755 /etc/nginx/ssl
sudo chmod 644 /etc/nginx/ssl/*.pem
```

### SELinux (RHEL/CentOS)

```bash
# Set proper context
sudo semanage fcontext -a -t bin_t /usr/local/bin/localstacker
sudo restorecon -v /usr/local/bin/localstacker
```

### AppArmor (Ubuntu)

Create profile if needed:

```bash
sudo aa-genprof localstacker
```

## Troubleshooting Deployment

### Issue: Binary not found after install

**Check PATH:**

```bash
echo $PATH
which localstacker
```

**Solution:** Add to PATH in `~/.bashrc` or `~/.zshrc`

```bash
export PATH="/usr/local/bin:$PATH"
```

### Issue: Permission denied

**Solution:** Ensure proper ownership and executable bit

```bash
sudo chown root:root /usr/local/bin/localstacker
sudo chmod +x /usr/local/bin/localstacker
```

### Issue: Library not found (dynamic linking)

**Solution:** Use static build or install dependencies

```bash
# Static build
cargo build --release --target x86_64-unknown-linux-musl

# Or install dependencies
sudo apt install libc6
```

## Performance Optimization

### Build Optimization

Already configured in `Cargo.toml`:

```toml
[profile.release]
strip = true        # Remove debug symbols
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization
```

### Runtime Optimization

- Binary size: ~5-8 MB (after strip)
- Memory usage: <10 MB
- Startup time: <100ms

## Multi-server Deployment

### Ansible Playbook

```yaml
---
- name: Deploy NusaCloud CLI
  hosts: webservers
  become: yes
  tasks:
    - name: Download binary
      get_url:
        url: https://github.com/user/localstacker/releases/download/v0.1.0/localstacker
        dest: /usr/local/bin/localstacker
        mode: '0755'
        
    - name: Create config directory
      file:
        path: /etc/nusacloud
        state: directory
        mode: '0755'
```

### Terraform (Infrastructure as Code)

```hcl
resource "null_resource" "install_nusacloud" {
  provisioner "remote-exec" {
    inline = [
      "curl -L -o /usr/local/bin/localstacker https://github.com/user/localstacker/releases/download/v0.1.0/localstacker",
      "chmod +x /usr/local/bin/localstacker"
    ]
  }
}
```

## Uninstallation

### Remove Binary

```bash
sudo rm /usr/local/bin/localstacker
```

### Remove Configuration

```bash
sudo rm -rf /etc/nusacloud
```

### Remove All Managed Domains

```bash
# List and remove each domain
localstacker list
localstacker remove domain1.local --remove-certs
localstacker remove domain2.local --remove-certs
```

### Clean Nginx Configs

```bash
# If you want to remove all nusacloud-managed configs
sudo rm /etc/nginx/sites-available/[managed-domains]
sudo rm /etc/nginx/sites-enabled/[managed-domains]
sudo systemctl reload nginx
```

---

## Support

For deployment issues:
- Check GitHub Issues
- Read the logs with `--verbose`
- Test with `--dry-run`
- Contact maintainers

Happy deploying! 🚀