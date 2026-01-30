# Project Structure

```
localstacker/
│
├── 📄 Cargo.toml                    # Rust project configuration
├── 📄 Makefile                      # Development workflow automation
├── 🔧 install.sh                    # Quick install script
├── 📄 .gitignore                    # Git ignore patterns
│
├── 📚 Documentation/
│   ├── README.md                    # Main documentation (features, usage)
│   ├── QUICKSTART.md                # 5-minute getting started
│   ├── ARCHITECTURE.md              # SOLID principles & design
│   ├── DEPLOYMENT.md                # Production deployment guide
│   ├── CONTRIBUTING.md              # Contribution guidelines
│   ├── CHANGELOG.md                 # Version history
│   ├── PROJECT_OVERVIEW.md          # This project summary
│   └── LICENSE                      # MIT License
│
├── 📁 src/                          # Source code
│   │
│   ├── main.rs                      # 🚀 CLI entry point & command routing
│   │   - CLI argument parsing (clap)
│   │   - Command dispatching
│   │   - Global options handling
│   │
│   ├── error.rs                     # 🛡️ Error types & handling
│   │   - Custom Error enum
│   │   - Error conversions
│   │   - Result type alias
│   │
│   ├── config.rs                    # ⚙️ Configuration management
│   │   - DomainConfig struct
│   │   - ConfigManager (load/save)
│   │   - Paths helper
│   │
│   ├── utils.rs                     # 🔧 Utility functions
│   │   - Logging functions
│   │   - Validation (domain, port)
│   │   - Command execution
│   │   - Root check
│   │
│   ├── 📁 core/                     # 🏗️ Core abstractions (SOLID)
│   │   │
│   │   ├── mod.rs                   # Trait definitions
│   │   │   - CertificateProvider trait
│   │   │   - WebServerConfig trait
│   │   │   - SystemService trait
│   │   │   - FileOperations trait
│   │   │
│   │   ├── mkcert.rs                # Certificate provider implementation
│   │   │   - MkcertProvider struct
│   │   │   - Install & generate certs
│   │   │
│   │   ├── nginx.rs                 # Web server config implementation
│   │   │   - NginxConfig struct
│   │   │   - Generate configs
│   │   │   - Enable/disable sites
│   │   │
│   │   ├── systemd.rs               # System service implementation
│   │   │   - SystemdService struct
│   │   │   - Start/stop/restart
│   │   │
│   │   └── file_ops.rs              # File operations implementation
│   │       - FileOps struct
│   │       - Copy/remove files
│   │
│   └── 📁 commands/                 # 🎯 CLI commands
│       │
│       ├── mod.rs                   # Command module exports
│       │
│       ├── setup.rs                 # Setup command (main workflow)
│       │   - Install mkcert
│       │   - Generate certificates
│       │   - Configure Nginx
│       │   - Enable site
│       │
│       ├── list.rs                  # List domains command
│       │   - Show all configured domains
│       │   - Detailed view option
│       │
│       ├── remove.rs                # Remove domain command
│       │   - Clean up configuration
│       │   - Remove certificates (optional)
│       │
│       ├── status.rs                # Status check command
│       │   - Check SSL certificates
│       │   - Check Nginx config
│       │   - Check backend port
│       │   - Check service status
│       │
│       └── install_mkcert.rs        # Install mkcert command
│           - Install mkcert manually
│           - Force reinstall option
│
└── 📁 examples/                     # 📖 Examples & templates
    │
    ├── custom-template.conf         # Custom Nginx template example
    ├── myapp.service                # Systemd service example
    └── tests_example.rs             # Testing patterns & examples

```

## 📊 Statistics

- **Total Files**: 30+
- **Rust Source Files**: 14
- **Documentation Files**: 8
- **Example Files**: 3
- **Configuration Files**: 4

## 🎨 Color Legend

- 📄 Configuration/Build files
- 📚 Documentation
- 📁 Directories
- 🚀 Main entry points
- 🛡️ Error handling
- ⚙️ Configuration
- 🔧 Utilities
- 🏗️ Core architecture
- 🎯 Commands
- 📖 Examples

## 🔑 Key Files Explained

### Entry Point
- `main.rs` - Parse CLI args, dispatch to commands

### Core Architecture (SOLID)
- `core/mod.rs` - Trait definitions (interfaces)
- `core/*.rs` - Trait implementations

### Commands (Business Logic)
- `commands/setup.rs` - Main SSL setup workflow
- `commands/list.rs` - List configured domains
- `commands/remove.rs` - Remove domains
- `commands/status.rs` - Health checks

### Infrastructure
- `error.rs` - Error handling
- `config.rs` - Persistence
- `utils.rs` - Shared utilities

## 🔄 Data Flow

```
User Input (CLI)
      ↓
main.rs (parse args)
      ↓
commands/* (business logic)
      ↓
core/* (via traits)
      ↓
System (nginx, mkcert, systemd)
```

## 📦 Dependencies

From `Cargo.toml`:
- `clap` - CLI argument parsing
- `anyhow` - Error handling
- `serde` - Serialization
- `serde_json` - JSON config
- `colored` - Colored output
- `dialoguer` - Interactive prompts
- `nix` - Unix system calls

## 🎯 Entry Points

**For Users:**
```bash
localstacker setup --domain myapp.local --port 3000
localstacker list
localstacker status
localstacker remove myapp.local
```

**For Developers:**
```rust
// Start here:
src/main.rs           // Understand CLI structure

// Then explore:
src/commands/setup.rs // Main workflow
src/core/mod.rs       // Trait definitions
src/core/*.rs         // Implementations

// Extend here:
src/core/             // Add new providers
src/commands/         // Add new commands
```

## 📚 Documentation Map

**Quick Start:**
1. README.md → Features overview
2. QUICKSTART.md → Get running in 5 mins

**Deep Dive:**
3. ARCHITECTURE.md → Understand design
4. DEPLOYMENT.md → Production setup

**Contributing:**
5. CONTRIBUTING.md → How to contribute
6. examples/ → Code examples

**Reference:**
7. CHANGELOG.md → Version history
8. PROJECT_OVERVIEW.md → This file

## 🚀 Getting Started Path

1. **Read** `README.md` (5 mins)
2. **Try** `QUICKSTART.md` (10 mins)
3. **Install** using `install.sh`
4. **Test** with dry-run mode
5. **Setup** your first domain
6. **Explore** the code

## 🎓 Learning Path

**Beginner:**
- Use the tool
- Read user documentation
- Try examples

**Intermediate:**
- Read ARCHITECTURE.md
- Understand SOLID principles
- Explore source code

**Advanced:**
- Add new features
- Contribute improvements
- Extend architecture

---

**This structure enables:**
✅ Easy navigation
✅ Clear separation of concerns
✅ Simple extension points
✅ Comprehensive documentation
✅ Professional development workflow