use crate::components::{
    certificate::{ErrorDisplay, ProofDisplay},
    layout::PageLayout,
    ui::Card,
};
use crate::router::Route;
use crate::services::ZkService;
use crate::utils::proof_link::decode_proof_from_query;
use web5claims::{VerificationResult, ZkProofClaim};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(VerifyProofPage)]
pub fn verify_proof_page() -> Html {
    log::info!("VerifyProofPage component is being rendered");

    let zk_service = use_state(|| ZkService::new());
    let proof = use_state(|| None::<ZkProofClaim>);
    let verification_result = use_state(|| None::<VerificationResult>);
    let is_verifying = use_state(|| false);
    let error_message = use_state(|| None::<String>);
    let navigator = use_navigator().unwrap();

    // Decode proof from query parameters on component mount
    {
        let proof = proof.clone();
        let error_message = error_message.clone();

        use_effect_with((), move |_| {
            log::info!("Starting proof decoding from query parameters");
            match decode_proof_from_query() {
                Ok(Some(decoded_proof)) => {
                    log::info!("Successfully decoded proof from URL");
                    proof.set(Some(decoded_proof));
                }
                Ok(None) => {
                    log::warn!("No proof data found in URL");
                    error_message.set(Some(
                        "No proof data found in URL. Please check the verification link."
                            .to_string(),
                    ));
                }
                Err(e) => {
                    log::error!("Failed to decode proof: {}", e);
                    error_message.set(Some(format!("Failed to decode proof: {}", e)));
                }
            }
            || ()
        });
    }

    // Auto-verify proof once loaded
    {
        let proof = proof.clone();
        let verification_result = verification_result.clone();
        let is_verifying = is_verifying.clone();
        let error_message = error_message.clone();
        let zk_service = zk_service.clone();

        use_effect_with(proof.clone(), move |proof_handle| {
            if let Some(proof_claim) = (**proof_handle).clone() {
                is_verifying.set(true);

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
            || ()
        });
    }

    let go_to_verifier = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Verifier))
    };

    let go_home = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Home))
    };

    let on_dismiss_error = {
        let error_message = error_message.clone();
        Callback::from(move |_| {
            error_message.set(None);
        })
    };

    log::info!("Rendering VerifyProofPage HTML");

    html! {
        <PageLayout>
            <div class="max-w-4xl mx-auto">
                <div class="text-center mb-8">
                    <h1 class="text-3xl font-bold mb-2">{"🔗 Verifying Shared Proof"}</h1>
                    <p class="text-base-content/70">
                        {"Automatically verifying the zero-knowledge proof from the shared link"}
                    </p>
                </div>

                <div class="space-y-6">
                    <ErrorDisplay
                        error={(*error_message).clone()}
                        on_dismiss={on_dismiss_error}
                        dismissible={true}
                    />

                    if *is_verifying && proof.is_some() {
                        <div class="alert alert-info">
                            <span class="loading loading-spinner loading-sm"></span>
                            <span>{"Verifying proof... Please wait."}</span>
                        </div>
                    }

                    <Card title="🔍 Proof Verification">
                        <ProofDisplay
                            proof={(*proof).clone()}
                            verification_result={(*verification_result).clone()}
                            on_verify={Callback::noop()}
                            on_copy_proof={Callback::noop()}
                            is_verifying={*is_verifying}
                        />
                    </Card>

                    // Navigation buttons
                    <div class="flex gap-4 justify-center">
                        <button
                            class="btn btn-outline"
                            onclick={go_home}
                        >
                            {"🏠 Home"}
                        </button>
                        <button
                            class="btn btn-primary"
                            onclick={go_to_verifier}
                        >
                            {"🔍 Verify Another Proof"}
                        </button>
                    </div>
                </div>
            </div>
        </PageLayout>
    }
}
