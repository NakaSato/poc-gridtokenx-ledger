#!/bin/bash

# Build and Development Script for Energy Trading Ledger API

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔨 Energy Trading Ledger - Build Script${NC}"
echo "=========================================="

# Function to print status
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    print_error "Cargo is not installed. Please install Rust first."
    exit 1
fi

# Build the project
print_info "Building the project..."
cargo build --release
print_status "Build completed successfully"

# Run tests
print_info "Running tests..."
cargo test
print_status "Tests completed"

# Check formatting
print_info "Checking code formatting..."
cargo fmt --check || {
    print_warning "Code formatting issues found. Run 'cargo fmt' to fix them."
}

# Run clippy for linting
print_info "Running clippy for linting..."
cargo clippy -- -W clippy::all || {
    print_warning "Clippy found some issues. Please review them."
}

print_info "Available commands:"
echo "  cargo run --bin ledger               # Run the CLI demo"
echo "  cargo run --example token_system_demo     # Test the token system"
echo "  cargo run --example energy_trading_demo   # Test energy trading"
echo "  cargo run --example simple_feature_demo   # Test simple features"

print_status "Build script completed successfully!"
