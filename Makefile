# UniLang Build System
# Requires: Rust 1.75+, JDK 21+, Python 3.11+

.PHONY: all build build-debug test lint fmt check ci clean dev-setup

# Default target
all: build

# Build the compiler (release mode)
build:
	cargo build --release

# Build the compiler (debug mode)
build-debug:
	cargo build

# Build specific components
build-lexer:
	cargo build -p unilang-lexer

build-parser:
	cargo build -p unilang-parser

build-runtime:
	cargo build -p unilang-runtime

# Run all tests
test:
	cargo test --all
	@echo "All tests passed."

# Run specific test suites
test-lexer:
	cargo test -p unilang-lexer

test-parser:
	cargo test -p unilang-parser

test-integration:
	cargo test --test integration

test-e2e:
	cargo test --test e2e

# Run benchmarks
bench:
	cargo bench --all

# Format code
fmt:
	cargo fmt --all
	@echo "Code formatted."

# Check formatting without modifying
fmt-check:
	cargo fmt --all -- --check

# Lint
lint:
	cargo clippy --all -- -D warnings
	@echo "Lint passed."

# Type check (no build)
check:
	cargo check --all

# Full CI pipeline
ci: fmt-check lint test build
	@echo "CI pipeline passed."

# Clean build artifacts
clean:
	cargo clean
	rm -rf build/
	find . -name "__pycache__" -type d -exec rm -rf {} + 2>/dev/null || true
	find . -name "*.pyc" -delete 2>/dev/null || true
	@echo "Clean complete."

# Development environment setup
dev-setup:
	@echo "Setting up UniLang development environment..."
	@command -v rustc >/dev/null 2>&1 || { echo "Error: Rust not found. Install from https://rustup.rs"; exit 1; }
	@command -v java >/dev/null 2>&1 || { echo "Error: Java not found. Install JDK 21+"; exit 1; }
	@command -v python3 >/dev/null 2>&1 || { echo "Error: Python 3 not found. Install Python 3.11+"; exit 1; }
	@echo "Rust: $$(rustc --version)"
	@echo "Java: $$(java --version 2>&1 | head -1)"
	@echo "Python: $$(python3 --version)"
	@echo ""
	@echo "Development environment ready."

# Generate documentation
docs:
	cargo doc --no-deps --open

# Run a .uniL file (placeholder)
run:
	@echo "Usage: make run FILE=path/to/file.uniL"
	@echo "Note: Compiler not yet implemented."

# Help
help:
	@echo "UniLang Build System"
	@echo ""
	@echo "Targets:"
	@echo "  build        Build compiler (release)"
	@echo "  build-debug  Build compiler (debug)"
	@echo "  test         Run all tests"
	@echo "  lint         Run linter"
	@echo "  fmt          Format code"
	@echo "  check        Type check"
	@echo "  ci           Full CI pipeline"
	@echo "  clean        Remove build artifacts"
	@echo "  dev-setup    Verify development prerequisites"
	@echo "  docs         Generate and open documentation"
	@echo "  help         Show this help"
