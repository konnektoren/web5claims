# Web5 Claims - ZK Language Learning Certificate Verifier 🛂

**🏆 ZK Hack Berlin 2025 Submission**
*Privacy-Preserving Educational Credentials with Identity Verification*

---

## 🎯 **ZK Hack Berlin Challenge**

**Event**: ZK Hack Berlin 2025 (June 20-22, 2025)
**Location**: Berlin, Germany
**Our Focus**: Innovative ZK applications in education with privacy-preserving identity verification

---

## 🚀 **Live Demo**

- **🌐 Main Application**: [web5.claims/](https://web5.claims/)
- **🛂 ZKPassport Identity Verification**: [web5.claims/zkpass/](https://web5.claims/zkpass/)

---

## 🎯 **The Problem We Solve**

> *"The users need to verify their identity to get a certificate on the learning platform. It prevents the falsification of the certificates and also it gives more value to the certificates. They are more trustable and become widely acceptable, such as at work or university, schools."*

### 🔍 **Core Issues in Educational Verification**:
- **Certificate Fraud**: Easy falsification of educational credentials
- **Privacy Invasion**: Traditional verification exposes too much personal information
- **Trust Deficit**: Employers and institutions lack confidence in unverified claims
- **Centralized Systems**: Reliance on specific authorities creates bottlenecks

---

## 💡 **Our ZK Solution**

Web5 Claims provides **privacy-preserving educational credential verification** using cutting-edge zero-knowledge technology:

### ✅ **What We Implemented**
1. **🛂 ZKPassport Identity Verification**: Complete integration with age (18+) and name verification
2. **🎓 Enhanced Certificate Issuance**: Trust-scored certificates with optional identity verification
3. **🔐 Zero-Knowledge Proofs**: Prove competency without revealing private details
4. **🌐 Consumer-Friendly Interface**: Intuitive Yew/Rust WASM frontend

### ❌ **What We Didn't Complete**
- **⛓️ Blockchain Deployment**: ZK circuits designed but not deployed to Aleo/Arbitrum

---

## 🛠 **Technical Implementation**

### **🔧 Quick Start**
```bash
# 1. Install dependencies
just install

# 2. Start main application
just serve
# Opens at: http://localhost:8080/

# 3. Start ZKPass verification (separate terminal)
just serve-zkpass
# Opens at: http://localhost:8000/
```

### **🏗️ Architecture**
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   ZKPassport    │    │   Web5 Claims   │    │   Enhanced      │
│   Identity      │───▶│   Certificate   │───▶│   Verification  │
│   Verification  │    │   Issuer        │    │   (Trust Score) │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### **💻 Tech Stack**
- **Frontend**: Yew (Rust WebAssembly) + Vite (ZKPassport integration)
- **Backend**: Rust with ZK proof verification
- **Identity**: ZKPassport SDK with QR code generation
- **Deployment**: GitHub Pages with automated CI/CD

---

## 🎓 **Real-World Impact**

### **📊 Trust Score Innovation**
- **Standard Certificates**: 75/100 trust score
- **Identity-Verified Certificates**: 95/100 trust score
- **Result**: Higher employer confidence, better opportunities for learners

### **🔐 Privacy Benefits**
```
✅ PROVEN (Public):                    ❌ HIDDEN (Private):
• German B2+ proficiency              • Exact CEFR level (B2/C1/C2)
• Age 18+ verified                     • Exact age (25)
• First name verified                  • Surname/last name
• Certificate authenticity             • Exact performance scores
• High trust score (95/100)           • Study hours
```

### **💼 Use Cases**
1. **Employment**: Verify skills for job applications with enhanced trust
2. **University Admissions**: Prove language proficiency for international programs
3. **Security Clearance**: High-trust positions requiring identity verification

---

## 🚧 **Challenges We Overcame**

> *"We didn't know too much about the zkProof. And we learn quite a lot, thanks to the amazing support of the teams. I am glad, that we implemented it."*

### **🎓 Learning Journey**
- **ZK Fundamentals**: Deep dive into zero-knowledge proof concepts
- **ZKPassport Integration**: Mastering SDK integration and QR code flows
- **Consumer UX**: Making complex cryptography user-friendly
- **Privacy Design**: Balancing verification needs with privacy protection

### **💪 Technical Breakthroughs**
- Successfully integrated ZKPassport with seamless redirect flow
- Implemented trust scoring system based on verification completeness
- Created consumer-friendly ZK experience hiding complexity
- Built complete end-to-end verification system

---

## 🌟 **Innovation Highlights**

1. **🥇 First ZK Education + Identity Platform**: Novel combination of educational credentials with ZK identity verification
2. **🎯 Granular Privacy**: Prove specific skills without revealing full educational history
3. **📈 Trust Enhancement**: Quantified credibility through verification completeness
4. **👥 Consumer-Focused**: Designed for everyday users, not crypto natives
5. **🔒 Privacy by Design**: Zero personal data storage beyond verification results

---

## 📊 **Project Status**

### **✅ Completed Features**
- [x] Complete ZKPassport integration with age and name verification
- [x] Intuitive Yew/Rust WASM frontend
- [x] Seamless redirect flow between applications
- [x] Privacy-preserving verification system
- [x] GitHub Pages deployment pipeline

### **🔄 Future Implementation**
- [ ] Aleo ZK circuit deployment
- [ ] Arbitrum Stylus smart contracts
- [ ] On-chain proof verification
- [ ] Multi-language platform integration

---

## 🤝 **Team**

Building at ZK Hack Berlin with passion for privacy-preserving education technology.

**Key Learnings**: "We learned quite a lot about ZK proofs thanks to the amazing support of the teams. We're glad we implemented it!"

---

## 📞 **Links & Resources**

- **🌐 Live Demo**: [web5.claims/](https://web5.claims/)
- **🛂 ZKPass Demo**: [web5.claims/zkpass/](https://web5.claims/zkpass/)
- **📁 Source Code**: [GitHub Repository](https://github.com/konnektoren/web5claims)

---

## 🚀 **Quick Demo Flow**

1. **Visit** [web5.claims/zkpass/](https://web5.claims/zkpass/)
2. **Enter** your first name (e.g., "John")
3. **Scan** QR code with ZKPassport app
4. **Verify** age (18+) and name with zero-knowledge proofs
5. **Redirect** to certificate issuer with verified identity
6. **Generate** enhanced certificate with 95/100 trust score
7. **Experience** privacy-preserving verification magic! ✨

---

**🏆 Built with ❤️ at ZK Hack Berlin 2025**

*Where privacy meets trust, and zero-knowledge makes education verification magical.*
