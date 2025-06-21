## 🏗 Development Setup

### Prerequisites

- Rust and Cargo
- Node.js and npm (for UI dependencies)
- Leo (for Aleo blockchain deployment)

### Leo/Aleo Setup

```bash
# Install Leo (Aleo's programming language)
curl -L https://raw.githubusercontent.com/AleoHQ/leo/main/install.sh | sh

# Verify Leo installation
leo --version

# Navigate to the Leo program directory
cd web5claimsleo
```

### Building the Project

```bash
# Build the main Rust project
cargo build

# Build the Leo program for Aleo
cd web5claimsleo
leo build

# Build the Yew UI for development
cd ../web5claims-ui
trunk build

# Or use justfile for convenience
just build-all
```

### Testing

```bash
# Test the main Rust project
cargo test

# Test the Leo program
cd web5claimsleo
leo test

# Run all tests
just test
```

### Leo Program Development

The Leo program (`web5claimsleo`) provides the zero-knowledge circuits for:

- **Certificate Issuance**: Create private language learning certificate records
- **Language Proficiency Proofs**: Prove CEFR level without revealing exact scores
- **Performance Threshold Proofs**: Prove achievement above thresholds privately
- **Combined Proofs**: Verify multiple criteria in a single proof

```bash
# Build Leo program
cd web5claimsleo
leo build

# Test Leo transitions
leo test

# Deploy to Aleo testnet (requires testnet credits)
leo deploy --network testnet

## Deploy with Broadcasting
Run the deployment command with the `--broadcast` flag:

```bash
leo deploy --network testnet --broadcast
```

# Or use justfile commands
just leo-build
just leo-test
just leo-deploy
```

### Leo Program Structure

```
web5claimsleo/
├── src/
│   └── main.leo          # Main Leo program with ZK circuits
├── tests/
│   └── test_web5claimsleo.leo  # Leo tests
├── program.json          # Leo program configuration
└── justfile             # Leo-specific build commands
```

### Aleo Integration

The Leo program provides these key transitions:

1. **`issue_certificate`** - Creates a private LanguageCertificate record
2. **`prove_language_proficiency`** - Generates ZK proof of language level
3. **`prove_performance_threshold`** - Proves score above threshold
4. **`prove_combined_requirements`** - Combined language + performance proof

```leo
// Example: Issue a German B2 certificate
leo run issue_certificate \
    aleo1student... \     // recipient address
    12345field \          // language hash (German)
    4u8 \                // level (B2 = 4)
    94u8 \               // score (94%)
    50u16 \              // total challenges
    47u16 \              // solved challenges
    1703664000u32        // timestamp
```

## 🚀 Deployment

### Local Development

```bash
# Start the development server
just serve

# Or manually
cd web5claims-ui
trunk serve --public-url /
```

### GitHub Pages Deployment

The project is configured for automatic GitHub Pages deployment:

```bash
# Build for GitHub Pages
just build-github-pages

# Check deployment readiness
just check-deploy

# The GitHub Action will automatically deploy on push to main
```

### Aleo Testnet Deployment

```bash
# Prerequisites: Get testnet credits from Aleo faucet
# https://faucet.aleo.org/

# Deploy the Leo program
cd web5claimsleo
leo deploy --network testnet3

# Verify deployment
leo verify --network testnet3
```

### Environment Setup for Aleo

```bash
# Set up Aleo account (generates in .env)
cd web5claimsleo
leo account new

# Or import existing account
echo "NETWORK=testnet3" > .env
echo "PRIVATE_KEY=your_private_key_here" >> .env
```

## 🧪 Testing the Complete Flow

### End-to-End Testing

```bash
# 1. Test Rust components
cargo test

# 2. Test Leo circuits
cd web5claimsleo
leo test

# 3. Test UI components
cd ../web5claims-ui
cargo test

# 4. Run integration tests
just ci-simulation
```

### Manual Testing Flow

1. **Create Certificate**: Use the UI to generate a language learning certificate
2. **Generate ZK Proof**: Create zero-knowledge proofs for different claims
3. **Verify Proof**: Verify proofs without revealing private data
4. **Share Verification**: Generate shareable verification links

```bash
# Simulate the complete flow
just test-build
just preview-prod
```

## 📊 Monitoring & Verification

### Leo Program Verification

```bash
# Check program status on Aleo
leo query --network testnet3 web5claimsleo.aleo

# Verify a specific transition
leo verify transition_id --network testnet3
```

### UI Development

```bash
# Hot reload development
just serve

# Production preview
just preview-prod

# Check for issues
just troubleshoot
```

## 🔧 Advanced Configuration

### Leo Program Customization

Edit `web5claimsleo/src/main.leo` to customize:

- Certificate fields and validation
- ZK proof logic
- Performance thresholds
- Additional claim types

### Aleo Network Configuration

```bash
# Switch between networks
leo clean && leo build --network testnet3
leo clean && leo build --network mainnet
```

### Development Utilities

```bash
# Complete development workflow
just dev

# Complete release workflow
just release

# Quick development cycle
just quick
```
```

You should also update the "Quick Start" section:

```md
## 🎯 Quick Start

```bash
# 1. Install dependencies
just install

# 2. Build everything (Rust + Leo + UI)
just build-all

# 3. Test the complete system
just test

# 4. Start development server
just serve

# 5. Deploy Leo program to Aleo (optional)
cd web5claimsleo
leo deploy --network testnet3
```

**For Leo/Aleo development specifically:**

```bash
# Leo-only workflow
cd web5claimsleo
just leo-build    # Build Leo program
just leo-test     # Test Leo circuits
just leo-deploy   # Deploy to Aleo testnet
```
```

And add this troubleshooting section:

```md
## 🔧 Troubleshooting

### Leo/Aleo Issues

```bash
# Leo installation issues
curl -L https://raw.githubusercontent.com/AleoHQ/leo/main/install.sh | sh
leo --version

# Build issues
cd web5claimsleo
leo clean
leo build

# Testnet deployment issues
leo account new  # Generate new account
# Get testnet credits from https://faucet.aleo.org/

# Network connectivity
leo query --network testnet3 credits.aleo
```

### Common Issues

- **Leo not found**: Ensure Leo is in your PATH after installation
- **Deployment fails**: Check you have testnet credits and valid account
- **Build errors**: Run `leo clean` and rebuild
- **Network issues**: Verify testnet3 connectivity

```bash
# Complete troubleshooting
just troubleshoot
```
