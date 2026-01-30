# Architecture Documentation

## Design Principles

This project follows **SOLID principles** to ensure maintainability, testability, and extensibility.

## SOLID Implementation

### 1. Single Responsibility Principle (SRP)

Each module has a single, well-defined responsibility:

- **`core/mkcert.rs`**: Certificate generation using mkcert
- **`core/nginx.rs`**: Nginx configuration management
- **`core/systemd.rs`**: System service management
- **`core/file_ops.rs`**: File system operations
- **`config.rs`**: Configuration persistence
- **`commands/*`**: CLI command implementations

### 2. Open/Closed Principle (OCP)

The system is **open for extension** but **closed for modification**:

```rust
// Easy to add new certificate providers
pub trait CertificateProvider {
    fn generate_cert(&self, domain: &str) -> Result<()>;
    // ...
}

// Without modifying existing code
pub struct LetsEncryptProvider;
impl CertificateProvider for LetsEncryptProvider { /* ... */ }
```

### 3. Liskov Substitution Principle (LSP)

All implementations of traits can be substituted without breaking functionality:

```rust
// Any CertificateProvider works
fn setup_ssl(provider: &dyn CertificateProvider, domain: &str) {
    provider.generate_cert(domain)?;
}

// Works with any implementation
setup_ssl(&MkcertProvider::new(), "test.local");
setup_ssl(&LetsEncryptProvider::new(), "test.local");
```

### 4. Interface Segregation Principle (ISP)

Interfaces are small and focused - clients only depend on methods they use:

```rust
// Separate, focused traits
pub trait CertificateProvider { /* only cert operations */ }
pub trait WebServerConfig { /* only web server operations */ }
pub trait SystemService { /* only service operations */ }
pub trait FileOperations { /* only file operations */ }
```

Instead of one large "Infrastructure" trait.

### 5. Dependency Inversion Principle (DIP)

High-level modules depend on abstractions, not concrete implementations:

```rust
// setup.rs depends on traits, not concrete types
pub fn run(...) -> Result<()> {
    let cert_provider: Box<dyn CertificateProvider> = Box::new(MkcertProvider::new());
    let web_server: Box<dyn WebServerConfig> = Box::new(NginxConfig::new());
    
    // Easy to swap implementations
    cert_provider.generate_cert(&domain)?;
    web_server.generate_config(&domain, port, None)?;
}
```

## Module Structure

```
localstacker/
├── src/
│   ├── main.rs           # Entry point, CLI parsing
│   ├── error.rs          # Error types
│   ├── config.rs         # Configuration management
│   ├── utils.rs          # Shared utilities
│   │
│   ├── core/             # Core abstractions and implementations
│   │   ├── mod.rs        # Trait definitions (interfaces)
│   │   ├── mkcert.rs     # Certificate provider impl
│   │   ├── nginx.rs      # Web server impl
│   │   ├── systemd.rs    # Service manager impl
│   │   └── file_ops.rs   # File operations impl
│   │
│   └── commands/         # CLI commands (use cases)
│       ├── setup.rs      # Setup new domain
│       ├── list.rs       # List configured domains
│       ├── remove.rs     # Remove domain
│       ├── status.rs     # Check domain status
│       └── install_mkcert.rs
```

## Data Flow

```
User Input (CLI)
    ↓
Commands Layer (setup, list, remove, status)
    ↓
Core Services Layer (via traits)
    ↓
Implementation Layer (mkcert, nginx, systemd)
    ↓
System (file system, processes)
```

## Adding New Features

### Add a new certificate provider

1. Implement `CertificateProvider` trait:

```rust
// src/core/letsencrypt.rs
pub struct LetsEncryptProvider;

impl CertificateProvider for LetsEncryptProvider {
    fn is_installed(&self) -> Result<bool> { /* ... */ }
    fn generate_cert(&self, domain: &str) -> Result<()> { /* ... */ }
    // ...
}
```

2. Export in `core/mod.rs`
3. Use in commands via dependency injection

### Add a new web server (Apache, Caddy, etc.)

