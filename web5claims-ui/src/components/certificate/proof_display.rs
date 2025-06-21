use crate::components::ui::Button;
use web5claims::{VerificationResult, ZkProofClaim};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProofDisplayProps {
    pub proof: Option<ZkProofClaim>,
    pub verification_result: Option<VerificationResult>,
    pub on_verify: Callback<MouseEvent>,
    pub on_copy_proof: Callback<MouseEvent>,
    #[prop_or_default]
    pub is_verifying: bool,
    #[prop_or_default]
    pub copy_status: Option<String>,
    #[prop_or_default]
    pub class: String,
}

#[function_component(ProofDisplay)]
pub fn proof_display(props: &ProofDisplayProps) -> Html {
    if props.proof.is_none() {
        return html! {
            <div class={classes!("alert", "alert-warning", props.class.clone())}>
                <div class="flex">
                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
                    </svg>
                    <span>{"No ZK proof generated yet. Please generate a proof first."}</span>
                </div>
            </div>
        };
    }

    let proof = props.proof.as_ref().unwrap();

    html! {
        <div class={classes!("space-y-4", props.class.clone())}>
            // Proof Information
            <div class="card bg-base-200 p-4">
                <h3 class="font-semibold text-lg mb-3">{"🔐 Generated Proof"}</h3>

                <div class="space-y-2 text-sm">
                    <div class="flex justify-between">
                        <span class="font-medium">{"Claim Type:"}</span>
                        <span class="text-primary">
                            {match &proof.claim_type {
                                web5claims::ClaimType::LanguageProficiency { language, min_level } =>
                                    format!("{} (Min: {:?})", language, min_level),
                                web5claims::ClaimType::PerformanceThreshold { min_percentage } =>
                                    format!("Performance ≥ {}%", min_percentage),
                                web5claims::ClaimType::Combined { criteria } =>
                                    format!("Combined ({} claims)", criteria.len()),
                                web5claims::ClaimType::CompletionDate { after_date } =>
                                    format!("Completed after {}", after_date.format("%Y-%m-%d")),
                            }}
                        </span>
                    </div>

                    <div class="flex justify-between">
                        <span class="font-medium">{"Platform:"}</span>
                        <span class="badge badge-accent">{&proof.metadata.platform}</span>
                    </div>

                    <div class="flex justify-between items-start">
                        <span class="font-medium">{"Proof ID:"}</span>
                        <span class="text-xs font-mono bg-base-300 px-2 py-1 rounded max-w-xs break-all">
                            {&proof.proof_id[..std::cmp::min(16, proof.proof_id.len())]}
                            if proof.proof_id.len() > 16 {
                                {"..."}
                            }
                        </span>
                    </div>

                    <div class="flex justify-between items-start">
                        <span class="font-medium">{"Circuit ID:"}</span>
                        <span class="text-xs font-mono bg-base-300 px-2 py-1 rounded max-w-xs break-all">
                            {&proof.proof_data.circuit_id}
                        </span>
                    </div>
                </div>
            </div>

            // Action Buttons
            <div class="flex gap-2">
                <Button
                    variant="primary"
                    size="md"
                    loading={props.is_verifying}
                    disabled={props.is_verifying}
                    onclick={props.on_verify.clone()}
                    class="flex-1"
                >
                    if props.is_verifying {
                        {"Verifying..."}
                    } else {
                        {"🔍 Verify Proof"}
                    }
                </Button>

                <Button
                    variant="ghost"
                    size="md"
                    onclick={props.on_copy_proof.clone()}
                    class="flex-1"
                >
                    {"📋 Copy Proof"}
                </Button>
            </div>

            // Copy Status
            if let Some(status) = &props.copy_status {
                <div class="alert alert-success">
                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                    <span>{status}</span>
                </div>
            }

            // Verification Result
            if let Some(result) = &props.verification_result {
                <div class={classes!(
                    "alert",
                    if result.is_valid && result.requirements_met { "alert-success" } else { "alert-error" }
                )}>
                    <div class="flex">
                        if result.is_valid && result.requirements_met {
                            <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                        } else {
                            <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                        }
                        <div>
                            <div class="font-semibold">
                                if result.is_valid && result.requirements_met {
                                    {"✅ Proof Verified Successfully"}
                                } else {
                                    {"❌ Proof Verification Failed"}
                                }
                            </div>
                            <div class="text-sm mt-1">
                                {"Valid: "}{if result.is_valid { "✓" } else { "✗" }}
                                {", Requirements Met: "}{if result.requirements_met { "✓" } else { "✗" }}
                            </div>
                            <div class="text-xs mt-2 opacity-75">
                                {"Platform: "}{&result.details.platform}
                                {", Verified: "}{result.details.verified_at.format("%Y-%m-%d %H:%M:%S")}
                            </div>
                            if !result.warnings.is_empty() {
                                <div class="text-xs mt-2">
                                    <strong>{"Warnings:"}</strong>
                                    <ul class="list-disc list-inside">
                                        {for result.warnings.iter().map(|warning| {
                                            html! { <li>{warning}</li> }
                                        })}
                                    </ul>
                                </div>
                            }
                        </div>
                    </div>
                </div>
            }

            // Proof Data (Collapsible)
            <details class="collapse bg-base-200">
                <summary class="collapse-title text-sm font-medium">
                    {"🔍 View Raw Proof Data"}
                </summary>
                <div class="collapse-content">
                    <div class="space-y-2">
                        <div>
                            <label class="text-xs font-medium">{"Public Inputs:"}</label>
                            <textarea
                                class="textarea textarea-bordered w-full h-24 text-xs font-mono"
                                readonly=true
                                value={serde_json::to_string_pretty(&proof.public_inputs).unwrap_or_else(|_| "Error serializing public inputs".to_string())}
                            ></textarea>
                        </div>
                        <div>
                            <label class="text-xs font-medium">{"Full Proof:"}</label>
                            <textarea
                                class="textarea textarea-bordered w-full h-32 text-xs font-mono"
                                readonly=true
                                value={serde_json::to_string_pretty(proof).unwrap_or_else(|_| "Error serializing proof".to_string())}
                            ></textarea>
                        </div>
                    </div>
                </div>
            </details>
        </div>
    }
}
