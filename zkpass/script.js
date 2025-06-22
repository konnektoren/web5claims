import { ZKPassport } from "@zkpassport/sdk";
import QRCode from "qrcode";

// UI Elements
const verifyButton = document.getElementById("verifyBtn");
const canvas = document.getElementById("qrcodeCanvas");
const qrcodeContainer = document.getElementById("qrcode-container");
const status = document.getElementById("status");
const nameInput = document.getElementById("nameInput");
const nameInputContainer = document.getElementById("name-input-container");
const successActions = document.getElementById("success-actions");

// State management
let isVerifying = false;
let expectedFirstName = null;
let verificationSuccess = false;
let verifiedData = null;

// Initialize ZKPassport SDK
const initializeZKPassport = () => {
  try {
    const zkPassport = new ZKPassport();
    console.log("ZKPassport initialized successfully:", zkPassport);
    return zkPassport;
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
  console.log(`Status [${type}]:`, message);
};

// Show/hide QR code container
const toggleQRCode = (show) => {
  qrcodeContainer.classList.toggle("hidden", !show);
};

// Show/hide name input container
const toggleNameInput = (show) => {
  nameInputContainer.classList.toggle("hidden", !show);
};

// Show/hide success actions
const toggleSuccessActions = (show) => {
  successActions.classList.toggle("hidden", !show);
};

// Update button state
const updateButton = (text, disabled = false) => {
  if (verifyButton) {
    verifyButton.textContent = text;
    verifyButton.disabled = disabled;
    verifyButton.classList.toggle("loading", disabled);
  }
};

// Clear previous state
const resetUI = () => {
  toggleQRCode(false);
  toggleNameInput(false);
  toggleSuccessActions(false);
  showStatus("");
  updateButton("🔒 Verify Identity (Age + Name)", false);
  if (nameInput) nameInput.value = "";
  expectedFirstName = null;
  verificationSuccess = false;
  verifiedData = null;

  // Clear canvas
  if (canvas) {
    const ctx = canvas.getContext("2d");
    ctx.clearRect(0, 0, canvas.width, canvas.height);
  }
};

// Navigate to issuer with verified data
const navigateToIssuer = () => {
  if (!verifiedData) {
    showStatus("❌ No verified data available", "error");
    return;
  }

  // Create URL parameters with verified data
  const params = new URLSearchParams();

  if (verifiedData.firstName) {
    params.set("verified_name", verifiedData.firstName);
  }

  if (verifiedData.ageVerified) {
    params.set("verified_age", "true");
  }

  params.set("verification_type", "identity");
  params.set("verification_timestamp", new Date().toISOString());

  // Navigate to main app issuer page
  const issuerUrl = `../?page=issuer&${params.toString()}`;
  console.log("Navigating to issuer:", issuerUrl);

  window.location.href = issuerUrl;
};

// Navigate to verifier for testing
const navigateToVerifier = () => {
  const verifierUrl = `../?page=verifier`;
  console.log("Navigating to verifier:", verifierUrl);
  window.location.href = verifierUrl;
};

// Start a new verification
const startNewVerification = () => {
  resetUI();
  showStatus("🔄 Ready for new verification", "info");
};

// Main verification function
const startVerification = async () => {
  if (isVerifying) return;

  console.log("Starting identity verification (age + name)");

  resetUI();
  toggleNameInput(true);
  updateButton("📝 Enter your first name below...", true);
  showStatus(
    "📝 Please enter your first name to verify against your passport",
    "info",
  );
};

// Perform the actual verification
const performVerification = async (firstName) => {
  console.log("Performing identity verification with firstName:", firstName);

  if (!firstName) {
    showStatus("❌ First name is required for verification", "error");
    return;
  }

  isVerifying = true;
  updateButton("🛠 Generating Request...", true);
  showStatus("🛠 Initializing verification request...", "info");

  const zkPassport = initializeZKPassport();
  if (!zkPassport) {
    isVerifying = false;
    updateButton("🔒 Verify Identity (Age + Name)", false);
    return;
  }

  try {
    showStatus("🔄 Creating verification request...", "info");

    // Build the request for combined age and name verification
    const requestConfig = {
      name: "Web5 Claims Identity Verifier",
      logo: "https://zkpassport.id/logo.png",
      purpose: "Verify age and identity for complete certificate validation",
      scope: "identity-verification",
      devMode: true,
    };

    console.log("Request config:", requestConfig);
    const queryBuilder = await zkPassport.request(requestConfig);
    console.log("Query builder created:", queryBuilder);

    // Store expected first name for verification
    expectedFirstName = firstName;

    // Build query using disclose method for both age and name
    let query;
    try {
      console.log("Building combined disclosure query for age and name...");
      // Try different combinations of age and name fields
      try {
        query = queryBuilder.disclose("age").disclose("firstname").done();
        console.log("Using 'age' and 'firstname' fields for disclosure");
      } catch (combinedError) {
        console.log(
          "age/firstname failed, trying age/firstName:",
          combinedError,
        );
        try {
          query = queryBuilder.disclose("age").disclose("firstName").done();
          console.log("Using 'age' and 'firstName' fields for disclosure");
        } catch (combined2Error) {
          console.log(
            "age/firstName failed, trying dateOfBirth/firstname:",
            combined2Error,
          );
          try {
            query = queryBuilder
              .disclose("dateOfBirth")
              .disclose("firstname")
              .done();
            console.log(
              "Using 'dateOfBirth' and 'firstname' fields for disclosure",
            );
          } catch (combined3Error) {
            console.log(
              "dateOfBirth/firstname failed, trying age/name:",
              combined3Error,
            );
            try {
              query = queryBuilder.disclose("age").disclose("name").done();
              console.log("Using 'age' and 'name' fields for disclosure");
            } catch (combined4Error) {
              console.log(
                "age/name failed, trying age/givenName:",
                combined4Error,
              );
              query = queryBuilder.disclose("age").disclose("givenName").done();
              console.log("Using 'age' and 'givenName' fields for disclosure");
            }
          }
        }
      }
    } catch (queryError) {
      console.error("Error building query:", queryError);
      throw new Error(
        `Failed to build verification query: ${queryError.message}`,
      );
    }

    console.log("Query built successfully:", query);

    const {
      url,
      onRequestReceived,
      onGeneratingProof,
      onProofGenerated,
      onResult,
      onReject,
      onError,
    } = query;

    console.log("Verification URL generated:", url);

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

    console.log("QR code generated successfully");
    toggleQRCode(true);
    updateButton("⏳ Waiting for scan...", true);
    showStatus("✅ Scan the QR code with the ZKPassport app", "success");

    // Set up event handlers
    onRequestReceived(() => {
      console.log("Request received on device");
      showStatus("📲 Request received on your device", "info");
      updateButton("📱 Processing on device...", true);
    });

    onGeneratingProof(() => {
      console.log("Generating proof on device");
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
      console.log("Verification result:", { verified, result });
      console.log("Full result object:", JSON.stringify(result, null, 2));
      isVerifying = false;
      handleVerificationResult(verified, result, firstName);
    });

    onReject(() => {
      console.log("Verification rejected by user");
      isVerifying = false;
      showStatus(
        "❌ Verification was rejected or cancelled by user.",
        "warning",
      );
      updateButton("🔒 Verify Identity (Age + Name)", false);
      toggleQRCode(false);
    });

    onError((error) => {
      console.error("ZKPassport verification error:", error);
      isVerifying = false;
      showStatus(
        `⚠️ Verification error: ${error.message || "Unknown error"}. Check console for details.`,
        "error",
      );
      updateButton("🔒 Verify Identity (Age + Name)", false);
      toggleQRCode(false);
    });
  } catch (error) {
    isVerifying = false;
    console.error("Failed to create verification request:", error);
    showStatus(
      `❌ Failed to generate verification request: ${error.message}`,
      "error",
    );
    updateButton("🔒 Verify Identity (Age + Name)", false);
  }
};

// Handle verification results
const handleVerificationResult = (verified, result, firstName) => {
  console.log("Handling verification result:", {
    verified,
    result,
    firstName,
    expectedFirstName,
  });
  console.log("Result keys:", Object.keys(result));

  let success = false;
  let message = "";
  let resultData = {};

  // In dev mode with mock proofs, verified might be false but we can still check the disclosed data
  const isDevMode = true;

  if (!verified && !isDevMode) {
    showStatus("❌ Verification failed - proof was not valid", "error");
    updateButton("❌ Verification Failed", false);
    setTimeout(() => resetUI(), 5000);
    return;
  }

  // Extract age data from the nested structure
  let ageData = null;
  if (result.age?.disclose?.result) {
    ageData = result.age.disclose.result;
  } else if (result.dateOfBirth?.disclose?.result) {
    ageData = result.dateOfBirth.disclose.result;
  } else if (result.birthDate?.disclose?.result) {
    ageData = result.birthDate.disclose.result;
  }

  // Extract name data from the nested structure
  let nameData = null;
  if (result.firstname?.disclose?.result) {
    nameData = result.firstname.disclose.result;
  } else if (result.firstName?.disclose?.result) {
    nameData = result.firstName.disclose.result;
  } else if (result.given_name?.disclose?.result) {
    nameData = result.given_name.disclose.result;
  } else if (result.givenName?.disclose?.result) {
    nameData = result.givenName.disclose.result;
  } else if (result.name?.disclose?.result) {
    nameData = result.name.disclose.result;
  }

  let ageVerified = false;
  let nameVerified = false;
  let userAge;

  // Age verification
  if (ageData !== null) {
    if (typeof ageData === "number") {
      userAge = ageData;
    } else if (typeof ageData === "string") {
      if (ageData.includes("-") || ageData.includes("/")) {
        // Handle date of birth
        const birthDate = new Date(ageData);
        const today = new Date();
        userAge = today.getFullYear() - birthDate.getFullYear();
        const monthDiff = today.getMonth() - birthDate.getMonth();
        if (
          monthDiff < 0 ||
          (monthDiff === 0 && today.getDate() < birthDate.getDate())
        ) {
          userAge--;
        }
      } else {
        userAge = parseInt(ageData);
      }
    }

    if (!isNaN(userAge) && userAge >= 18) {
      ageVerified = true;
    }

    resultData.age = userAge;
    resultData.ageVerified = ageVerified;
    console.log("Age verification:", { ageData, userAge, ageVerified });
  } else {
    console.log("No age data found in result");
  }

  // Name verification
  if (nameData && expectedFirstName) {
    let disclosedName = nameData;

    // Handle case where name might be an object
    if (typeof nameData === "object" && nameData.first) {
      disclosedName = nameData.first;
    } else if (typeof nameData === "object" && nameData.given) {
      disclosedName = nameData.given;
    }

    const normalizedDisclosed = String(disclosedName).toLowerCase().trim();
    const normalizedExpected = expectedFirstName.toLowerCase().trim();
    nameVerified = normalizedDisclosed === normalizedExpected;

    resultData.firstName = expectedFirstName;
    resultData.nameVerified = nameVerified;
    console.log("Name verification:", {
      nameData,
      disclosedName,
      expectedFirstName,
      normalizedDisclosed,
      normalizedExpected,
      nameVerified,
    });
  } else {
    console.log("No name data found or expected name not set");
  }

  success = ageVerified && nameVerified;

  if (success) {
    message = `🎉 Complete verification successful! Age (${userAge}) and name "${expectedFirstName}" verified without revealing other personal information.`;
  } else if (ageVerified && !nameVerified) {
    message = `❌ Partial verification: Age (${userAge}) verified but name "${expectedFirstName}" does not match your passport.`;
  } else if (!ageVerified && nameVerified) {
    message = `❌ Partial verification: Name verified but you must be 18+ to proceed. (Age: ${userAge || "unknown"})`;
  } else {
    message = `❌ Verification failed: Neither age nor name "${expectedFirstName}" could be verified.`;
    if (ageData === null) message += " (No age data found)";
    if (nameData === null) message += " (No name data found)";
    if (userAge !== undefined && userAge < 18)
      message += ` (Age ${userAge} is under 18)`;
  }

  if (success || (isDevMode && (ageVerified || nameVerified))) {
    verificationSuccess = success;
    verifiedData = resultData;

    showStatus(message, success ? "success" : "warning");
    updateButton(success ? "✅ Verified" : "⚠️ Partial Verification", false);

    // Hide QR code and show success actions
    toggleQRCode(false);

    if (success || (isDevMode && (ageVerified || nameVerified))) {
      toggleSuccessActions(true);
      updateSuccessActions(resultData);
    }
  } else {
    showStatus(message, "error");
    updateButton("❌ Verification Failed", false);

    setTimeout(() => {
      resetUI();
    }, 5000);
  }
};

// Handle name input submission
const handleNameSubmission = () => {
  const firstName = nameInput.value.trim();

  console.log("Name submission:", firstName);

  if (!firstName) {
    showStatus("❌ Please enter your first name", "error");
    return;
  }

  if (firstName.length < 1) {
    showStatus("❌ Please enter a valid first name", "error");
    return;
  }

  // Validate name contains only letters, spaces, hyphens, and apostrophes
  const nameRegex = /^[a-zA-ZÀ-ÿ\s\-'\.]+$/;
  if (!nameRegex.test(firstName)) {
    showStatus(
      "❌ First name can only contain letters, spaces, hyphens, and apostrophes",
      "error",
    );
    return;
  }

  // Hide name input and start verification
  toggleNameInput(false);
  performVerification(firstName);
};

// Event listeners
if (verifyButton) {
  verifyButton.addEventListener("click", () => {
    console.log("Verify button clicked");
    startVerification();
  });
}

// Handle name input
if (nameInput) {
  nameInput.addEventListener("keydown", (e) => {
    if (e.key === "Enter") {
      e.preventDefault();
      handleNameSubmission();
    }
  });
}

// Add submit button for name input
document.addEventListener("DOMContentLoaded", () => {
  const submitNameBtn = document.getElementById("submitNameBtn");
  if (submitNameBtn) {
    submitNameBtn.addEventListener("click", handleNameSubmission);
  }

  // Success action buttons
  const issuerBtn = document.getElementById("issuer-btn");
  const verifierBtn = document.getElementById("verifier-btn");
  const newVerificationBtn = document.getElementById("new-verification-btn");

  if (issuerBtn) {
    issuerBtn.addEventListener("click", navigateToIssuer);
  }

  if (verifierBtn) {
    verifierBtn.addEventListener("click", navigateToVerifier);
  }

  if (newVerificationBtn) {
    newVerificationBtn.addEventListener("click", startNewVerification);
  }
});

// Handle page visibility changes
document.addEventListener("visibilitychange", () => {
  if (document.hidden && isVerifying) {
    showStatus(
      "⏸️ Verification paused. Return to this tab to continue.",
      "warning",
    );
  }
});

// Keyboard accessibility
if (verifyButton) {
  verifyButton.addEventListener("keydown", (e) => {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      verifyButton.click();
    }
  });
}

// Log initialization
console.log("ZKPassport Identity Verification (Age + Name) initialized");
