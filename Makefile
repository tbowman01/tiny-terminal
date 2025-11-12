.PHONY: help build release run test lint fmt clean install check all watch

# Default target
.DEFAULT_GOAL := help

# Binary name
BINARY_NAME := tiny-terminal

# Cargo flags
CARGO := cargo
CARGO_FLAGS :=
RELEASE_FLAGS := --release

help: ## Show this help message
	@echo "tiny-terminal - Makefile commands"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

all: fmt lint test build ## Run format, lint, test, and build

build: ## Build debug binary
	@echo "Building debug binary..."
	$(CARGO) build $(CARGO_FLAGS)

release: ## Build optimized release binary
	@echo "Building release binary..."
	$(CARGO) build $(RELEASE_FLAGS)
	@echo "Binary location: target/release/$(BINARY_NAME)"
	@ls -lh target/release/$(BINARY_NAME) 2>/dev/null || true

run: ## Run the application in release mode
	@echo "Running $(BINARY_NAME)..."
	$(CARGO) run $(RELEASE_FLAGS)

run-debug: ## Run the application in debug mode
	@echo "Running $(BINARY_NAME) (debug)..."
	$(CARGO) run

test: ## Run all tests
	@echo "Running tests..."
	$(CARGO) test $(CARGO_FLAGS)

test-verbose: ## Run tests with verbose output
	@echo "Running tests (verbose)..."
	$(CARGO) test $(CARGO_FLAGS) -- --nocapture

lint: ## Run clippy for linting
	@echo "Running clippy..."
	$(CARGO) clippy $(RELEASE_FLAGS) -- -D warnings

lint-fix: ## Run clippy with automatic fixes
	@echo "Running clippy with fixes..."
	$(CARGO) clippy --fix $(RELEASE_FLAGS) --allow-dirty --allow-staged

fmt: ## Format code with rustfmt
	@echo "Formatting code..."
	$(CARGO) fmt

fmt-check: ## Check if code is formatted correctly
	@echo "Checking code format..."
	$(CARGO) fmt -- --check

check: fmt-check lint test ## Run all checks (format, lint, test)
	@echo "All checks passed!"

clean: ## Remove build artifacts
	@echo "Cleaning build artifacts..."
	$(CARGO) clean
	@echo "Clean complete."

install: release ## Install binary to ~/.cargo/bin
	@echo "Installing $(BINARY_NAME)..."
	$(CARGO) install --path .
	@echo "Installation complete. Binary available at: ~/.cargo/bin/$(BINARY_NAME)"

uninstall: ## Uninstall binary from ~/.cargo/bin
	@echo "Uninstalling $(BINARY_NAME)..."
	$(CARGO) uninstall $(BINARY_NAME)

watch: ## Watch for changes and rebuild (requires cargo-watch)
	@echo "Watching for changes..."
	$(CARGO) watch -x build

watch-test: ## Watch for changes and run tests (requires cargo-watch)
	@echo "Watching for changes and running tests..."
	$(CARGO) watch -x test

watch-run: ## Watch for changes and run application (requires cargo-watch)
	@echo "Watching for changes and running application..."
	$(CARGO) watch -x "run $(RELEASE_FLAGS)"

audit: ## Run cargo audit for security vulnerabilities
	@echo "Running security audit..."
	$(CARGO) audit

update: ## Update dependencies
	@echo "Updating dependencies..."
	$(CARGO) update

outdated: ## Check for outdated dependencies (requires cargo-outdated)
	@echo "Checking for outdated dependencies..."
	$(CARGO) outdated

bloat: ## Analyze binary size (requires cargo-bloat)
	@echo "Analyzing binary size..."
	$(CARGO) bloat $(RELEASE_FLAGS)

bench: ## Run benchmarks
	@echo "Running benchmarks..."
	$(CARGO) bench

doc: ## Generate and open documentation
	@echo "Generating documentation..."
	$(CARGO) doc --no-deps --open

doc-private: ## Generate documentation including private items
	@echo "Generating documentation (including private)..."
	$(CARGO) doc --no-deps --document-private-items --open

coverage: ## Generate code coverage report (requires cargo-tarpaulin)
	@echo "Generating coverage report..."
	$(CARGO) tarpaulin --out Html --output-dir coverage

# Example runs with different configurations
run-sparse: ## Run with sparse configuration
	@echo "Running with sparse configuration..."
	$(CARGO) run $(RELEASE_FLAGS) -- --density 0.5 --fps 30

run-dense: ## Run with dense configuration
	@echo "Running with dense configuration..."
	$(CARGO) run $(RELEASE_FLAGS) -- --density 1.75 --fps 90

run-example: ## Run with example configuration
	@echo "Running with example configuration..."
	$(CARGO) run $(RELEASE_FLAGS) -- --config ./examples/project-config/.tiny-terminal.toml

# CI/CD helpers
ci-check: ## Run all CI checks locally
	@echo "Running CI checks..."
	@$(MAKE) fmt-check
	@$(MAKE) lint
	@$(MAKE) test
	@$(MAKE) build
	@echo "✓ All CI checks passed!"

pre-commit: check ## Run pre-commit checks (format, lint, test)
	@echo "✓ Pre-commit checks complete!"

# Version management
version: ## Show current version
	@grep '^version' Cargo.toml | head -1 | cut -d'"' -f2

# Development setup
setup: ## Set up development environment
	@echo "Setting up development environment..."
	@rustup component add rustfmt clippy
	@echo "✓ Development environment ready!"

setup-tools: ## Install helpful cargo tools
	@echo "Installing cargo tools..."
	@cargo install cargo-watch cargo-audit cargo-outdated cargo-bloat cargo-tarpaulin 2>/dev/null || true
	@echo "✓ Cargo tools installed!"
