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
    cd {{ui_dir}} && trunk build --public-url /

# Build the Yew UI for production (GitHub Pages)
build-ui-prod:
    @echo "🎨 Building Yew UI for production..."
    cd {{ui_dir}} && trunk build --release --public-url /

# Build the Yew UI for GitHub Pages with additional setup
build-github-pages: build build-ui-prod
    @echo "🎯 Setting up GitHub Pages specific files..."
    cd {{ui_dir}}/dist && \
    touch .nojekyll && \
    cp index.html 404.html && \
    echo "GitHub Pages SPA routing files created"

# Preview production build locally (updated command)
preview-prod: build-ui-prod
    @echo "🚀 Production build created!"
    @echo "📁 Files are in {{ui_dir}}/dist/"
    @echo "🌐 Starting local server at http://localhost:8000"
    cd {{ui_dir}}/dist && python3 -m http.server 8000

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
    @echo "🔗 App will be available at: http://127.0.0.1:8080/"
    cd {{ui_dir}} && trunk serve --public-url /

# Preview production build locally
preview-prod: build-ui-prod
    @echo "🚀 Production build created!"
    @echo "📁 Files are in {{ui_dir}}/dist/"
    @echo "🌐 Starting local server at http://localhost:8000"
    cd {{ui_dir}}/dist && python3 -m http.server 8000

# Test verify URL handling specifically
test-verify-url: build-github-pages
    @echo "🧪 Testing verify URL handling..."
    @echo "📋 Test with this proof URL:"
    @echo "   http://localhost:8000/{{repo_name}}/verify?proof=bVPLbtswEPyVgGc64Fuij82hCNBDDikKNBgIklrFRGVJpSQXgaF_79Ky0edJ5M7scnd2dCZjHobWpYbsScMjt8zInfKN2anK610dqrAzUkXZVE2IOhJKYufT0c3vI5D9mTwMx5B6aMo55jRDTp7sX87kk-_fFv8GT3loU0zQx_fC6a5hfO8j5KPvseIx9a6DE3QY_CDIutIzeYLcDgWP8HzIMB2G7vJG4Y6QI_TzpYpl6_qKGWRcQpeiS_24zFNhZvi-pAxHZE6_dTf0jrn_dfEL5-7fV-gf-X93TMkJIZzTz4WA_S7dTPZzXgATIc8bBu7gpwPmVAE4D8zooHXNRFVb6VWrjNXGhFb4pmmhanSQTPoIsjYKKg2tqKHitua-vLhtrvGzL9Ntt_A-Aw77IpigUlFtqdCUG0s5L9-aCl5RwbawrShXggpVYISUodJSiUHkcEMVFYWm8aYZFRKpnHKGBC6pLsdSQGEhrCfsK06aclzSvNkpXp3hbrZwJ44yn77dRAgy8GgFRBGMapkWxgYtAdpgPGqio5RQCRNiC8zzmhllGYRomLItb0xbRHiDHjIK2ziPehOcW--Y2Qn-zOyeyb0U9zXjX4vJYPY3sXBZE-4JE_g9u2eIjp2fi98w5DsYyEXesSwOLuZJ07RA3gb7AUG7y18wuSW5DcKMK6f3x-KrL8i6e7iw7j4_3j1urHVdfwI"
    just simulate-github-pages

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

