use crate::components::{
    certificate::{ErrorDisplay, ProofDisplay},
    layout::PageLayout,
    ui::{Button, Card},
};
use crate::services::ZkService;
use crate::utils::proof_link::{decode_proof_from_url, generate_verify_link};
use wasm_bindgen::JsCast;
use web5claims::{VerificationResult, ZkProofClaim};
use yew::prelude::*;

#[function_component(VerifierPage)]
pub fn verifier_page() -> Html {
    let zk_service = use_state(|| ZkService::new());
    let proof_json = use_state(|| String::new());
    let proof = use_state(|| None::<ZkProofClaim>);
    let verification_result = use_state(|| None::<VerificationResult>);
    let is_verifying = use_state(|| false);
    let error_message = use_state(|| None::<String>);
    let verify_link = use_state(|| None::<String>);

    let on_proof_input = {
        let proof_json = proof_json.clone();
        let proof = proof.clone();
        let error_message = error_message.clone();

        Callback::from(move |value: String| {
            proof_json.set(value.clone());

            if !value.trim().is_empty() {
                // Try to parse the proof
                match serde_json::from_str::<ZkProofClaim>(&value) {
                    Ok(parsed_proof) => {
                        proof.set(Some(parsed_proof));
                        error_message.set(None);
                    }
                    Err(_) => {
                        proof.set(None);
                        error_message.set(Some(
                            "Invalid proof format. Please paste a valid JSON proof.".to_string(),
                        ));
                    }
                }
            } else {
                proof.set(None);
                error_message.set(None);
            }
        })
    };

    let verify_proof = {
        let proof = proof.clone();
        let verification_result = verification_result.clone();
        let is_verifying = is_verifying.clone();
        let error_message = error_message.clone();
        let zk_service = zk_service.clone();

        Callback::from(move |_| {
            if let Some(proof_claim) = (*proof).clone() {
                is_verifying.set(true);
                error_message.set(None);

                let on_success = {
                    let verification_result = verification_result.clone();
                    let is_verifying = is_verifying.clone();
                    Callback::from(move |result| {
                        verification_result.set(Some(result));
                        is_verifying.set(false);
                    })
                };

                let on_error = {
                    let error_message = error_message.clone();
                    let is_verifying = is_verifying.clone();
                    Callback::from(move |error: String| {
                        error_message.set(Some(error));
                        is_verifying.set(false);
                    })
                };

                zk_service.verify_proof(proof_claim, on_success, on_error);
            }
        })
    };

    let generate_link = {
        let proof = proof.clone();
        let verify_link = verify_link.clone();
        let error_message = error_message.clone();

        Callback::from(move |_| {
            if let Some(proof_claim) = (*proof).clone() {
                match generate_verify_link(&proof_claim) {
                    Ok(link) => {
                        verify_link.set(Some(link));
                        error_message.set(None);
                    }
                    Err(e) => {
                        error_message.set(Some(format!("Failed to generate link: {}", e)));
                    }
                }
            }
        })
    };

    let on_dismiss_error = {
        let error_message = error_message.clone();
        Callback::from(move |_| {
            error_message.set(None);
        })
    };

    html! {
        <PageLayout>
            <div class="max-w-4xl mx-auto">
                <div class="text-center mb-8">
                    <h1 class="text-3xl font-bold mb-2">{"🔍 Proof Verifier"}</h1>
                    <p class="text-base-content/70">
                        {"Verify zero-knowledge proofs without accessing private information"}
                    </p>
                </div>

                <div class="grid lg:grid-cols-2 gap-8">
                    // Input Section
                    <Card title="📋 Paste Proof Data">
                        <div class="space-y-4">
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text font-semibold">{"ZK Proof JSON:"}</span>
                                </label>
                                <textarea
                                    class="textarea textarea-bordered h-32 font-mono text-xs"
                                    placeholder="Paste the ZK proof JSON data here..."
                                    value={(*proof_json).clone()}
                                    oninput={move |e: InputEvent| {
                                        if let Some(target) = e.target() {
                                            if let Ok(textarea) = target.dyn_into::<web_sys::HtmlTextAreaElement>() {
                                                on_proof_input.emit(textarea.value());
                                            }
                                        }
                                    }}
                                ></textarea>
                            </div>

                            <ErrorDisplay
                                error={(*error_message).clone()}
                                on_dismiss={on_dismiss_error}
                                dismissible={true}
                            />

                            if proof.is_some() {
                                <div class="alert alert-success">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                    </svg>
                                    <span>{"Valid proof format detected!"}</span>
                                </div>
                            }

                            <div class="flex gap-2">
                                <Button
                                    variant="primary"
                                    size="md"
                                    loading={*is_verifying}
                                    disabled={proof.is_none() || *is_verifying}
                                    onclick={&verify_proof}
                                    class="flex-1"
                                >
                                    if *is_verifying {
                                        {"Verifying..."}
                                    } else {
                                        {"🔍 Verify Proof"}
                                    }
                                </Button>

                                <Button
                                    variant="outline"
                                    size="md"
                                    disabled={proof.is_none()}
                                    onclick={generate_link}
                                    class="flex-1"
                                >
                                    {"🔗 Generate Link"}
                                </Button>
                            </div>

                            if let Some(link) = (*verify_link).clone() {
                                <div class="card bg-base-200 p-4">
                                    <h3 class="font-semibold mb-2">{"🔗 Shareable Verify Link:"}</h3>
                                    <div class="flex gap-2">
                                        <input
                                            type="text"
                                            class="input input-bordered flex-1 text-xs font-mono"
                                            value={link.clone()}
                                            readonly=true
                                        />
                                        <Button
                                            variant="ghost"
                                            size="sm"
                                            onclick={move |_| {
                                                let link = link.clone();
                                                wasm_bindgen_futures::spawn_local(async move {
                                                    if let Some(window) = web_sys::window() {
                                                        let _ = window.navigator().clipboard().write_text(&link);
                                                    }
                                                });
                                            }}
                                        >
                                            {"📋"}
                                        </Button>
                                    </div>
                                    <div class="text-xs text-base-content/70 mt-2">
                                        {"Share this link to let others verify the proof"}
                                    </div>
                                </div>
                            }
                        </div>
                    </Card>

                    // Results Section
                    <Card title="✅ Verification Results">
                        <ProofDisplay
                            proof={(*proof).clone()}
                            verification_result={(*verification_result).clone()}
                            on_verify={verify_proof.clone()}
                            on_copy_proof={Callback::noop()}
                            is_verifying={*is_verifying}
                        />
                    </Card>
                </div>
            </div>
        </PageLayout>
    }
}
