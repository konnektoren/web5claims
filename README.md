# Web5 Claims - ZK Language Learning Certificate Verifier

A zero-knowledge verifiable credential system for language learning achievements, built for ZK Hack Berlin.

## 🚀 Quick Start

### Prerequisites
- Rust (latest stable)
- Node.js 18+
- Python 3 (for local server)

### Development Setup

```bash
# 1. Install required tools
just install

# 2. Check project status
just status

# 3. Start development server (main UI)
just serve
# Opens at: http://localhost:8080/

# 4. Start ZKPass verification (in another terminal)
just serve-zkpass
# Opens at: http://localhost:8000/
```

### Available Commands

```bash
# 🏗️  BUILD COMMANDS
just build-all          # Build everything (main + UI + ZKPass)
just build              # Build main Rust project
just build-ui           # Build UI for development
just build-zkpass       # Build ZKPass for development

# 🚀 SERVE COMMANDS
just serve              # Start main UI development server
just serve-zkpass       # Start ZKPass development server
just preview-prod       # Preview production build locally

# 🧪 TESTING
just test               # Run all tests
just test-build         # Test complete build process
just check              # Run quality checks (format + clippy + test)

# 🌐 DEPLOYMENT
just build-github-pages # Build for GitHub Pages deployment
just check-deploy       # Verify deployment readiness

# 📊 INFO & HELP
just status             # Show project status
just help               # Show detailed help
just troubleshoot       # Debug common issues
```

### Project Structure

```
web5claims/
├── src/                    # Main Rust library
├── web5claims-ui/          # Yew frontend application
├── zkpass/                 # ZKPass Vite integration
├── web5claimsleo/          # Leo zero-knowledge circuits
└── justfile               # Build automation
```

## 🎯 Project Overview

Web5 Claims is a privacy-preserving certificate verification system designed for language learning applications. Our system allows learners to prove their language proficiency without revealing personal information, using zero-knowledge proofs to maintain privacy while ensuring credential authenticity.

## 🏆 Hackathon Tracks

This project targets multiple ZK Hack Berlin prize tracks:

- **🥇 ZK Hack Berlin Winners** - Main competition
- **🔐 ZKPassport - Private Identity Verification** - Privacy-focused credential verification
- **⚡ Arbitrum - Build a ZK Verifier in Rust for Stylus** - Rust-based ZK verification
- **👥 Xion - Consumer ZK Apps** - User-friendly ZK applications

## 💡 The Problem

Traditional language learning certificates reveal too much personal information:
- Full identity exposure when proving qualifications
- Centralized verification systems prone to fraud
- No granular privacy controls (prove specific skills without revealing entire transcript)
- Employers and institutions require trust in issuing authorities

## 🚀 Our Solution

Web5 Claims provides:

1. **Zero-Knowledge Verification**: Prove language proficiency without revealing identity
2. **Granular Claims**: Verify specific skills (e.g., "speaks Spanish at B2 level") without full transcript
3. **Decentralized Trust**: Blockchain-based verification removes need for centralized authorities
4. **Privacy by Design**: Learners control what information to share and with whom
5. **Multi-Layer Identity**: Integration with ZKPassport for enhanced identity verification

## 🛠 Technical Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Language      │    │   Web5 Claims   │    │   Verifier      │
│  Learning App   │───▶│   ZK System     │───▶│   Application   │
│                 │    │                 │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
        │                        │                        │
        │                        │                        │
        ▼                        ▼                        ▼
  Issues VCs            Generates ZK Proofs        Verifies Claims
```

### Core Components

- **🦀 Rust Core**: High-performance zero-knowledge proof verification
- **🎨 Yew Frontend**: WebAssembly-based reactive UI
- **🛂 ZKPassport Integration**: Advanced identity verification with passport scanning
- **⚡ Leo Circuits**: Aleo-based zero-knowledge programs
- **🌐 GitHub Pages Deployment**: Decentralized hosting

### Technology Stack

- **Backend**: Rust with zero-knowledge proof libraries
- **Frontend**: Yew (Rust WebAssembly framework)
- **ZK Proofs**: Aleo Leo language for privacy-preserving computations
- **Identity**: ZKPassport SDK for passport-based verification
- **Deployment**: GitHub Pages with automated CI/CD

## 🎓 Use Cases

### For Learners
- Prove language skills for job applications without revealing full educational history
- Demonstrate specific competencies (listening, speaking, writing) independently
- Maintain privacy while building verifiable skill portfolio
- Use passport-based identity verification for enhanced credibility

### For Employers
- Verify candidate qualifications without accessing personal data
- Trust in skill verification without relying on specific institutions
- Streamlined verification process with cryptographic guarantees

### For Educational Institutions
- Issue tamper-proof digital certificates
- Reduce administrative overhead for verification requests
- Enable students to selectively share achievements

## 🧪 Development Workflow

### Local Development
```bash
# Full development setup
just dev

# Quick iteration (UI only)
just quick

# Test everything
just ci-simulation
```

### Production Testing
```bash
# Build for production
just build-github-pages

# Test production build locally
just preview-prod
# Visit: http://localhost:8000/web5claims/
# ZKPass: http://localhost:8000/web5claims/zkpass/
```

### Troubleshooting
```bash
# Check environment
just troubleshoot

# Clean rebuild
just clean && just install && just build-all

# Detailed project info
just deployment-info
```

## 🏗 Implementation Roadmap

### ✅ Phase 1: Core ZK System (Hackathon)
- [x] Basic verifiable credential structure
- [x] Yew-based frontend application
- [x] ZKPassport integration for identity verification
- [x] Rust-based proof verification
- [x] GitHub Pages deployment pipeline

### 🔄 Phase 2: Integration (In Progress)
- [ ] Leo zero-knowledge circuits implementation
- [ ] API integration with language learning platforms
- [ ] Advanced proof generation and verification
- [ ] Mobile-responsive design improvements

### 🚀 Phase 3: Advanced Features
- [ ] Cross-platform credential portability
- [ ] Advanced privacy controls and selective disclosure
- [ ] Reputation and trust scoring systems
- [ ] Integration with major language learning platforms

## 🌟 Innovation Highlights

1. **Educational ZK Applications**: Novel use of zero-knowledge proofs in education sector
2. **Granular Privacy**: Selective disclosure of specific skills vs. all-or-nothing approaches
3. **Consumer-Friendly**: Designed for everyday users, not just crypto natives
4. **Rust Performance**: High-performance verification suitable for production use
5. **Multi-Modal Identity**: Combines traditional and passport-based verification
6. **Full-Stack ZK**: End-to-end zero-knowledge architecture from frontend to backend

## 🔗 Live Demo

- **Main Application**: [web5.claims/](https://web5.claims/)
- **ZKPassport Verification**: [web5.claims/zkpass](https://web5.claims/zkpass/)

## 🏆 Prize Track Alignment

- **ZK Hack Berlin**: Innovative application of ZK technology in education with full-stack implementation
- **ZKPassport**: Advanced privacy-preserving identity verification for educational credentials using passport scanning
- **Arbitrum**: Production-ready Rust ZK verifier optimized for Stylus integration
- **Xion**: Consumer-focused ZK application with intuitive UI and real-world utility

## 🤝 Team

Building at ZK Hack Berlin with passion for privacy-preserving education technology.

## 📄 License

MIT License - see LICENSE file for details.

---

*Built with ❤️ at ZK Hack Berlin 2025*
