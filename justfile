# Default recipe - shows available commands
default:
    @just --list

# Variables
ui_dir := "web5claims-ui"
zkpass_dir := "zkpass"
main_project := "web5claims"
repo_name := "web5claims"

# 🏗️  BUILD COMMANDS

# Build everything (main project + UI + ZKPass)
build-all: build build-ui build-zkpass

# Build the main Rust project
build:
    @echo "🦀 Building main Rust project..."
    cargo build

# Build the Yew UI for development
build-ui:
    @echo "🎨 Building Yew UI for development..."
    cd {{ui_dir}} && trunk build --public-url /

# Build ZKPass for development
build-zkpass:
    @echo "🛂 Building ZKPass for development..."
    cd {{zkpass_dir}} && npm install && npm run build

# Build the Yew UI for production (GitHub Pages)
build-ui-prod:
    @echo "🎨 Building Yew UI for production..."
    cd {{ui_dir}} && trunk build --release --public-url /

# Build ZKPass for production (GitHub Pages)
build-zkpass-prod:
    @echo "🛂 Building ZKPass for production..."
    cd {{zkpass_dir}} && npm install && npm run build:github-pages
    @echo "🔍 Checking ZKPass build output..."
    @ls -la {{zkpass_dir}}/dist/ || echo "No dist folder"
    @ls -la {{zkpass_dir}}/dist/assets/ || echo "No assets folder"
    @echo "📄 Generated index.html:"
    @head -10 {{zkpass_dir}}/dist/index.html || echo "No index.html"

