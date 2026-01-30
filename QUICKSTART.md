# Quick Start Guide

Get up and running with NusaCloud CLI in 5 minutes!

## Installation

### Option 1: Quick Install (Recommended)

```bash
# Clone the repository
git clone <repository-url>
cd localstacker

# Run the install script
chmod +x install.sh
./install.sh
```

### Option 2: Manual Install

```bash
# Build
cargo build --release

# Install (requires sudo)
sudo cp target/release/localstacker /usr/local/bin/
```

### Option 3: Using Make

```bash
make install
```

## First Steps

### 1. Setup Your First Domain

Let's setup SSL for a local app running on port 3000:

```bash
sudo localstacker setup \
  --domain myapp.local \
  --port 3000
```

**What this does:**
- ✅ Installs mkcert (if needed)
- ✅ Generates SSL certificate
- ✅ Creates Nginx config
- ✅ Enables the site
- ✅ Reloads Nginx

### 2. Add Domain to /etc/hosts

```bash
sudo sh -c 'echo "127.0.0.1 myapp.local" >> /etc/hosts'
```

### 3. Start Your Application

```bash
# Example: Node.js app
node server.js
# Or any app listening on port 3000
```

### 4. Visit Your Site

Open browser: `https://myapp.local`

**No certificate warnings!** 🎉

## Common Scenarios

### React App (Port 3000)

```bash
sudo localstacker setup --domain react.local --port 3000
echo "127.0.0.1 react.local" | sudo tee -a /etc/hosts
npm start
# Visit: https://react.local
```

### Django App (Port 8000)

```bash
sudo localstacker setup --domain django.local --port 8000
echo "127.0.0.1 django.local" | sudo tee -a /etc/hosts
python manage.py runserver
# Visit: https://django.local
```

### Flask App (Port 5000)

```bash
sudo localstacker setup --domain flask.local --port 5000
echo "127.0.0.1 flask.local" | sudo tee -a /etc/hosts
flask run
# Visit: https://flask.local
```

### Rust Actix (Port 8080)

```bash
sudo localstacker setup --domain actix.local --port 8080 --service myapp.service
echo "127.0.0.1 actix.local" | sudo tee -a /etc/hosts
cargo run
# Visit: https://actix.local
```

### Multiple Domains (Microservices)

```bash
# Auth service
sudo localstacker setup --domain auth.local --port 3001

# API service
sudo localstacker setup --domain api.local --port 3002

# Frontend
sudo localstacker setup --domain app.local --port 3000

# Add all to /etc/hosts
cat << EOF | sudo tee -a /etc/hosts
127.0.0.1 auth.local
127.0.0.1 api.local
127.0.0.1 app.local
EOF
```

## Managing Domains

### List All Domains

```bash
sudo localstacker list
```

Output:
```
✓ myapp.local → localhost:3000
✓ api.local → localhost:8080
✓ admin.local → localhost:4000

Showing 3 domain(s). Use --detailed for more info.
```

### Check Status

```bash
sudo localstacker status
```

Shows:
- SSL certificate status
- Nginx config status
- Backend port status
- Service status (if configured)
- HTTPS connectivity

### Remove Domain

```bash
# Remove config (keep certificates)
sudo localstacker remove myapp.local

# Remove everything including certificates
sudo localstacker remove myapp.local --remove-certs
```

## Advanced Usage

### Custom Nginx Template

Create your template:

```bash
cat > my-template.conf << 'EOF'
server {
    listen 443 ssl http2;
    server_name {{domain}};

    ssl_certificate /etc/nginx/ssl/{{domain}}.pem;
    ssl_certificate_key /etc/nginx/ssl/{{domain}}-key.pem;

    # Your custom config
    client_max_body_size 100M;
    
    location / {
        proxy_pass http://127.0.0.1:{{port}};
        proxy_set_header Host $host;
    }
}
EOF
```

Use it:

```bash
sudo localstacker setup \
  --domain custom.local \
  --port 9000 \
  --template my-template.conf
```

### Dry Run (Preview Changes)

```bash
sudo localstacker setup \
  --domain test.local \
  --port 8080 \
  --dry-run \
  --verbose
```

### Auto-Confirm (CI/CD)

```bash
sudo localstacker setup \
  --domain ci.local \
  --port 5000 \
  --yes
```

## Troubleshooting

### Issue: "Permission denied"

**Solution:** Run with `sudo`

```bash
sudo localstacker setup --domain myapp.local --port 3000
```

### Issue: "Port already in use"

**Check what's using the port:**

```bash
sudo ss -tlnp | grep :3000
```

**Solution:** Stop the conflicting service or use different port

### Issue: "Nginx test failed"

**Check Nginx config:**

```bash
sudo nginx -t
```

**View error log:**

```bash
sudo tail -f /var/log/nginx/error.log
```

### Issue: "Certificate not trusted"

**Reinstall mkcert CA:**

```bash
sudo localstacker install-mkcert --force
```

**For Firefox:** Import CA certificate manually (Firefox uses own certificate store)

### Issue: "Domain not accessible"

**Checklist:**
1. Is backend running? `curl http://localhost:3000`
2. Is domain in /etc/hosts? `cat /etc/hosts | grep myapp.local`
3. Is Nginx running? `sudo systemctl status nginx`
4. Check status: `sudo localstacker status myapp.local`

## Tips & Tricks

### Auto-start Services

If you have a systemd service, link it:

```bash
sudo localstacker setup \
  --domain myapp.local \
  --port 3000 \
  --service myapp.service
```

The tool will restart the service after setup.

### Wildcard Subdomains

Setup multiple subdomains for one app:

```bash
# Setup main domain
sudo localstacker setup --domain myapp.local --port 3000

# Add wildcard to /etc/hosts
echo "127.0.0.1 api.myapp.local" | sudo tee -a /etc/hosts
echo "127.0.0.1 admin.myapp.local" | sudo tee -a /etc/hosts
```

Then handle routing in your app.

### Quick Domain Check

```bash
# Check if domain is working
curl -k https://myapp.local

# Check with headers
curl -k -I https://myapp.local

# Check SSL info
openssl s_client -connect myapp.local:443 -servername myapp.local
```

### Backup Your Configuration

```bash
# Backup config
sudo cp -r /etc/nusacloud ~/nusacloud-backup

# Backup certificates
sudo cp -r /etc/nginx/ssl ~/ssl-backup
```

### Development Workflow

```bash
# Morning setup
sudo localstacker setup --domain dev.local --port 3000
npm run dev

# Check during development
sudo localstacker status dev.local

# End of day cleanup
sudo localstacker remove dev.local
```

## Next Steps

- Read [README.md](README.md) for full feature list
- Check [ARCHITECTURE.md](ARCHITECTURE.md) to understand the design
- See [examples/](examples/) for more configuration examples
- Contribute on GitHub!

## Getting Help

```bash
# General help
localstacker --help

# Command-specific help
localstacker setup --help
localstacker list --help
localstacker remove --help
localstacker status --help
```

---

Happy coding with HTTPS! 🚀🔐