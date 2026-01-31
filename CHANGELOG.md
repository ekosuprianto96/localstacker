# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Let's Encrypt integration
- Apache web server support
- Certificate renewal automation
- Configuration backup/restore
- Shell completion scripts

## [0.1.1] - 2026-01-31

### Fixed
- **SSL Certificate CA Mismatch**: Fixed issue where SSL certificates showed "not secure" in browser when running `sudo localstacker setup`. The root cause was mkcert using root's CA instead of the actual user's CA. Now properly detects `SUDO_USER` and uses the correct CAROOT path.

### Added
- **Auto-update domain configuration**: Running `setup` on an existing domain now automatically updates the configuration instead of throwing an "already exists" error. This makes it easy to regenerate certificates or change port mappings.
- `get_real_user_home()` utility function to detect actual user's home directory when running as sudo
- `get_mkcert_caroot()` utility function to get correct mkcert CA root path
- `execute_command_with_env()` utility function to run commands with custom environment variables
- `upsert_domain()` method in ConfigManager for update-or-insert functionality

### Changed
- MkcertProvider now stores and uses CAROOT environment variable for all mkcert operations
- Setup command shows "Configuration updated" when updating existing domain

## [0.1.0] - 2025-01-31

### Added
- Initial release
- Automatic mkcert installation
- SSL certificate generation
- Nginx configuration automation
- Domain management (setup, list, remove, status)
- Systemd service integration
- Configuration tracking
- Dry-run mode
- Verbose logging
- Custom Nginx templates support
- Beautiful CLI with colored output
- Comprehensive error handling
- SOLID architecture implementation

### Features
- `setup` command - Setup SSL for a domain
- `list` command - List all configured domains
- `remove` command - Remove domain configuration
- `status` command - Check domain health
- `install-mkcert` command - Install mkcert

### Documentation
- README.md - Main documentation
- QUICKSTART.md - Quick start guide
- ARCHITECTURE.md - Architecture documentation
- DEPLOYMENT.md - Deployment guide
- CONTRIBUTING.md - Contribution guidelines
- Examples and templates

[Unreleased]: https://github.com/ekosuprianto96/localstacker/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/ekosuprianto96/localstacker/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/ekosuprianto96/localstacker/releases/tag/v0.1.0