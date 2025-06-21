use crate::types::AppState;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CertificateDisplayProps {
    pub state: UseStateHandle<AppState>,
}

#[function_component(CertificateDisplay)]
pub fn certificate_display(props: &CertificateDisplayProps) -> Html {
    let generate_zk_proof = {
        let _state = props.state.clone();
        Callback::from(move |_| {
            // TODO: Implement ZK proof generation with Aleo
            gloo::console::log!("Generating ZK proof...");
        })
    };

    match &props.state.certificate_data {
        Some(cert) => {
            html! {
                <div class="space-y-4">
                    // Certificate Summary
                    <div class="card bg-base-200 p-4">
                        <h3 class="font-bold text-lg mb-2">{"📜 Certificate Summary"}</h3>
                        <div class="space-y-2 text-sm">
                            <div><strong>{"Course:"}</strong> {&cert.game_path_name}</div>
                            <div><strong>{"Student:"}</strong> {&cert.profile_name}</div>
                            <div><strong>{"Performance:"}</strong> {cert.performance_percentage}{"% "}
                                <span class="badge badge-success">
                                    {format!("{}/{}", cert.solved_challenges, cert.total_challenges)}
                                </span>
                            </div>
                            <div><strong>{"Date:"}</strong> {cert.date.format("%Y-%m-%d")}</div>
                        </div>
                    </div>

                    // ZK Proof Section
                    <div class="card bg-gradient-to-r from-primary to-secondary text-primary-content p-4">
                        <h3 class="font-bold text-lg mb-2">{"🔐 Zero-Knowledge Proof"}</h3>
                        <p class="text-sm opacity-90 mb-4">
                            {"Generate a ZK proof to verify your language skills without revealing exact scores."}
                        </p>

                        <div class="grid grid-cols-1 gap-2 mb-4 text-sm">
                            <div class="flex justify-between">
                                <span>{"✅ Prove: Level >= B2"}</span>
                                <span class="opacity-80">{"Public"}</span>
                            </div>
                            <div class="flex justify-between">
                                <span>{"✅ Prove: Passed >= 90%"}</span>
                                <span class="opacity-80">{"Public"}</span>
                            </div>
                            <div class="flex justify-between">
                                <span>{"❌ Hide: Exact score"}</span>
                                <span class="opacity-80">{"Private"}</span>
                            </div>
                            <div class="flex justify-between">
                                <span>{"❌ Hide: Student name"}</span>
                                <span class="opacity-80">{"Private"}</span>
                            </div>
                        </div>

                        <button
                            class="btn btn-accent w-full"
                            onclick={generate_zk_proof}
                        >
                            {"🚀 Generate ZK Proof with Aleo"}
                        </button>
                    </div>

                    // Certificate Data (for debugging)
                    <div class="collapse collapse-arrow bg-base-200">
                        <input type="checkbox" />
                        <div class="collapse-title text-sm font-medium">
                            {"🔍 View Certificate Data (Debug)"}
                        </div>
                        <div class="collapse-content">
                            <pre class="text-xs bg-base-300 p-2 rounded overflow-auto">
                                {format!("{:#?}", cert)}
                            </pre>
                        </div>
                    </div>
                </div>
            }
        }
        None => {
            html! {
                <div class="text-center py-8">
                    <div class="text-6xl mb-4">{"📝"}</div>
                    <p class="text-gray-500">
                        {"Generate a certificate first to create ZK proofs"}
                    </p>
                </div>
            }
        }
    }
}