1. Implement `WebServerConfig` trait:

```rust
// src/core/apache.rs
pub struct ApacheConfig;

impl WebServerConfig for ApacheConfig {
    fn generate_config(&self, domain: &str, port: u16) -> Result<String> { /* ... */ }
    fn reload(&self) -> Result<()> { /* ... */ }
    // ...
}
```

2. Export and use

### Add a new command

1. Create file in `commands/`:

```rust
// src/commands/backup.rs
pub fn run() -> Result<()> {
    // Implementation
}
```

2. Add to `commands/mod.rs`
3. Add CLI argument in `main.rs`

## Error Handling

Custom error types with clear semantics:

```rust
pub enum Error {
    Io(std::io::Error),
    Command(String),
    Validation(String),
    NotFound(String),
    Permission(String),
    Already(String),
    Config(String),
}
```

All functions return `Result<T, Error>` for consistent error propagation.

## Testing Strategy

The SOLID design makes testing easy:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock certificate provider
    struct MockCertProvider;
    impl CertificateProvider for MockCertProvider {
        // Implement with test behavior
    }
    
    #[test]
    fn test_setup_with_mock() {
        let provider = MockCertProvider;
        // Test without touching actual system
    }
}
```

## Configuration Management

Configurations are stored in JSON format at `/etc/nusacloud/domains.json`:

```json
{
  "domains": {
    "myapp.local": {
      "domain": "myapp.local",
      "port": 3000,
      "service": "myapp.service",
      "ssl_cert_path": "/etc/nginx/ssl/myapp.local.pem",
      "ssl_key_path": "/etc/nginx/ssl/myapp.local-key.pem",
      "nginx_config_path": "/etc/nginx/sites-available/myapp.local",
      "created_at": "2025-01-31 10:30:00",
      "enabled": true
    }
  }
}
```

This allows:
- Persistence across runs
- Easy inspection and debugging
- Programmatic access by other tools

## Security Considerations

1. **Root Access**: Required for:
   - Writing to `/etc/nginx/`
   - Reloading Nginx
   - Installing system packages

2. **Certificate Storage**: 
   - Certificates stored in `/etc/nginx/ssl/`
   - Proper file permissions maintained

3. **Validation**:
   - Domain name validation
   - Port number validation
   - Configuration syntax checking

4. **Dry Run Mode**: 
   - Test commands without system changes
   - Useful for CI/CD pipelines

## Future Enhancements

### Planned Features

1. **Multi-certificate providers**:
   - Let's Encrypt integration
   - Custom CA support

2. **Multi-web server support**:
   - Apache
   - Caddy
   - Traefik

3. **Certificate renewal**:
   - Automatic renewal tracking
   - Notification system

4. **Backup/Restore**:
   - Configuration export/import
   - Certificate backup

5. **Templates**:
   - Template repository
   - Custom template validation

### How to Contribute

Thanks to SOLID design:
1. Fork the repository
2. Create a feature branch
3. Implement your feature following existing patterns
4. Add tests
5. Submit pull request

The modular architecture ensures your changes won't break existing functionality.

## Performance Considerations

- **Minimal dependencies**: Fast compilation and small binary size
- **Efficient file operations**: No unnecessary I/O
- **Lazy initialization**: Services created only when needed
- **Configuration caching**: Read once, use multiple times

## Debugging

Enable verbose mode to see detailed operation logs:

```bash
sudo localstacker setup --domain test.local --port 3000 --verbose
```

Use dry-run mode to see what would happen without executing:

```bash
sudo localstacker setup --domain test.local --port 3000 --dry-run
```

## Conclusion

This architecture prioritizes:
- ✅ **Maintainability**: Easy to understand and modify
- ✅ **Extensibility**: Add features without breaking existing code
- ✅ **Testability**: Mock dependencies for unit testing
- ✅ **Reliability**: Clear error handling and validation
- ✅ **Usability**: Intuitive CLI with helpful messages

The SOLID principles ensure the codebase remains clean and professional as it grows.