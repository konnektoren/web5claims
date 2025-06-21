# Default recipe - shows available commands
default:
    @just --list

# Variables
ui_dir := "web5claims-ui"
main_project := "web5claims"

# 🏗️  BUILD COMMANDS

# Build everything (main project + UI)
build-all: build build-ui

# Build the main Rust project
build:
    @echo "🦀 Building main Rust project..."
    cargo build

# Build the Yew UI
build-ui:
    @echo "🎨 Building Yew UI..."
    cd {{ui_dir}} && trunk build

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
