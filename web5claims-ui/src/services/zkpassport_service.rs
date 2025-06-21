use js_sys::{Object, Promise, Reflect};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use yew::Callback;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkPassportProof {
    pub proof: String,
    pub public_signals: Vec<String>,
    pub verification_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportData {
    pub nationality: String,
    pub age_over_18: bool,
    pub expiry_date: String,
    pub document_type: String,
}

pub struct ZkPassportService {
    sdk: Option<JsValue>,
}

impl ZkPassportService {
    pub fn new() -> Self {
        let sdk = Self::get_zkpassport_sdk();
        Self { sdk }
    }

    fn get_zkpassport_sdk() -> Option<JsValue> {
        if let Some(window) = web_sys::window() {
            match Reflect::get(&window, &JsValue::from_str("ZKPassport")) {
                Ok(zkpassport) if !zkpassport.is_undefined() => {
                    log::info!("ZK Passport SDK found");
                    Some(zkpassport)
                }
                _ => {
                    log::warn!("ZK Passport SDK not found");
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn is_available(&self) -> bool {
        self.sdk.is_some()
    }

    pub async fn initialize(&self) -> Result<(), String> {
        let sdk = self.sdk.as_ref().ok_or("ZK Passport SDK not available")?;

        // Call initialize method
        if let Ok(init_fn) = Reflect::get(sdk, &JsValue::from_str("initialize")) {
            if init_fn.is_function() {
                let init_fn = init_fn.dyn_into::<js_sys::Function>().unwrap();
                match init_fn.call0(sdk) {
                    Ok(result) => {
                        if let Ok(promise) = result.dyn_into::<Promise>() {
                            JsFuture::from(promise)
                                .await
                                .map_err(|e| format!("Initialization failed: {:?}", e))?;
                        }
                        log::info!("ZK Passport SDK initialized");
                        Ok(())
                    }
                    Err(e) => Err(format!("Failed to call initialize: {:?}", e)),
                }
            } else {
                log::info!("No initialize method found, assuming SDK is ready");
                Ok(())
            }
        } else {
            log::info!("No initialize method found, assuming SDK is ready");
            Ok(())
        }
    }

    pub async fn scan_passport(&self) -> Result<PassportData, String> {
        let sdk = self.sdk.as_ref().ok_or("ZK Passport SDK not available")?;

        // Call scanPassport method
        let scan_fn = Reflect::get(sdk, &JsValue::from_str("scanPassport"))
            .map_err(|_| "scanPassport method not found")?;

        if !scan_fn.is_function() {
            return Err("scanPassport is not a function".to_string());
        }

        let scan_fn = scan_fn.dyn_into::<js_sys::Function>().unwrap();
        let result = scan_fn
            .call0(sdk)
            .map_err(|e| format!("Failed to call scanPassport: {:?}", e))?;

        // Clone result before using it
        if let Ok(promise) = result.clone().dyn_into::<Promise>() {
            let passport_data = JsFuture::from(promise)
                .await
                .map_err(|e| format!("Passport scan failed: {:?}", e))?;

            // Parse the passport data
            self.parse_passport_data(passport_data)
        } else {
            self.parse_passport_data(result)
        }
    }

    pub async fn generate_proof(
        &self,
        passport_data: &PassportData,
        claim_type: &str,
    ) -> Result<ZkPassportProof, String> {
        let sdk = self.sdk.as_ref().ok_or("ZK Passport SDK not available")?;

        // Call generateProof method
        let proof_fn = Reflect::get(sdk, &JsValue::from_str("generateProof"))
            .map_err(|_| "generateProof method not found")?;

        if !proof_fn.is_function() {
            return Err("generateProof is not a function".to_string());
        }

        let proof_fn = proof_fn.dyn_into::<js_sys::Function>().unwrap();

        // Create proof request using manual JSON serialization to avoid serde_wasm_bindgen
        let proof_request = js_sys::Object::new();
        let _ = Reflect::set(
            &proof_request,
            &JsValue::from_str("claimType"),
            &JsValue::from_str(claim_type),
        );

        // Create passport data object manually
        let passport_obj = js_sys::Object::new();
        let _ = Reflect::set(
            &passport_obj,
            &JsValue::from_str("nationality"),
            &JsValue::from_str(&passport_data.nationality),
        );
        let _ = Reflect::set(
            &passport_obj,
            &JsValue::from_str("age_over_18"),
            &JsValue::from_bool(passport_data.age_over_18),
        );
        let _ = Reflect::set(
            &passport_obj,
            &JsValue::from_str("expiry_date"),
            &JsValue::from_str(&passport_data.expiry_date),
        );
        let _ = Reflect::set(
            &passport_obj,
            &JsValue::from_str("document_type"),
            &JsValue::from_str(&passport_data.document_type),
        );

        let _ = Reflect::set(
            &proof_request,
            &JsValue::from_str("passportData"),
            &passport_obj,
        );

        let result = proof_fn
            .call1(sdk, &proof_request)
            .map_err(|e| format!("Failed to call generateProof: {:?}", e))?;

        // Clone result before using it
        if let Ok(promise) = result.clone().dyn_into::<Promise>() {
            let proof_data = JsFuture::from(promise)
                .await
                .map_err(|e| format!("Proof generation failed: {:?}", e))?;

            self.parse_proof_data(proof_data)
        } else {
            self.parse_proof_data(result)
        }
    }

    pub async fn verify_proof(&self, proof: &ZkPassportProof) -> Result<bool, String> {
        let sdk = self.sdk.as_ref().ok_or("ZK Passport SDK not available")?;

        // Call verifyProof method
        let verify_fn = Reflect::get(sdk, &JsValue::from_str("verifyProof"))
            .map_err(|_| "verifyProof method not found")?;

        if !verify_fn.is_function() {
            return Err("verifyProof is not a function".to_string());
        }

        let verify_fn = verify_fn.dyn_into::<js_sys::Function>().unwrap();

        // Create proof object manually
        let proof_obj = js_sys::Object::new();
        let _ = Reflect::set(
            &proof_obj,
            &JsValue::from_str("proof"),
            &JsValue::from_str(&proof.proof),
        );

        // Create public signals array
        let signals_array = js_sys::Array::new();
        for signal in &proof.public_signals {
            signals_array.push(&JsValue::from_str(signal));
        }
        let _ = Reflect::set(
            &proof_obj,
            &JsValue::from_str("public_signals"),
            &signals_array,
        );

        let _ = Reflect::set(
            &proof_obj,
            &JsValue::from_str("verification_key"),
            &JsValue::from_str(&proof.verification_key),
        );

        let result = verify_fn
            .call1(sdk, &proof_obj)
            .map_err(|e| format!("Failed to call verifyProof: {:?}", e))?;

        // Clone result before using it
        if let Ok(promise) = result.clone().dyn_into::<Promise>() {
            let verification_result = JsFuture::from(promise)
                .await
                .map_err(|e| format!("Proof verification failed: {:?}", e))?;

            Ok(verification_result.as_bool().unwrap_or(false))
        } else {
            Ok(result.as_bool().unwrap_or(false))
        }
    }

    fn parse_passport_data(&self, js_value: JsValue) -> Result<PassportData, String> {
        // Manual parsing instead of serde_wasm_bindgen
        if let Some(obj) = js_value.dyn_ref::<Object>() {
            let nationality = Reflect::get(obj, &JsValue::from_str("nationality"))
                .ok()
                .and_then(|v| v.as_string())
                .unwrap_or_else(|| "Unknown".to_string());

            let age_over_18 = Reflect::get(obj, &JsValue::from_str("age_over_18"))
                .ok()
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let expiry_date = Reflect::get(obj, &JsValue::from_str("expiry_date"))
                .ok()
                .and_then(|v| v.as_string())
                .unwrap_or_else(|| "Unknown".to_string());

            let document_type = Reflect::get(obj, &JsValue::from_str("document_type"))
                .ok()
                .and_then(|v| v.as_string())
                .unwrap_or_else(|| "Passport".to_string());

            Ok(PassportData {
                nationality,
                age_over_18,
                expiry_date,
                document_type,
            })
        } else {
            Err("Invalid passport data format".to_string())
        }
    }

    fn parse_proof_data(&self, js_value: JsValue) -> Result<ZkPassportProof, String> {
        // Manual parsing instead of serde_wasm_bindgen
        if let Some(obj) = js_value.dyn_ref::<Object>() {
            let proof = Reflect::get(obj, &JsValue::from_str("proof"))
                .ok()
                .and_then(|v| v.as_string())
                .unwrap_or_else(|| "".to_string());

            let verification_key = Reflect::get(obj, &JsValue::from_str("verification_key"))
                .ok()
                .and_then(|v| v.as_string())
                .unwrap_or_else(|| "".to_string());

            let public_signals =
                if let Ok(signals) = Reflect::get(obj, &JsValue::from_str("public_signals")) {
                    if let Some(array) = signals.dyn_ref::<js_sys::Array>() {
                        let mut signals_vec = Vec::new();
                        for i in 0..array.length() {
                            if let Some(signal) = array.get(i).as_string() {
                                signals_vec.push(signal);
                            }
                        }
                        signals_vec
                    } else {
                        Vec::new()
                    }
                } else {
                    Vec::new()
                };

            Ok(ZkPassportProof {
                proof,
                public_signals,
                verification_key,
            })
        } else {
            Err("Invalid proof data format".to_string())
        }
    }
}

impl Default for ZkPassportService {
    fn default() -> Self {
        Self::new()
    }
}

// Async wrappers for use with Yew callbacks
impl ZkPassportService {
    pub fn scan_passport_async(
        &self,
        on_success: Callback<PassportData>,
        on_error: Callback<String>,
    ) {
        if !self.is_available() {
            on_error.emit("ZK Passport SDK not available".to_string());
            return;
        }

        let service = self.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match service.scan_passport().await {
                Ok(data) => on_success.emit(data),
                Err(e) => on_error.emit(e),
            }
        });
    }

    pub fn generate_proof_async(
        &self,
        passport_data: PassportData,
        claim_type: String,
        on_success: Callback<ZkPassportProof>,
        on_error: Callback<String>,
    ) {
        if !self.is_available() {
            on_error.emit("ZK Passport SDK not available".to_string());
            return;
        }

        let service = self.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match service.generate_proof(&passport_data, &claim_type).await {
                Ok(proof) => on_success.emit(proof),
                Err(e) => on_error.emit(e),
            }
        });
    }

    pub fn verify_proof_async(
        &self,
        proof: ZkPassportProof,
        on_success: Callback<bool>,
        on_error: Callback<String>,
    ) {
        if !self.is_available() {
            on_error.emit("ZK Passport SDK not available".to_string());
            return;
        }

        let service = self.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match service.verify_proof(&proof).await {
                Ok(valid) => on_success.emit(valid),
                Err(e) => on_error.emit(e),
            }
        });
    }
}

// Make it cloneable for use in async contexts
impl Clone for ZkPassportService {
    fn clone(&self) -> Self {
        Self {
            sdk: self.sdk.clone(),
        }
    }
}
