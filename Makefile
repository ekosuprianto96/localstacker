.PHONY: help build install clean test check run dev release fmt clippy

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build the project in debug mode
	cargo build

release: ## Build optimized release binary
	cargo build --release
	@echo ""
	@echo "✅ Binary available at: target/release/localstacker"

install: release ## Build and install to /usr/local/bin (requires sudo)
	@echo "📦 Installing to /usr/local/bin..."
	sudo cp target/release/localstacker /usr/local/bin/
	sudo chmod +x /usr/local/bin/localstacker
	@echo "✅ Installed successfully!"
	@echo "   Run: localstacker --help"

clean: ## Remove build artifacts
	cargo clean
	rm -rf target/

test: ## Run tests
	cargo test

check: ## Run checks (clippy + fmt check)
	cargo clippy -- -D warnings
	cargo fmt -- --check

fmt: ## Format code
	cargo fmt

clippy: ## Run clippy linter
	cargo clippy -- -D warnings

dev: ## Build and run with example args (dry-run)
	cargo build
	sudo ./target/debug/localstacker setup \
		--domain example.local \
		--port 3000 \
		--verbose \
		--dry-run

run: build ## Quick build and run help
	./target/debug/localstacker --help

# Development shortcuts
setup-dev: ## Setup development domain (requires sudo)
	sudo cargo run -- setup --domain dev.local --port 8080 --verbose

list-dev: ## List configured domains
	sudo cargo run -- list --detailed

status-dev: ## Check status of configured domains
	sudo cargo run -- status

remove-dev: ## Remove dev.local domain (dry-run)
	sudo cargo run -- remove dev.local --dry-run --verbose

# Documentation
docs: ## Generate and open documentation
	cargo doc --open

# Release preparation
pre-release: clean fmt clippy test release ## Run all checks before release
	@echo ""
	@echo "✅ All checks passed! Ready for release."
	@echo "   Binary size: $$(du -h target/release/localstacker | cut -f1)"