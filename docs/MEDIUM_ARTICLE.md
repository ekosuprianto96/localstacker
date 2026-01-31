# Stop Fighting with SSL Certificates in Local Development — Here's a One-Command Solution

## I built a CLI tool that automates HTTPS setup for local development in seconds

![LocalStacker Banner](https://your-image-url.com/banner.png)
*Replace with actual screenshot or banner*

---

If you've ever worked on a modern web application, you know the drill: your production server uses HTTPS, but your local development environment? Plain old HTTP.

This mismatch causes real problems:
- 🔒 OAuth callbacks that only work with HTTPS
- 🍪 Secure cookies that refuse to set
- 🌐 Service workers that won't register
- 🔌 WebSocket connections that behave differently
- 😤 That annoying "Not Secure" warning in your browser

I got tired of manually setting up SSL certificates, configuring Nginx, and remembering all the steps every time I started a new project. So I built **LocalStacker** — a Rust CLI tool that does it all in one command.

---

## 🎯 What is LocalStacker?

LocalStacker is an open-source CLI tool that automates the entire SSL setup process for local development. It handles:

✅ **mkcert installation** — automatically installs if missing  
✅ **SSL certificate generation** — creates locally-trusted certificates  
✅ **Nginx configuration** — generates production-ready configs  
✅ **Service management** — reloads Nginx automatically  
✅ **Domain tracking** — keeps record of all your local domains  

All with a single command.

---

## ⚡ The Magic: One Command Setup

Here's how simple it is:

```bash
sudo localstacker setup --domain myapp.local --port 3000
```

That's it. In about 5 seconds, you'll have:

- A trusted SSL certificate for `myapp.local`
- A production-ready Nginx configuration with:
  - HTTP to HTTPS redirect
  - TLS 1.2/1.3 support
  - Security headers (HSTS, X-Frame-Options, etc.)
  - WebSocket support
  - Proper proxy headers
- The site enabled and Nginx reloaded

Now just add the domain to your `/etc/hosts`:

```bash
echo "127.0.0.1 myapp.local" | sudo tee -a /etc/hosts
```

And visit `https://myapp.local` — no certificate warnings! 🎉

---

## 🛠️ Real-World Use Cases

### React Development

```bash
sudo localstacker setup --domain react.local --port 3000
npm start
# Visit: https://react.local ✅
```

### Django Backend

```bash
sudo localstacker setup --domain api.local --port 8000
python manage.py runserver
# Visit: https://api.local ✅
```

### Microservices Setup

```bash
# Frontend
sudo localstacker setup --domain app.local --port 3000

# Auth Service  
sudo localstacker setup --domain auth.local --port 3001

# API Gateway
sudo localstacker setup --domain api.local --port 8080
```

All services now have proper HTTPS, just like production.

---

## 📊 Managing Your Domains

### List all configured domains

```bash
sudo localstacker list
```

Output:
```
Configured SSL Domains

✓ myapp.local → localhost:3000
✓ api.local → localhost:8080

Showing 2 domain(s). Use --detailed for more info.
```

### Check domain health

```bash
sudo localstacker status
```

Output:
```
Domain Status Report

Domain: myapp.local
  SSL Certificate: ✓ Present
  Nginx Config: ✓ Present
  Site Enabled: ✓ Yes
  Backend Port: 3000 (listening)
  HTTPS Check: ✓ Accessible
```

### Clean up when done

```bash
sudo localstacker remove myapp.local --remove-certs
```

---

## 🏗️ Built with SOLID Principles

As a software engineer, I wanted this tool to be maintainable and extensible. LocalStacker is built in Rust following SOLID principles:

- **Single Responsibility**: Each module handles one concern
- **Open/Closed**: Easy to add new certificate providers or web servers
- **Liskov Substitution**: Trait-based design allows swapping implementations
- **Interface Segregation**: Small, focused interfaces
- **Dependency Inversion**: Depends on abstractions, not concrete implementations

Want to add Let's Encrypt support? Just implement the `CertificateProvider` trait. Apache instead of Nginx? Implement `WebServerConfig`. The architecture makes it straightforward.

---

## 🚀 Getting Started

### Installation

```bash
# Clone the repository
git clone https://github.com/ekosuprianto96/localstacker.git
cd localstacker

# Quick install
chmod +x install.sh
./install.sh
```

Or with Cargo:

```bash
cargo build --release
sudo cp target/release/localstacker /usr/local/bin/
```

### Prerequisites

- Linux (tested on Ubuntu, Debian, Arch)
- Nginx installed
- Root/sudo access

Rust will be automatically installed if missing.

---

## 🔧 Advanced Features

### Custom Nginx Templates

Have specific Nginx requirements? Use your own template:

```bash
sudo localstacker setup \
  --domain custom.local \
  --port 9000 \
  --template ./my-nginx.conf
```

Variables `{{domain}}` and `{{port}}` will be replaced automatically.

### Dry Run Mode

Preview what will happen without making changes:

```bash
sudo localstacker setup \
  --domain test.local \
  --port 8080 \
  --dry-run --verbose
```

### CI/CD Integration

Skip all prompts for automated pipelines:

```bash
sudo localstacker setup \
  --domain ci.local \
  --port 5000 \
  --yes
```

---

## 💡 Why I Built This

I work on multiple projects simultaneously — React frontends, Node.js APIs, Python microservices. Each needs HTTPS for:

- Testing OAuth flows (Google, GitHub, etc.)
- Secure cookie handling
- PWA development with service workers
- WebSocket connections that mirror production

Setting this up manually every time was tedious and error-prone. I'd forget steps, make typos in configs, or spend 30 minutes debugging why certificates weren't trusted.

LocalStacker is the tool I wished existed. Now it does, and I'm sharing it with the community.

---

## 🤝 Open Source

LocalStacker is completely open source under the MIT license. Contributions are welcome!

**GitHub**: [github.com/ekosuprianto96/localstacker](https://github.com/ekosuprianto96/localstacker)

If you find it useful:
- ⭐ Star the repository
- 🐛 Report bugs or request features
- 🔧 Submit pull requests
- 📢 Share with fellow developers

---

## 🎉 Conclusion

Local HTTPS development doesn't have to be painful. With LocalStacker, you can:

1. Setup SSL in one command
2. Get production-like Nginx configs automatically
3. Manage multiple domains easily
4. Focus on building your app, not infrastructure

Give it a try and let me know what you think!

```bash
git clone https://github.com/ekosuprianto96/localstacker.git
cd localstacker && ./install.sh
```

---

**Happy coding with HTTPS!** 🔐🚀

---

*If you enjoyed this article, follow me for more developer tools and tips. Have questions? Drop a comment below!*

---

**Tags**: #Rust #CLI #WebDevelopment #DevTools #HTTPS #Nginx #LocalDevelopment #OpenSource