# Build the Yew UI for GitHub Pages with additional setup
build-github-pages: build build-ui-prod build-zkpass-prod
    @echo "🎯 Setting up GitHub Pages specific files..."
    cd {{ui_dir}}/dist && \
    touch .nojekyll && \
    cp index.html 404.html && \
    echo "GitHub Pages SPA routing files created"
    @echo "🛂 Copying ZKPass build to GitHub Pages..."
    mkdir -p {{ui_dir}}/dist/zkpass
    cp -r {{zkpass_dir}}/dist/* {{ui_dir}}/dist/zkpass/
    @echo "✅ ZKPass integrated into GitHub Pages build"
    @echo "🔍 Checking ZKPass assets..."
    @ls -la {{ui_dir}}/dist/zkpass/assets/ || echo "No assets folder found"
    @echo "📄 ZKPass index.html content:"
    @head -20 {{ui_dir}}/dist/zkpass/index.html || echo "No index.html found"

# Preview production build locally (updated command)
preview-prod: build-github-pages
    @echo "🚀 Production build created!"
    @echo "📁 Files are in {{ui_dir}}/dist/"
    @echo "🌐 Starting local server at http://localhost:8000"
    @echo "🛂 ZKPass available at: http://localhost:8000/zkpass/"
    cd {{ui_dir}}/dist && python3 -m http.server 8000

# Build everything for production
build-all-prod: build build-ui-prod build-zkpass-prod

# Clean all build artifacts
clean:
    @echo "🧹 Cleaning build artifacts..."
    cargo clean
    cd {{ui_dir}} && rm -rf dist
    cd {{zkpass_dir}} && rm -rf dist node_modules

# 🚀 SERVE COMMANDS

# Serve the UI in development mode
serve:
    @echo "🌐 Starting development server..."
    @echo "🔗 App will be available at: http://127.0.0.1:8080/"
    cd {{ui_dir}} && trunk serve --public-url /

# Serve ZKPass in development mode
serve-zkpass:
    @echo "🛂 Starting ZKPass development server..."
    @echo "🔗 ZKPass will be available at: http://localhost:8000/"
    cd {{zkpass_dir}} && npm run dev

# Serve both UI and ZKPass (requires two terminals)
serve-all:
    @echo "🌐 Starting both servers..."
    @echo "🔗 Main UI: http://127.0.0.1:8080/"
    @echo "🛂 ZKPass: http://localhost:8000/"
    @echo "⚠️  Note: Run 'just serve' and 'just serve-zkpass' in separate terminals"

# Test verify URL handling specifically
test-verify-url: build-github-pages
    @echo "🧪 Testing verify URL handling..."
    @echo "📋 Test with this proof URL:"
    @echo "   http://localhost:8000/{{repo_name}}/verify?proof=bVPLbtswEPyVgGc64Fuij82hCNBDDikKNBgIklrFRGVJpSQXgaF_79Ky0edJ5M7scnd2dCZjHobWpYbsScMjt8zInfKN2anK610dqrAzUkXZVE2IOhJKYufT0c3vI5D9mTwMx5B6aMo55jRDTp7sX87kk-_fFv8GT3loU0zQx_fC6a5hfO8j5KPvseIx9a6DE3QY_CDIutIzeYLcDgWP8HzIMB2G7vJG4Y6QI_TzpYpl6_qKGWRcQpeiS_24zFNhZvi-pAxHZE6_dTf0jrn_dfEL5-7fV-gf-X93TMkJIZzTz4WA_S7dTPZzXgATIc8bBu7gpwPmVAE4D8zooHXNRFVb6VWrjNXGhFb4pmmhanSQTPoIsjYKKg2tqKHitua-vLhtrvGzL9Ntt_A-Aw77IpigUlFtqdCUG0s5L9-aCl5RwbawrShXggpVYISUodJSiUHkcEMVFYWm8aYZFRKpnHKGBC6pLsdSQGEhrCfsK06aclzSvNkpXp3hbrZwJ44yn77dRAgy8GgFRBGMapkWxgYtAdpgPGqio5RQCRNiC8zzmhllGYRomLItb0xbRHiDHjIK2ziPehOcW--Y2Qn-zOyeyb0U9zXjX4vJYPY3sXBZE-4JE_g9u2eIjp2fi98w5DsYyEXesSwOLuZJ07RA3gb7AUG7y18wuSW5DcKMK6f3x-KrL8i6e7iw7j4_3j1urHVdfwI"
    @echo "🛂 ZKPass available at: http://localhost:8000/zkpass/"
    just preview-prod

# 🧪 TEST COMMANDS

# Run all tests
test: test-main test-ui test-zkpass

# Test the main Rust project
test-main:
    @echo "🧪 Running main project tests..."
    cargo test

# Test the UI (lint check for now, since Yew testing is complex)
test-ui:
    @echo "🧪 Checking UI code..."
    cd {{ui_dir}} && cargo check

# Test ZKPass (lint and build check)
test-zkpass:
    @echo "🧪 Testing ZKPass..."
    cd {{zkpass_dir}} && npm install && npm run build

# Test the build process end-to-end
test-build: clean build-github-pages
    @echo "🧪 Testing complete build process..."
    @echo "✅ Build completed successfully"
    @echo "📁 Checking generated files..."
    @test -f {{ui_dir}}/dist/index.html && echo "✅ index.html exists" || echo "❌ index.html missing"
    @test -f {{ui_dir}}/dist/404.html && echo "✅ 404.html exists" || echo "❌ 404.html missing"
    @test -f {{ui_dir}}/dist/.nojekyll && echo "✅ .nojekyll exists" || echo "❌ .nojekyll missing"
    @test -d {{ui_dir}}/dist/zkpass && echo "✅ ZKPass folder exists" || echo "❌ ZKPass folder missing"
    @test -f {{ui_dir}}/dist/zkpass/index.html && echo "✅ ZKPass index.html exists" || echo "❌ ZKPass index.html missing"
    @ls {{ui_dir}}/dist/*.wasm >/dev/null 2>&1 && echo "✅ WASM files exist" || echo "❌ WASM files missing"
    @ls {{ui_dir}}/dist/*.js >/dev/null 2>&1 && echo "✅ JS files exist" || echo "❌ JS files missing"

# 🔧 DEVELOPMENT COMMANDS

# Install required tools
install:
    @echo "📦 Installing required tools..."
    cargo install trunk
    rustup target add wasm32-unknown-unknown
    cd {{zkpass_dir}} && npm install

# Install only ZKPass dependencies
install-zkpass:
    @echo "📦 Installing ZKPass dependencies..."
    cd {{zkpass_dir}} && npm install

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
    @echo "  ZKPass folder: $(test -d {{ui_dir}}/dist/zkpass && echo '✅ EXISTS' || echo '❌ MISSING')"
    @echo "  ZKPass index: $(test -f {{ui_dir}}/dist/zkpass/index.html && echo '✅ EXISTS' || echo '❌ MISSING')"
    @echo ""
    @echo "📦 Generated files:"
    @echo "  WASM files: $(ls {{ui_dir}}/dist/*.wasm 2>/dev/null | wc -l | tr -d ' ') found"
    @echo "  JS files: $(ls {{ui_dir}}/dist/*.js 2>/dev/null | wc -l | tr -d ' ') found"
    @echo "  CSS files: $(ls {{ui_dir}}/dist/*.css 2>/dev/null | wc -l | tr -d ' ') found"
    @echo "  ZKPass assets: $(ls {{ui_dir}}/dist/zkpass/assets/* 2>/dev/null | wc -l | tr -d ' ') found"
    @echo ""
    @echo "🌐 Deployment URLs will be:"
    @echo "  https://your-username.github.io/{{repo_name}}/"
    @echo "  https://your-username.github.io/{{repo_name}}/issuer"
    @echo "  https://your-username.github.io/{{repo_name}}/verifier"
    @echo "  https://your-username.github.io/{{repo_name}}/verify?proof=..."
    @echo "  https://your-username.github.io/{{repo_name}}/zkpass/"
    @echo ""
    @echo "🚀 Ready for GitHub Pages deployment!"

# Generate a deployment preview
preview-github-pages: build-github-pages
    @echo "🔍 GitHub Pages Deployment Preview"
    @echo "=================================="
    @echo ""
    @echo "📁 Build Output:"
    @find {{ui_dir}}/dist -type f | head -30
    @echo ""
    @echo "📊 File Sizes:"
    @du -h {{ui_dir}}/dist/* 2>/dev/null | sort -hr | head -15
    @echo ""
    @echo "🛂 ZKPass Integration:"
    @echo "  ZKPass files: $(find {{ui_dir}}/dist/zkpass -name '*.js' -o -name '*.css' -o -name '*.html' | wc -l | tr -d ' ') files"
    @echo "  ZKPass size: $(du -sh {{ui_dir}}/dist/zkpass 2>/dev/null | cut -f1)"
    @echo ""
    @echo "🔗 Test locally with: just preview-prod"

# 🔄 WORKFLOW COMMANDS

# Complete development workflow
dev: clean check build-all serve

# Complete release workflow
release: clean check build-github-pages check-deploy

# Quick development cycle
quick: build-ui serve

# Quick ZKPass development cycle
quick-zkpass: build-zkpass serve-zkpass

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
    @echo "  📦 Node.js: $(node --version 2>/dev/null || echo 'NOT INSTALLED')"
    @echo "  📦 NPM: $(npm --version 2>/dev/null || echo 'NOT INSTALLED')"
    @echo ""
    @echo "📦 Project Structure:"
    @echo "  Main project: $(test -f Cargo.toml && echo '✅ EXISTS' || echo '❌ MISSING')"
    @echo "  UI project: $(test -f {{ui_dir}}/Cargo.toml && echo '✅ EXISTS' || echo '❌ MISSING')"
    @echo "  ZKPass project: $(test -f {{zkpass_dir}}/package.json && echo '✅ EXISTS' || echo '❌ MISSING')"
    @echo "  Leo project: $(test -f web5claimsleo/program.json && echo '✅ EXISTS' || echo '❌ MISSING')"
    @echo ""
    @echo "🏗️  Build Status:"
    @echo "  Main target dir: $(test -d target && echo '✅ EXISTS' || echo '❌ NOT BUILT')"
    @echo "  UI dist folder: $(test -d {{ui_dir}}/dist && echo '✅ EXISTS' || echo '❌ NOT BUILT')"
    @echo "  ZKPass dist folder: $(test -d {{zkpass_dir}}/dist && echo '✅ EXISTS' || echo '❌ NOT BUILT')"
    @echo "  ZKPass node_modules: $(test -d {{zkpass_dir}}/node_modules && echo '✅ EXISTS' || echo '❌ NOT INSTALLED')"
    @echo "  GitHub Pages ready: $(test -f {{ui_dir}}/dist/.nojekyll && echo '✅ YES' || echo '❌ NO')"
    @echo ""
    @echo "🌐 URLs (when deployed):"
    @echo "  GitHub Pages: https://your-username.github.io/{{repo_name}}/"
    @echo "  ZKPass: https://your-username.github.io/{{repo_name}}/zkpass/"
    @echo "  Local dev UI: http://localhost:8080/"
    @echo "  Local dev ZKPass: http://localhost:8000/"
    @echo "  Local prod: http://localhost:8000/{{repo_name}}/"

# Show detailed help information
help:
    @echo "🔐 Web5 Claims - ZK Language Learning Certificates"
    @echo "=================================================="
    @echo ""
    @echo "🏗️  BUILD COMMANDS:"
    @echo "  build               - Build main Rust project"
    @echo "  build-ui            - Build UI for development"
    @echo "  build-zkpass        - Build ZKPass for development"
    @echo "  build-ui-prod       - Build UI for production"
    @echo "  build-zkpass-prod   - Build ZKPass for production"
    @echo "  build-github-pages  - Build everything for GitHub Pages"
    @echo "  build-all           - Build everything for development"
    @echo "  build-all-prod      - Build everything for production"
    @echo ""
    @echo "🚀 SERVE COMMANDS:"
    @echo "  serve               - Start main UI development server"
    @echo "  serve-zkpass        - Start ZKPass development server"
    @echo "  serve-all           - Instructions for running both servers"
    @echo "  preview-prod        - Preview production build locally"
    @echo "  test-verify-url     - Test verify URL handling"
    @echo ""
    @echo "🌐 GITHUB PAGES:"
    @echo "  check-deploy        - Verify build is ready for deployment"
    @echo "  preview-github-pages - Show deployment preview and stats"
    @echo ""
    @echo "🔧 DEVELOPMENT:"
    @echo "  install             - Install all required tools"
    @echo "  install-zkpass      - Install only ZKPass dependencies"
    @echo "  dev                 - Full development workflow"
    @echo "  quick               - Quick UI build and serve"
    @echo "  quick-zkpass        - Quick ZKPass build and serve"
    @echo ""
    @echo "🧪 TESTING:"
    @echo "  test                - Run all tests"
    @echo "  test-main           - Test main Rust project"
    @echo "  test-ui             - Test UI project"
    @echo "  test-zkpass         - Test ZKPass project"
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
    @echo "  just install        - Install all tools and dependencies"
    @echo "  just dev            - Start development (UI)"
    @echo "  just serve-zkpass   - Start ZKPass development server"
    @echo "  just release        - Prepare for deployment"
    @echo ""
    @echo "🔗 USEFUL COMMANDS:"
    @echo "  just status && just preview-prod    - Check status and test production"
    @echo "  just ci-simulation                  - Full pipeline test"
    @echo "  just clean && just release          - Clean rebuild for deployment"

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
    @echo "  Last UI build: $(test -f {{ui_dir}}/dist/index.html && stat -c %y {{ui_dir}}/dist/index.html 2>/dev/null || echo 'Never built')"
    @echo "  Last ZKPass build: $(test -f {{zkpass_dir}}/dist/index.html && stat -c %y {{zkpass_dir}}/dist/index.html 2>/dev/null || echo 'Never built')"
    @echo "  Repo name: {{repo_name}}"
    @echo "  Target URL: https://your-username.github.io/{{repo_name}}/"
    @echo "  ZKPass URL: https://your-username.github.io/{{repo_name}}/zkpass/"

# Quick troubleshooting
troubleshoot:
    @echo "🔧 Web5 Claims Troubleshooting"
    @echo "=============================="
    @echo ""
    @echo "🛠️  Tools Check:"
    @which cargo >/dev/null && echo "✅ Cargo installed" || echo "❌ Cargo missing - install Rust"
    @which trunk >/dev/null && echo "✅ Trunk installed" || echo "❌ Trunk missing - run 'just install'"
    @rustup target list --installed | grep -q wasm32 && echo "✅ WASM target installed" || echo "❌ WASM target missing - run 'just install'"
    @which node >/dev/null && echo "✅ Node.js installed" || echo "❌ Node.js missing - install from nodejs.org"
    @which npm >/dev/null && echo "✅ NPM installed" || echo "❌ NPM missing - install Node.js"
    @which python3 >/dev/null && echo "✅ Python3 available" || echo "❌ Python3 missing - needed for local server"
    @echo ""
    @echo "📁 Project Structure:"
    @test -f Cargo.toml && echo "✅ Main Cargo.toml exists" || echo "❌ Main Cargo.toml missing"
    @test -f {{ui_dir}}/Cargo.toml && echo "✅ UI Cargo.toml exists" || echo "❌ UI Cargo.toml missing"
    @test -f {{ui_dir}}/index.html && echo "✅ UI index.html exists" || echo "❌ UI index.html missing"
    @test -f {{ui_dir}}/Trunk.toml && echo "✅ Trunk.toml exists" || echo "❌ Trunk.toml missing"
    @test -f {{zkpass_dir}}/package.json && echo "✅ ZKPass package.json exists" || echo "❌ ZKPass package.json missing"
    @test -f {{zkpass_dir}}/index.html && echo "✅ ZKPass index.html exists" || echo "❌ ZKPass index.html missing"
    @test -d {{zkpass_dir}}/node_modules && echo "✅ ZKPass dependencies installed" || echo "❌ ZKPass dependencies missing - run 'just install-zkpass'"
    @echo ""
    @echo "🏗️  Common Solutions:"
    @echo "  Build fails: just clean && just install && just build-all"
    @echo "  UI serve fails: cd {{ui_dir}} && trunk serve --port 8081"
    @echo "  ZKPass serve fails: cd {{zkpass_dir}} && npm run dev -- --port 8001"
    @echo "  WASM errors: rustup target add wasm32-unknown-unknown"
    @echo "  Trunk errors: cargo install trunk --force"
    @echo "  NPM errors: cd {{zkpass_dir}} && rm -rf node_modules && npm install"
    @echo "  Permission errors: check file permissions in dist folders"
    @echo ""
    @echo "🌐 Test URLs:"
    @echo "  Development UI: http://localhost:8080/"
    @echo "  Development ZKPass: http://localhost:8000/"
    @echo "  Production: http://localhost:8000/{{repo_name}}/"
    @echo "  Production ZKPass: http://localhost:8000/{{repo_name}}/zkpass/"
    @echo "  GitHub Pages: https://your-username.github.io/{{repo_name}}/"
    @echo "  GitHub Pages ZKPass: https://your-username.github.io/{{repo_name}}/zkpass/"
