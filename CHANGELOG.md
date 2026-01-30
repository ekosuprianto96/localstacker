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

[Unreleased]: https://github.com/yourusername/localstacker/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/localstacker/releases/tag/v0.1.0