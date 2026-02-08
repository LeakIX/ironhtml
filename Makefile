# ironhtml - Makefile

# =============================================================================
# Configuration
# =============================================================================

CARGO := cargo

# =============================================================================
# Default target
# =============================================================================

.PHONY: all
all: build ## Build the entire project

# =============================================================================
# Building
# =============================================================================

.PHONY: build
build: ## Build all crates
	@echo "Building..."
	$(CARGO) build --all-targets

.PHONY: build-release
build-release: ## Build in release mode
	@echo "Building release..."
	$(CARGO) build --all-targets --release

.PHONY: build-wasm
build-wasm: ## Build for WebAssembly target
	@echo "Building for WebAssembly..."
	$(CARGO) build --target wasm32-unknown-unknown

.PHONY: build-examples
build-examples: ## Build and run all examples
	@echo "Building examples..."
	$(CARGO) build --examples
	@echo "Running ironhtml-bootstrap examples..."
	$(CARGO) run --example landing_page > /dev/null
	$(CARGO) run --example wallet_dashboard > /dev/null
	$(CARGO) run --example bootstrap_docs > /dev/null
	@echo "All examples built successfully"

# =============================================================================
# Formatting
# =============================================================================

.PHONY: format
format: format-rust format-toml format-md ## Format all code
	@echo "Formatting complete"

.PHONY: format-rust
format-rust: ## Format Rust code with cargo fmt
	@echo "Formatting Rust code..."
	$(CARGO) fmt --all

.PHONY: format-toml
format-toml: ## Format TOML files with taplo
	@echo "Formatting TOML files..."
	taplo format

.PHONY: format-md
format-md: ## Format Markdown files with Prettier
	@echo "Formatting Markdown files..."
	npx prettier --write "*.md"

# =============================================================================
# Format Checking (for CI)
# =============================================================================

.PHONY: format-check
format-check: format-check-rust format-check-toml format-check-md ## Check formatting without modifying files
	@echo "Format check complete"

.PHONY: format-check-rust
format-check-rust: ## Check Rust formatting
	@echo "Checking Rust formatting..."
	$(CARGO) fmt --all --check

.PHONY: format-check-toml
format-check-toml: ## Check TOML formatting with taplo
	@echo "Checking TOML formatting..."
	taplo format --check

.PHONY: format-check-md
format-check-md: ## Check Markdown formatting with Prettier
	@echo "Checking Markdown formatting..."
	npx prettier --check "*.md"

# =============================================================================
# Linting
# =============================================================================

.PHONY: lint
lint: ## Lint all code with clippy
	@echo "Linting..."
	$(CARGO) clippy --all-targets -- -D warnings

# =============================================================================
# Testing
# =============================================================================

.PHONY: test
test: ## Run all tests
	@echo "Running tests..."
	$(CARGO) test --all-targets
	@echo "All tests passed"

.PHONY: test-doc
test-doc: ## Run documentation tests
	@echo "Running doc tests..."
	$(CARGO) test --doc --features macros

.PHONY: test-parse5
test-parse5: ## Run parse5 integration tests
	@echo "Running parse5 integration tests..."
	@cd tests/parse5 && npm install --silent && npm test

# =============================================================================
# Benchmarks
# =============================================================================

.PHONY: bench
bench: ## Run benchmarks
	@echo "Running benchmarks..."
	$(CARGO) bench -p ironhtml --features macros

# =============================================================================
# Documentation
# =============================================================================

.PHONY: doc
doc: ## Generate documentation
	@echo "Generating documentation..."
	$(CARGO) doc --no-deps

.PHONY: doc-open
doc-open: ## Generate and open documentation
	@echo "Generating and opening documentation..."
	$(CARGO) doc --no-deps --open

# =============================================================================
# Cleaning
# =============================================================================

.PHONY: clean
clean: ## Clean build artifacts
	@echo "Cleaning..."
	rm -rf target

# =============================================================================
# Publishing
# =============================================================================

# Crates must be published in dependency order.
# Sleep between publishes to let the crates.io index update.
PUBLISH_CRATES := \
	ironhtml-elements \
	ironhtml-attributes \
	ironhtml-macro \
	ironhtml-parser \
	ironhtml \
	ironhtml-bootstrap

.PHONY: publish
publish: ## Publish all crates to crates.io
	@for crate in $(PUBLISH_CRATES); do \
		echo "Publishing $$crate..."; \
		$(CARGO) publish -p $$crate || exit 1; \
		echo "Waiting for crates.io index to update..."; \
		sleep 30; \
	done
	@echo "All crates published successfully"

.PHONY: publish-dry-run
publish-dry-run: ## Dry-run publish for all crates
	$(CARGO) package --workspace --allow-dirty
	@echo "Dry-run complete — all crates are ready to publish"

# =============================================================================
# CI
# =============================================================================

.PHONY: ci
ci: format-check lint test build-examples test-parse5 ## Run all CI checks
	@echo "CI checks passed"

# =============================================================================
# Help
# =============================================================================

.PHONY: help
help: ## Show this help message
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
