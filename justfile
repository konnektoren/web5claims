# Default recipe - shows available commands
default:
    @just --list

# Variables
ui_dir := "web5claims-ui"
main_project := "web5claims"
repo_name := "web5claims"

# 🏗️  BUILD COMMANDS

# Build everything (main project + UI)
build-all: build build-ui

# Build the main Rust project
build:
    @echo "🦀 Building main Rust project..."
    cargo build

# Build the Yew UI for development
build-ui:
    @echo "🎨 Building Yew UI for development..."
    cd {{ui_dir}} && trunk build

# Build the Yew UI for production (GitHub Pages)
build-ui-prod:
    @echo "🎨 Building Yew UI for production..."
    cd {{ui_dir}} && trunk build --release --public-url /{{repo_name}}/

# Build everything for production
build-all-prod: build build-ui-prod

# Clean all build artifacts
clean:
    @echo "🧹 Cleaning build artifacts..."
    cargo clean
    cd {{ui_dir}} && rm -rf dist

# 🚀 SERVE COMMANDS

# Serve the UI in development mode
serve:
    @echo "🌐 Starting development server..."
    cd {{ui_dir}} && trunk serve

# Preview production build locally
preview-prod: build-ui-prod
    @echo "🚀 Production build created!"
    @echo "📁 Files are in {{ui_dir}}/dist/"
    @echo "🌐 Starting local server at http://localhost:8000"
    cd {{ui_dir}}/dist && python3 -m http.server 8000

# 🧪 TEST COMMANDS

# Run all tests
test: test-main test-ui

# Test the main Rust project
test-main:
    @echo "🧪 Running main project tests..."
    cargo test

# Test the UI (lint check for now, since Yew testing is complex)
test-ui:
    @echo "🧪 Checking UI code..."
    cd {{ui_dir}} && cargo check

# 🔧 DEVELOPMENT COMMANDS

# Install required tools
install:
    @echo "📦 Installing required tools..."
    cargo install trunk
    rustup target add wasm32-unknown-unknown

# Format all code
fmt:
    @echo "✨ Formatting code..."
    cargo fmt --all

# Run clippy on all code
clippy:
    @echo "🔍 Running clippy..."
    cargo clippy --all-targets --all-features
    cd {{ui_dir}} && cargo clippy

# Check code quality (format + clippy + test)
check: fmt clippy test

# 🌐 GITHUB PAGES COMMANDS

# Simulate GitHub Pages deployment locally
simulate-deploy: build-ui-prod
    @echo "🎭 Simulating GitHub Pages deployment..."
    @echo "🌐 Your app will be available at: http://localhost:8000"
    @echo "📝 This simulates: https://your-username.github.io/{{repo_name}}/"
    cd {{ui_dir}}/dist && python3 -m http.server 8000

# Check if ready for GitHub Pages deployment
check-deploy: build-ui-prod
    @echo "✅ Checking deployment readiness..."
    @echo "📁 Dist folder exists: $(test -d {{ui_dir}}/dist && echo 'YES' || echo 'NO')"
    @echo "📄 Index.html exists: $(test -f {{ui_dir}}/dist/index.html && echo 'YES' || echo 'NO')"
    @echo "🎯 WASM files: $(ls {{ui_dir}}/dist/*.wasm 2>/dev/null | wc -l) found"
    @echo "📦 JS files: $(ls {{ui_dir}}/dist/*.js 2>/dev/null | wc -l) found"
    @echo ""
    @echo "🚀 Ready for GitHub Pages deployment!"

# 🔄 WORKFLOW COMMANDS

# Complete development workflow
dev: clean check build-all serve

# Complete release workflow
release: clean check build-all-prod check-deploy

# Quick development cycle
quick: build-ui serve

# 📊 STATUS COMMANDS

# Show project status
status:
    @echo "📊 Project Status:"
    @echo "  📁 Current directory: $(pwd)"
    @echo "  🦀 Rust version: $(cargo --version)"
    @echo "  🎯 WASM target: $(rustup target list --installed | grep wasm32 || echo 'NOT INSTALLED')"
    @echo "  🚚 Trunk: $(trunk --version 2>/dev/null || echo 'NOT INSTALLED')"
    @echo "  📦 Main project: $(test -f Cargo.toml && echo 'EXISTS' || echo 'MISSING')"
    @echo "  🎨 UI project: $(test -f {{ui_dir}}/Cargo.toml && echo 'EXISTS' || echo 'MISSING')"
    @echo "  🌐 Dist folder: $(test -d {{ui_dir}}/dist && echo 'EXISTS' || echo 'NOT BUILT')"

# Show help information
help:
    @echo "🔐 Web5 Claims - ZK Language Learning Certificates"
    @echo ""
    @echo "🏗️  BUILD COMMANDS:"
    @echo "  build          - Build main Rust project"
    @echo "  build-ui       - Build UI for development"
    @echo "  build-ui-prod  - Build UI for production (GitHub Pages)"
    @echo "  build-all      - Build everything for development"
    @echo "  build-all-prod - Build everything for production"
    @echo ""
    @echo "🚀 SERVE COMMANDS:"
    @echo "  serve          - Start development server"
    @echo "  preview-prod   - Preview production build locally"
    @echo ""
    @echo "🌐 GITHUB PAGES:"
    @echo "  simulate-deploy - Test GitHub Pages deployment locally"
    @echo "  check-deploy   - Verify build is ready for deployment"
    @echo ""
    @echo "🔧 DEVELOPMENT:"
    @echo "  install        - Install required tools"
    @echo "  dev           - Full development workflow"
    @echo "  quick         - Quick build and serve"
    @echo ""
    @echo "🧪 TESTING:"
    @echo "  test          - Run all tests"
    @echo "  check         - Run quality checks (format + clippy + test)"
    @echo ""
    @echo "📊 INFO:"
    @echo "  status        - Show project status"
    @echo "  help          - Show this help message"
