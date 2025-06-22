import { ZKPassport } from "@zkpassport/sdk";
import QRCode from "qrcode";

// UI Elements
const button = document.getElementById("verifyBtn");
const canvas = document.getElementById("qrcodeCanvas");
const qrcodeContainer = document.getElementById("qrcode-container");
const status = document.getElementById("status");

// State management
let isVerifying = false;

// Initialize ZKPassport SDK
const initializeZKPassport = () => {
  try {
    return new ZKPassport();
  } catch (error) {
    console.error("Failed to initialize ZKPassport SDK:", error);
    showStatus(
      "❌ ZKPassport SDK not available. Please check your connection.",
      "error",
    );
    return null;
  }
};

// Update UI status
const showStatus = (message, type = "info") => {
  status.innerHTML = message;
  status.className = `status ${type}`;
};

// Show/hide QR code container
const toggleQRCode = (show) => {
  qrcodeContainer.classList.toggle("hidden", !show);
};

// Update button state
const updateButton = (text, disabled = false) => {
  button.textContent = text;
  button.disabled = disabled;
  button.classList.toggle("loading", disabled);
};

// Clear previous state
const resetUI = () => {
  toggleQRCode(false);
  showStatus("");
  updateButton("🔐 Verify You Are 18+", false);

  // Clear canvas
  const ctx = canvas.getContext("2d");
  ctx.clearRect(0, 0, canvas.width, canvas.height);
};

// Main verification function
const verifyAge = async () => {
  if (isVerifying) return;

  isVerifying = true;
  resetUI();
  updateButton("🛠 Generating Request...", true);
  showStatus("🛠 Initializing verification request...", "info");

  const zkPassport = initializeZKPassport();
  if (!zkPassport) {
    isVerifying = false;
    updateButton("🔐 Verify You Are 18+", false);
    return;
  }

  try {
    showStatus("🔄 Creating verification request...", "info");

    const queryBuilder = await zkPassport.request({
      name: "Web5 Claims Age Verifier",
      logo: "https://zkpassport.id/logo.png",
      purpose: "Verify age for enhanced language certificate credibility",
      scope: "age-verification",
      devMode: true, // Enable for development/testing
    });

    const {
      url,
      onRequestReceived,
      onGeneratingProof,
      onProofGenerated,
      onResult,
      onReject,
      onError,
    } = queryBuilder.gte("age", 18).done();

    // Generate and display QR code
    showStatus("📱 Generating QR code...", "info");
    await QRCode.toCanvas(canvas, url, {
      width: 300,
      margin: 2,
      color: {
        dark: "#000000",
        light: "#FFFFFF",
      },
    });

    toggleQRCode(true);
    updateButton("⏳ Waiting for scan...", true);
    showStatus("✅ Scan the QR code with the ZKPassport app", "success");

    // Set up event handlers
    onRequestReceived(() => {
      showStatus("📲 Request received on your device", "info");
      updateButton("📱 Processing on device...", true);
    });

    onGeneratingProof(() => {
      showStatus(
        "🔄 Generating zero-knowledge proof on your device...",
        "info",
      );
      updateButton("🔐 Generating proof...", true);
    });

    onProofGenerated(({ name }) => {
      console.log("ZK Proof generated:", name);
      showStatus("✅ Zero-knowledge proof generated successfully", "success");
      updateButton("⏳ Verifying proof...", true);
    });

    onResult(({ verified, result }) => {
      isVerifying = false;

      if (verified && result.age && result.age.gte && result.age.gte.result) {
        showStatus(
          "🎉 Age verification successful! You are verified as 18+ without revealing personal information.",
          "success",
        );
        updateButton("✅ Verified 18+", false);

        // Optional: Store verification result or redirect
        setTimeout(() => {
          if (
            confirm(
              "Verification successful! Would you like to return to Web5 Claims?",
            )
          ) {
            window.location.href = "../";
          }
        }, 3000);
      } else {
        showStatus(
          "❌ Age verification failed. You must be 18+ to proceed.",
          "error",
        );
        updateButton("❌ Verification Failed", false);

        setTimeout(() => {
          resetUI();
        }, 3000);
      }
    });

    onReject(() => {
      isVerifying = false;
      showStatus(
        "❌ Verification was rejected or cancelled by user.",
        "warning",
      );
      updateButton("🔐 Verify You Are 18+", false);
      toggleQRCode(false);
    });

    onError((error) => {
      isVerifying = false;
      console.error("ZKPassport verification error:", error);
      showStatus(
        "⚠️ An error occurred during verification. Please try again or check console for details.",
        "error",
      );
      updateButton("🔐 Verify You Are 18+", false);
      toggleQRCode(false);
    });
  } catch (error) {
    isVerifying = false;
    console.error("Failed to create verification request:", error);
    showStatus(
      "❌ Failed to generate verification request. Please check your connection and try again.",
      "error",
    );
    updateButton("🔐 Verify You Are 18+", false);
  }
};

// Event listeners
button.addEventListener("click", verifyAge);

// Handle page visibility changes (pause verification if tab is hidden)
document.addEventListener("visibilitychange", () => {
  if (document.hidden && isVerifying) {
    showStatus(
      "⏸️ Verification paused. Return to this tab to continue.",
      "warning",
    );
  }
});

// Keyboard accessibility
button.addEventListener("keydown", (e) => {
  if (e.key === "Enter" || e.key === " ") {
    e.preventDefault();
    verifyAge();
  }
});

// Log initialization
console.log("ZKPassport Age Verification initialized");
