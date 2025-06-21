use crate::components::certificate_image::Web5CertificateImage;
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

    let copy_certificate_data = {
        let state = props.state.clone();
        Callback::from(move |_| {
            if let Some(cert) = &state.certificate_data {
                let cert_data = cert.to_base64();

                // Use gloo for clipboard access (more reliable in WASM)
                #[cfg(feature = "clipboard")]
                {
                    use gloo::utils::document;
                    use web_sys::HtmlInputElement;

                    // Create a temporary input element to copy text
                    if let Some(document) = document().dyn_into::<web_sys::HtmlDocument>().ok() {
                        if let Ok(input) = document.create_element("input") {
                            let input: HtmlInputElement = input.dyn_into().unwrap();
                            input.set_value(&cert_data);
                            document.body().unwrap().append_child(&input).unwrap();
                            input.select();
                            let _ = document.exec_command("copy");
                            document.body().unwrap().remove_child(&input).unwrap();
                            gloo::console::log!("Certificate data copied to clipboard");
                        }
                    }
                }

                #[cfg(not(feature = "clipboard"))]
                {
                    gloo::console::log!("Certificate data (copy manually):", &cert_data);
                }
            }
        })
    };

    match &props.state.certificate_data {
        Some(cert) => {
            // Convert date to string
            let formatted_date = cert.date.format("%Y-%m-%d").to_string();

            html! {
                            <div class="space-y-6">
                                // Certificate Image Display
                                <div class="card bg-gradient-to-br from-blue-50 to-indigo-100 border border-blue-200">
                                    <div class="card-body p-6">
                                        <h3 class="card-title text-xl mb-4 text-center">
                                            {"🎓 Certificate of Achievement"}
                                        </h3>

                                        <div class="flex justify-center mb-4">
                                            <Web5CertificateImage certificate_data={cert.clone()} />
                                        </div>

                                        // Certificate Actions
                                        <div class="flex flex-wrap gap-2 justify-center">
                                            <button
                                                class="btn btn-outline btn-sm"
                                                onclick={copy_certificate_data}
                                                title="Copy certificate data"
                                            >
                                                {"📋 Copy Data"}
                                            </button>
                                            <a
                                                class="btn btn-outline btn-sm"
                                                href={format!("data:text/plain;charset=utf-8,{}", cert.to_base64())}
                                                download="certificate.txt"
                                            >
                                                {"💾 Download"}
                                            </a>
                                        </div>
                                    </div>
                                </div>

                                // Certificate Summary Stats
                                <div class="stats stats-vertical lg:stats-horizontal shadow w-full">
                                    <div class="stat">
                                        <div class="stat-figure text-secondary">
                                            {"🎯"}
                                        </div>
                                        <div class="stat-title">{"Performance"}</div>
                                        <div class="stat-value text-primary">{cert.performance_percentage}{"%"}</div>
                                        <div class="stat-desc">
                                            {format!("{}/{} challenges", cert.solved_challenges, cert.total_challenges)}
                                        </div>
                                    </div>

                                    <div class="stat">
                                        <div class="stat-figure text-secondary">
                                            {"🌍"}
                                        </div>
                                        <div class="stat-title">{"Course"}</div>
                                        <div class="stat-value text-secondary text-lg">
                                            {cert.game_path_name.replace("_", " ")}
                                        </div>
                                        <div class="stat-desc">{formatted_date}</div>
                                    </div>

                                    <div class="stat">
                                        <div class="stat-figure text-secondary">
                                            {"👤"}
                                        </div>
                                        <div class="stat-title">{"Student"}</div>
                                        <div class="stat-value text-accent text-lg">{&cert.profile_name}</div>
                                        <div class="stat-desc">{"Verified learner"}</div>
                                    </div>
                                </div>

                                // ZK Proof Section
                                <div class="card bg-gradient-to-r from-primary to-secondary text-primary-content">
                                    <div class="card-body">
                                        <h3 class="card-title text-xl mb-3">
                                            {"🔐 Zero-Knowledge Proof Generation"}
                                        </h3>
                                        <p class="text-sm opacity-90 mb-4">
                                            {"Generate a ZK proof to verify your language skills without revealing exact scores or personal information."}
                                        </p>

                                        // Proof Claims Preview
                                        <div class="bg-base-100 bg-opacity-20 rounded-lg p-4 mb-4">
                                            <h4 class="font-semibold mb-2">{"🔍 Proof Claims:"}</h4>
                                            <div class="grid grid-cols-1 md:grid-cols-2 gap-2 text-sm">
                                                <div class="flex items-center gap-2">
                                                    <span class="badge badge-success badge-sm">{"Public"}</span>
                                                    <span>{"Level ≥ B2"}</span>
                                                </div>
                                                <div class="flex items-center gap-2">
                                                    <span class="badge badge-success badge-sm">{"Public"}</span>
                                                    <span>{"Score ≥ 90%"}</span>
                                                </div>
                                                <div class="flex items-center gap-2">
                                                    <span class="badge badge-error badge-sm">{"Private"}</span>
                                                    <span>{"Exact score"}</span>
                                                </div>
                                                <div class="flex items-center gap-2">
                                                    <span class="badge badge-error badge-sm">{"Private"}</span>
                                                    <span>{"Student identity"}</span>
                                                </div>
                                            </div>
                                        </div>

                                        // ZK Proof Actions
                                        <div class="flex flex-col sm:flex-row gap-3">
                                            <button
                                                class="btn btn-accent flex-1"
                                                onclick={generate_zk_proof}
                                            >
                                                {"🚀 Generate Aleo ZK Proof"}
                                            </button>
                                            <button
                                                class="btn btn-outline btn-accent"
                                                disabled={true}
                                            >
                                                {"⚡ Stylus Verifier"}
                                            </button>
                                        </div>

                                        <div class="text-xs opacity-75 mt-2">
                                            {"🏆 Building for ZK Hack Berlin - Aleo, Arbitrum Stylus, ZKPassport & Xion tracks"}
                                        </div>
                                    </div>
                                </div>

                                // Technical Details (Collapsible)
                                <div class="collapse collapse-arrow bg-base-200">
                                    <input type="checkbox" />
                                    <div class="collapse-title text-sm font-medium">
                                        {"🔧 Technical Details & Certificate Data"}
                                    </div>
                                    <div class="collapse-content">
                                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
                                            // Certificate Raw Data
                                            <div>
                                                <h4 class="font-semibold mb-2">{"📜 Certificate Structure:"}</h4>
                                                <pre class="text-xs bg-base-300 p-3 rounded overflow-auto max-h-64">
                                                    {format!("{:#?}", cert)}
                                                </pre>
                                            </div>

                                            // ZK Circuit Preview
                                            <div>
                                                <h4 class="font-semibold mb-2">{"⚙️ ZK Circuit Preview:"}</h4>
                                                <pre class="text-xs bg-base-300 p-3 rounded overflow-auto max-h-64">
            {r#"// Aleo Language Skills Verification
transition verify_language_skill(
    private performance: u8,      // 94
    private level_code: u8,       // 4 (B2)
    private completion_date: u64, // timestamp
    public min_performance: u8,   // 90
    public min_level: u8,         // 4 (B2)
    public min_date: u64         // 2023-01-01
) -> bool {
    let performance_check = performance >= min_performance;
    let level_check = level_code >= min_level;
    let date_check = completion_date >= min_date;

    return performance_check && level_check && date_check;
}"#}
                                                </pre>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }
        }
        None => {
            html! {
                <div class="hero min-h-[400px] bg-base-200 rounded-lg">
                    <div class="hero-content text-center">
                        <div class="max-w-md">
                            <div class="text-6xl mb-4">{"📝"}</div>
                            <h3 class="text-2xl font-bold mb-4">{"Ready for ZK Magic!"}</h3>
                            <p class="text-gray-600 mb-6">
                                {"Generate a language learning certificate first, then create zero-knowledge proofs to verify your skills without revealing personal information."}
                            </p>
                            <div class="flex flex-wrap gap-2 justify-center text-sm">
                                <div class="badge badge-primary">{"🔐 Privacy-First"}</div>
                                <div class="badge badge-secondary">{"⚡ Aleo Powered"}</div>
                                <div class="badge badge-accent">{"🏆 ZK Hack Berlin"}</div>
                            </div>
                        </div>
                    </div>
                </div>
            }
        }
    }
}
