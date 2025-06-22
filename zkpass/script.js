import { ZKPassport } from "@zkpassport/sdk";
import QRCode from "qrcode";

// UI Elements
const ageButton = document.getElementById("verifyAgeBtn");
const nameButton = document.getElementById("verifyNameBtn");
const bothButton = document.getElementById("verifyBothBtn");
const canvas = document.getElementById("qrcodeCanvas");
const qrcodeContainer = document.getElementById("qrcode-container");
const status = document.getElementById("status");
const nameInput = document.getElementById("nameInput");
const nameInputContainer = document.getElementById("name-input-container");

// State management
let isVerifying = false;
let currentVerificationType = null;
let expectedFirstName = null;

// Initialize ZKPassport SDK
const initializeZKPassport = () => {
  try {
    // You might need to replace this with your domain
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

// Update button state
const updateButton = (button, text, disabled = false) => {
  if (button) {
    button.textContent = text;
    button.disabled = disabled;
    button.classList.toggle("loading", disabled);
  }
};

// Update all buttons
const updateAllButtons = (text, disabled = false) => {
  updateButton(ageButton, disabled ? text : "🔐 Verify Age (18+)", disabled);
  updateButton(nameButton, disabled ? text : "👤 Verify First Name", disabled);
  updateButton(bothButton, disabled ? text : "🔒 Verify Age + Name", disabled);
};

// Clear previous state
const resetUI = () => {
  toggleQRCode(false);
  toggleNameInput(false);
  showStatus("");
  updateAllButtons("", false);
  if (nameInput) nameInput.value = "";
  currentVerificationType = null;
  expectedFirstName = null;

  // Clear canvas
  if (canvas) {
    const ctx = canvas.getContext("2d");
    ctx.clearRect(0, 0, canvas.width, canvas.height);
  }
};

// Main verification function
const startVerification = async (type) => {
  if (isVerifying) return;

  console.log("Starting verification type:", type);
  currentVerificationType = type;

  // For age verification, go directly to QR code generation
  if (type === "age") {
    await performVerification(type);
    return;
  }

  // For name-based verifications, show input first
  if (type === "name" || type === "both") {
    resetUI();
    currentVerificationType = type;
    toggleNameInput(true);
    updateAllButtons("📝 Enter your first name below...", true);
    showStatus(
      "📝 Please enter your first name to verify against your passport",
      "info",
    );
    return;
  }

  console.error("Unknown verification type:", type);
  showStatus("❌ Unknown verification type", "error");
};

// Perform the actual verification
const performVerification = async (type, firstName = null) => {
  console.log("Performing verification:", {
    type,
    firstName,
    currentVerificationType,
  });

  const verificationType = type || currentVerificationType;

  if (!verificationType) {
    console.error("No verification type available");
    showStatus("❌ Verification type not set", "error");
    return;
  }

  isVerifying = true;
  updateAllButtons("🛠 Generating Request...", true);
  showStatus("🛠 Initializing verification request...", "info");

  const zkPassport = initializeZKPassport();
  if (!zkPassport) {
    isVerifying = false;
    updateAllButtons("", false);
    return;
  }

  try {
    showStatus("🔄 Creating verification request...", "info");

    // Build the request based on verification type
    const requestConfig = {
      name: "Web5 Claims Identity Verifier",
      logo: "https://zkpassport.id/logo.png",
      purpose: getVerificationPurpose(verificationType),
      scope: getVerificationScope(verificationType),
      devMode: true,
    };

    console.log("Request config:", requestConfig);
    const queryBuilder = await zkPassport.request(requestConfig);
    console.log("Query builder created:", queryBuilder);

    // Store expected first name for verification
    if (firstName) {
      expectedFirstName = firstName;
    }

    // Build query using disclose method based on ZKPassport API
    let query;
    try {
      switch (verificationType) {
        case "age":
          console.log("Building age disclosure query...");
          // Disclose age-related fields that can prove 18+
          query = queryBuilder.disclose("age").done();
          break;

        case "name":
          console.log(
            "Building name disclosure query with firstName:",
            firstName,
          );
          // Try different possible field names for first name
          try {
            query = queryBuilder.disclose("firstname").done();
            console.log("Using 'firstname' field for disclosure");
          } catch (nameError) {
            console.log("firstname field failed, trying firstName:", nameError);
            try {
              query = queryBuilder.disclose("firstName").done();
              console.log("Using 'firstName' field for disclosure");
            } catch (firstNameError) {
              console.log(
                "firstName failed, trying given_name:",
                firstNameError,
              );
              try {
                query = queryBuilder.disclose("given_name").done();
                console.log("Using 'given_name' field for disclosure");
              } catch (given_nameError) {
                console.log(
                  "given_name failed, trying givenName:",
                  given_nameError,
                );
                query = queryBuilder.disclose("givenName").done();
                console.log("Using 'givenName' field for disclosure");
              }
            }
          }
          break;

        case "both":
          console.log("Building combined disclosure query...");
          // Disclose both age and name fields
          try {
            query = queryBuilder.disclose("age").disclose("firstname").done();
            console.log(
              "Using 'age' and 'firstname' fields for combined disclosure",
            );
          } catch (combinedError) {
            console.log(
              "age/firstname failed, trying age/firstName:",
              combinedError,
            );
            try {
              query = queryBuilder.disclose("age").disclose("firstName").done();
              console.log(
                "Using 'age' and 'firstName' fields for combined disclosure",
              );
            } catch (combined2Error) {
              console.log(
                "age/firstName failed, trying age/given_name:",
                combined2Error,
              );
              try {
                query = queryBuilder
                  .disclose("age")
                  .disclose("given_name")
                  .done();
                console.log(
                  "Using 'age' and 'given_name' fields for combined disclosure",
                );
              } catch (combined3Error) {
                console.log(
                  "age/given_name failed, trying age/givenName:",
                  combined3Error,
                );
                query = queryBuilder
                  .disclose("age")
                  .disclose("givenName")
                  .done();
                console.log(
                  "Using 'age' and 'givenName' fields for combined disclosure",
                );
              }
            }
          }
          break;

        default:
          throw new Error(`Invalid verification type: ${verificationType}`);
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
    updateAllButtons("⏳ Waiting for scan...", true);
    showStatus("✅ Scan the QR code with the ZKPassport app", "success");

    // Set up event handlers
    onRequestReceived(() => {
      console.log("Request received on device");
      showStatus("📲 Request received on your device", "info");
      updateAllButtons("📱 Processing on device...", true);
    });

    onGeneratingProof(() => {
      console.log("Generating proof on device");
      showStatus(
        "🔄 Generating zero-knowledge proof on your device...",
        "info",
      );
      updateAllButtons("🔐 Generating proof...", true);
    });

    onProofGenerated(({ name }) => {
      console.log("ZK Proof generated:", name);
      showStatus("✅ Zero-knowledge proof generated successfully", "success");
      updateAllButtons("⏳ Verifying proof...", true);
    });

    onResult(({ verified, result }) => {
      console.log("Verification result:", { verified, result });
      isVerifying = false;
      handleVerificationResult(verificationType, verified, result, firstName);
    });

    onReject(() => {
      console.log("Verification rejected by user");
      isVerifying = false;
      showStatus(
        "❌ Verification was rejected or cancelled by user.",
        "warning",
      );
      updateAllButtons("", false);
      toggleQRCode(false);
    });

    onError((error) => {
      console.error("ZKPassport verification error:", error);
      isVerifying = false;
      showStatus(
        `⚠️ Verification error: ${error.message || "Unknown error"}. Check console for details.`,
        "error",
      );
      updateAllButtons("", false);
      toggleQRCode(false);
    });
  } catch (error) {
    isVerifying = false;
    console.error("Failed to create verification request:", error);
    showStatus(
      `❌ Failed to generate verification request: ${error.message}`,
      "error",
    );
    updateAllButtons("", false);
  }
};

// Handle verification results
const handleVerificationResult = (type, verified, result, firstName) => {
  console.log("Handling verification result:", {
    type,
    verified,
    result,
    firstName,
    expectedFirstName,
  });

  let success = false;
  let message = "";

  if (!verified) {
    showStatus("❌ Verification failed - proof was not valid", "error");
    updateAllButtons("❌ Verification Failed", false);
    setTimeout(() => resetUI(), 5000);
    return;
  }

  switch (type) {
    case "age":
      // Check if the disclosed age proves 18+
      const age = result.age || result.dateOfBirth;
      if (age) {
        // Calculate age if we have date of birth
        let userAge;
        if (typeof age === "string" && age.includes("-")) {
          // Date of birth format
          const birthDate = new Date(age);
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
          userAge = parseInt(age);
        }

        success = userAge >= 18;
        message = success
          ? "🎉 Age verification successful! You are verified as 18+ without revealing personal information."
          : "❌ Age verification failed. You must be 18+ to proceed.";
      } else {
        message = "❌ Age information not found in verification result.";
      }
      break;

    case "name":
      // Check if disclosed name matches expected name
      const disclosedName =
        result.firstname ||
        result.firstName ||
        result.given_name ||
        result.givenName;
      if (disclosedName && expectedFirstName) {
        success =
          disclosedName.toLowerCase().trim() ===
          expectedFirstName.toLowerCase().trim();
        message = success
          ? `🎉 Name verification successful! Your first name "${expectedFirstName}" has been verified without revealing other personal information.`
          : `❌ Name verification failed. The name "${expectedFirstName}" does not match your passport (received: "${disclosedName}").`;
      } else {
        message =
          "❌ Name information not found in verification result or expected name not set.";
      }
      break;

    case "both":
      // Check both age and name
      const userAge2 = result.age || result.dateOfBirth;
      const disclosedName2 =
        result.firstname ||
        result.firstName ||
        result.given_name ||
        result.givenName;

      let ageVerified = false;
      let nameVerified = false;

      if (userAge2) {
        let calculatedAge;
        if (typeof userAge2 === "string" && userAge2.includes("-")) {
          const birthDate = new Date(userAge2);
          const today = new Date();
          calculatedAge = today.getFullYear() - birthDate.getFullYear();
          const monthDiff = today.getMonth() - birthDate.getMonth();
          if (
            monthDiff < 0 ||
            (monthDiff === 0 && today.getDate() < birthDate.getDate())
          ) {
            calculatedAge--;
          }
        } else {
          calculatedAge = parseInt(userAge2);
        }
        ageVerified = calculatedAge >= 18;
      }

      if (disclosedName2 && expectedFirstName) {
        nameVerified =
          disclosedName2.toLowerCase().trim() ===
          expectedFirstName.toLowerCase().trim();
      }

      success = ageVerified && nameVerified;

      if (success) {
        message = `🎉 Complete verification successful! Age (18+) and name "${expectedFirstName}" verified without revealing other personal information.`;
      } else if (ageVerified && !nameVerified) {
        message = `❌ Partial verification: Age verified but name "${expectedFirstName}" does not match your passport.`;
      } else if (!ageVerified && nameVerified) {
        message = `❌ Partial verification: Name verified but you must be 18+ to proceed.`;
      } else {
        message = `❌ Verification failed: Neither age nor name "${expectedFirstName}" could be verified.`;
      }
      break;
  }

  if (success) {
    showStatus(message, "success");
    updateAllButtons("✅ Verified", false);

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
    showStatus(message, "error");
    updateAllButtons("❌ Verification Failed", false);

    setTimeout(() => {
      resetUI();
    }, 5000);
  }
};

// Get verification purpose text
const getVerificationPurpose = (type) => {
  switch (type) {
    case "age":
      return "Verify age for enhanced language certificate credibility";
    case "name":
      return "Verify first name for personalized certificate validation";
    case "both":
      return "Verify age and identity for complete certificate validation";
    default:
      return "Identity verification for Web5 Claims";
  }
};

// Get verification scope
const getVerificationScope = (type) => {
  switch (type) {
    case "age":
      return "age-verification";
    case "name":
      return "name-verification";
    case "both":
      return "age-name-verification";
    default:
      return "identity-verification";
  }
};

// Handle name input submission
const handleNameSubmission = () => {
  const firstName = nameInput.value.trim();

  console.log(
    "Name submission:",
    firstName,
    "Current type:",
    currentVerificationType,
  );

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

  if (!currentVerificationType) {
    console.error("No current verification type set");
    showStatus("❌ Verification type not set. Please try again.", "error");
    resetUI();
    return;
  }

  // Hide name input and start verification
  toggleNameInput(false);
  performVerification(currentVerificationType, firstName);
};

// Event listeners
if (ageButton) {
  ageButton.addEventListener("click", () => {
    console.log("Age button clicked");
    startVerification("age");
  });
}

if (nameButton) {
  nameButton.addEventListener("click", () => {
    console.log("Name button clicked");
    startVerification("name");
  });
}

if (bothButton) {
  bothButton.addEventListener("click", () => {
    console.log("Both button clicked");
    startVerification("both");
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

// Keyboard accessibility for buttons
[ageButton, nameButton, bothButton].forEach((button) => {
  if (button) {
    button.addEventListener("keydown", (e) => {
      if (e.key === "Enter" || e.key === " ") {
        e.preventDefault();
        button.click();
      }
    });
  }
});

// Log initialization
console.log("ZKPassport Identity Verification initialized");