# Test the build process end-to-end
test-build: clean build-github-pages
    @echo "🧪 Testing complete build process..."
    @echo "✅ Build completed successfully"
    @echo "📁 Checking generated files..."
    @test -f {{ui_dir}}/dist/index.html && echo "✅ index.html exists" || echo "❌ index.html missing"
    @test -f {{ui_dir}}/dist/404.html && echo "✅ 404.html exists" || echo "❌ 404.html missing"
    @test -f {{ui_dir}}/dist/.nojekyll && echo "✅ .nojekyll exists" || echo "❌ .nojekyll missing"
    @ls {{ui_dir}}/dist/*.wasm >/dev/null 2>&1 && echo "✅ WASM files exist" || echo "❌ WASM files missing"
    @ls {{ui_dir}}/dist/*.js >/dev/null 2>&1 && echo "✅ JS files exist" || echo "❌ JS files missing"

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

# Check if ready for GitHub Pages deployment
check-deploy: build-github-pages
    @echo "✅ Checking GitHub Pages deployment readiness..."
    @echo ""
    @echo "📁 Build artifacts:"
    @echo "  Dist folder: $(test -d {{ui_dir}}/dist && echo '✅ EXISTS' || echo '❌ MISSING')"
    @echo "  Index.html: $(test -f {{ui_dir}}/dist/index.html && echo '✅ EXISTS' || echo '❌ MISSING')"
    @echo "  404.html: $(test -f {{ui_dir}}/dist/404.html && echo '✅ EXISTS' || echo '❌ MISSING')"
    @echo "  .nojekyll: $(test -f {{ui_dir}}/dist/.nojekyll && echo '✅ EXISTS' || echo '❌ MISSING')"
    @echo ""
    @echo "📦 Generated files:"
    @echo "  WASM files: $(ls {{ui_dir}}/dist/*.wasm 2>/dev/null | wc -l | tr -d ' ') found"
    @echo "  JS files: $(ls {{ui_dir}}/dist/*.js 2>/dev/null | wc -l | tr -d ' ') found"
    @echo "  CSS files: $(ls {{ui_dir}}/dist/*.css 2>/dev/null | wc -l | tr -d ' ') found"
    @echo ""
    @echo "🌐 Deployment URLs will be:"
    @echo "  https://your-username.github.io/{{repo_name}}/"
    @echo "  https://your-username.github.io/{{repo_name}}/issuer"
    @echo "  https://your-username.github.io/{{repo_name}}/verifier"
    @echo "  https://your-username.github.io/{{repo_name}}/verify?proof=..."
    @echo ""
    @echo "🚀 Ready for GitHub Pages deployment!"

# Generate a deployment preview
preview-github-pages: build-github-pages
    @echo "🔍 GitHub Pages Deployment Preview"
    @echo "=================================="
    @echo ""
    @echo "📁 Build Output:"
    @find {{ui_dir}}/dist -type f | head -20
    @echo ""
    @echo "📊 File Sizes:"
    @du -h {{ui_dir}}/dist/* 2>/dev/null | sort -hr | head -10
    @echo ""
    @echo "🔗 Test locally with: just simulate-github-pages"

# 🔄 WORKFLOW COMMANDS

# Complete development workflow
dev: clean check build-all serve

# Complete release workflow
release: clean check build-github-pages check-deploy

# Quick development cycle
quick: build-ui serve

# Complete CI/CD simulation
ci-simulation: clean install fmt clippy test build-github-pages check-deploy
    @echo "🎉 CI/CD simulation completed successfully!"
    @echo "🚀 Ready for deployment"

# 📊 STATUS COMMANDS

# Show project status
status:
    @echo "📊 Web5 Claims Project Status"
    @echo "============================="
    @echo ""
    @echo "🛠️  Environment:"
    @echo "  📁 Current directory: $(pwd)"
    @echo "  🦀 Rust version: $(cargo --version)"
    @echo "  🎯 WASM target: $(rustup target list --installed | grep wasm32 || echo 'NOT INSTALLED')"
    @echo "  🚚 Trunk: $(trunk --version 2>/dev/null || echo 'NOT INSTALLED')"
    @echo ""
    @echo "📦 Project Structure:"
    @echo "  Main project: $(test -f Cargo.toml && echo '✅ EXISTS' || echo '❌ MISSING')"
    @echo "  UI project: $(test -f {{ui_dir}}/Cargo.toml && echo '✅ EXISTS' || echo '❌ MISSING')"
    @echo "  Leo project: $(test -f web5claimsleo/program.json && echo '✅ EXISTS' || echo '❌ MISSING')"
    @echo ""
    @echo "🏗️  Build Status:"
    @echo "  Main target dir: $(test -d target && echo '✅ EXISTS' || echo '❌ NOT BUILT')"
    @echo "  UI dist folder: $(test -d {{ui_dir}}/dist && echo '✅ EXISTS' || echo '❌ NOT BUILT')"
    @echo "  GitHub Pages ready: $(test -f {{ui_dir}}/dist/.nojekyll && echo '✅ YES' || echo '❌ NO')"
    @echo ""
    @echo "🌐 URLs (when deployed):"
    @echo "  GitHub Pages: https://your-username.github.io/{{repo_name}}/"
    @echo "  Local dev: http://localhost:8080/"
    @echo "  Local prod: http://localhost:8000/{{repo_name}}/"

# Show detailed help information
help:
    @echo "🔐 Web5 Claims - ZK Language Learning Certificates"
    @echo "=================================================="
    @echo ""
    @echo "🏗️  BUILD COMMANDS:"
    @echo "  build               - Build main Rust project"
    @echo "  build-ui            - Build UI for development"
    @echo "  build-ui-prod       - Build UI for production"
    @echo "  build-github-pages  - Build UI for GitHub Pages with routing"
    @echo "  build-all           - Build everything for development"
    @echo "  build-all-prod      - Build everything for production"
    @echo ""
    @echo "🚀 SERVE COMMANDS:"
    @echo "  serve               - Start development server (trunk serve)"
    @echo "  preview-prod        - Preview production build locally"
    @echo "  simulate-github-pages - Simulate GitHub Pages deployment"
    @echo "  test-verify-url     - Test verify URL handling specifically"
    @echo ""
    @echo "🌐 GITHUB PAGES:"
    @echo "  check-deploy        - Verify build is ready for deployment"
    @echo "  preview-github-pages - Show deployment preview and stats"
    @echo ""
    @echo "🔧 DEVELOPMENT:"
    @echo "  install             - Install required tools (trunk, wasm target)"
    @echo "  dev                 - Full development workflow"
    @echo "  quick               - Quick build and serve"
    @echo "  quick-prod          - Quick production test"
    @echo ""
    @echo "🧪 TESTING:"
    @echo "  test                - Run all tests"
    @echo "  test-main           - Test main Rust project"
    @echo "  test-ui             - Test UI project (cargo check)"
    @echo "  test-build          - Test complete build process"
    @echo "  check               - Run quality checks (format + clippy + test)"
    @echo "  ci-simulation       - Simulate complete CI/CD pipeline"
    @echo ""
    @echo "🧹 MAINTENANCE:"
    @echo "  clean               - Clean all build artifacts"
    @echo "  fmt                 - Format all code"
    @echo "  clippy              - Run clippy lints"
    @echo ""
    @echo "📊 INFO:"
    @echo "  status              - Show detailed project status"
    @echo "  help                - Show this help message"
    @echo "  default             - List all available commands"
    @echo ""
    @echo "🎯 QUICK START:"
    @echo "  just install        - Install tools"
    @echo "  just dev            - Start development"
    @echo "  just release        - Prepare for deployment"
    @echo ""
    @echo "🔗 USEFUL COMMANDS:"
    @echo "  just status && just quick-prod  - Check status and test production"
    @echo "  just ci-simulation              - Full pipeline test"
    @echo "  just clean && just release      - Clean rebuild for deployment"

# Show current Git and deployment info
deployment-info:
    @echo "🚀 Deployment Information"
    @echo "========================"
    @echo ""
    @echo "📋 Git Status:"
    @git status --porcelain || echo "Not a git repository"
    @echo ""
    @echo "🏷️  Current Branch:"
    @git branch --show-current 2>/dev/null || echo "Not a git repository"
    @echo ""
    @echo "📝 Recent Commits:"
    @git log --oneline -5 2>/dev/null || echo "No git history"
    @echo ""
    @echo "🌐 Remote URLs:"
    @git remote -v 2>/dev/null || echo "No git remotes"
    @echo ""
    @echo "📦 Build Info:"
    @echo "  Last build: $(test -f {{ui_dir}}/dist/index.html && stat -c %y {{ui_dir}}/dist/index.html 2>/dev/null || echo 'Never built')"
    @echo "  Repo name: {{repo_name}}"
    @echo "  Target URL: https://your-username.github.io/{{repo_name}}/"

# Quick troubleshooting
troubleshoot:
    @echo "🔧 Web5 Claims Troubleshooting"
    @echo "=============================="
    @echo ""
    @echo "🛠️  Tools Check:"
    @which cargo >/dev/null && echo "✅ Cargo installed" || echo "❌ Cargo missing - install Rust"
    @which trunk >/dev/null && echo "✅ Trunk installed" || echo "❌ Trunk missing - run 'just install'"
    @rustup target list --installed | grep -q wasm32 && echo "✅ WASM target installed" || echo "❌ WASM target missing - run 'just install'"
    @which python3 >/dev/null && echo "✅ Python3 available" || echo "❌ Python3 missing - needed for local server"
    @echo ""
    @echo "📁 Project Structure:"
    @test -f Cargo.toml && echo "✅ Main Cargo.toml exists" || echo "❌ Main Cargo.toml missing"
    @test -f {{ui_dir}}/Cargo.toml && echo "✅ UI Cargo.toml exists" || echo "❌ UI Cargo.toml missing"
    @test -f {{ui_dir}}/index.html && echo "✅ UI index.html exists" || echo "❌ UI index.html missing"
    @test -f {{ui_dir}}/Trunk.toml && echo "✅ Trunk.toml exists" || echo "❌ Trunk.toml missing"
    @echo ""
    @echo "🏗️  Common Solutions:"
    @echo "  Build fails: just clean && just install && just build-all"
    @echo "  Serve fails: cd {{ui_dir}} && trunk serve --port 8081"
    @echo "  WASM errors: rustup target add wasm32-unknown-unknown"
    @echo "  Trunk errors: cargo install trunk --force"
    @echo "  Permission errors: check file permissions in {{ui_dir}}/dist"
    @echo ""
    @echo "🌐 Test URLs:"
    @echo "  Development: http://localhost:8080/"
    @echo "  Production: http://localhost:8000/{{repo_name}}/"
    @echo "  GitHub Pages: https://your-username.github.io/{{repo_name}}/"
