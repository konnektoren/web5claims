use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let navigator = use_navigator().unwrap();

    let go_to_issuer = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Issuer))
    };

    let go_to_verifier = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Verifier))
    };

    html! {
        <div class="min-h-screen bg-base-200">
            <div class="hero min-h-screen">
                <div class="hero-content text-center">
                    <div class="max-w-4xl">
                        <div class="text-6xl mb-6">{"🔐"}</div>
                        <h1 class="text-5xl font-bold text-primary mb-6">
                            {"Web5 Claims"}
                        </h1>
                        <p class="text-xl text-base-content/80 mb-8 max-w-2xl mx-auto">
                            {"Zero-knowledge verifiable credential system for language learning achievements. "}
                            {"Prove your skills without revealing private data."}
                        </p>

                        <div class="grid md:grid-cols-2 gap-6 mb-8">
                            // Issuer Card
                            <div class="card bg-base-100 shadow-xl">
                                <div class="card-body">
                                    <div class="text-4xl mb-4">{"🎓"}</div>
                                    <h2 class="card-title justify-center text-2xl">{"Certificate Issuer"}</h2>
                                    <p class="text-base-content/70 mb-4">
                                        {"Generate language learning certificates and create zero-knowledge proofs."}
                                    </p>
                                    <div class="card-actions justify-center">
                                        <button
                                            class="btn btn-primary btn-lg"
                                            onclick={go_to_issuer}
                                        >
                                            {"Create Certificate"}
                                        </button>
                                    </div>
                                </div>
                            </div>

                            // Verifier Card
                            <div class="card bg-base-100 shadow-xl">
                                <div class="card-body">
                                    <div class="text-4xl mb-4">{"🔍"}</div>
                                    <h2 class="card-title justify-center text-2xl">{"Proof Verifier"}</h2>
                                    <p class="text-base-content/70 mb-4">
                                        {"Verify zero-knowledge proofs without accessing private information."}
                                    </p>
                                    <div class="card-actions justify-center">
                                        <button
                                            class="btn btn-secondary btn-lg"
                                            onclick={go_to_verifier}
                                        >
                                            {"Verify Proof"}
                                        </button>
                                    </div>
                                </div>
                            </div>

                            // ZKPass Card
                            <div class="card bg-base-100 shadow-xl">
                                <div class="card-body">
                                    <div class="text-4xl mb-4">{"🛂"}</div>
                                    <h2 class="card-title justify-center text-xl">{"ZKPass Identity"}</h2>
                                    <p class="text-base-content/70 mb-4">
                                        {"Advanced identity verification using passport scanning and zero-knowledge proofs."}
                                    </p>
                                    <div class="card-actions justify-center">
                                        <a
                                            href={Route::zkpass_external_url()}
                                            target="_blank"
                                            class="btn btn-accent btn-lg"
                                        >
                                            {"Verify Identity"}
                                        </a>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Features Section
                        <div class="grid md:grid-cols-4 gap-4 text-sm">
                            <div class="bg-base-100 p-4 rounded-lg">
                                <div class="text-2xl mb-2">{"🌍"}</div>
                                <h3 class="font-semibold">{"Language Proficiency"}</h3>
                                <p class="text-base-content/70">{"Prove language level without revealing exact scores"}</p>
                            </div>
                            <div class="bg-base-100 p-4 rounded-lg">
                                <div class="text-2xl mb-2">{"📊"}</div>
                                <h3 class="font-semibold">{"Performance Threshold"}</h3>
                                <p class="text-base-content/70">{"Verify achievements above thresholds privately"}</p>
                            </div>
                            <div class="bg-base-100 p-4 rounded-lg">
                                <div class="text-2xl mb-2">{"🛂"}</div>
                                <h3 class="font-semibold">{"Identity Verification"}</h3>
                                <p class="text-base-content/70">{"Passport-based identity proofs with ZKPass"}</p>
                            </div>
                            <div class="bg-base-100 p-4 rounded-lg">
                                <div class="text-2xl mb-2">{"⛓️"}</div>
                                <h3 class="font-semibold">{"Blockchain Verified"}</h3>
                                <p class="text-base-content/70">{"Powered by Aleo and Arbitrum ZK technology"}</p>
                            </div>
                        </div>

                        // ZK Hack Berlin Badge
                        <div class="mt-8">
                            <div class="badge badge-accent badge-lg">
                                {"🏆 ZK Hack Berlin 2024"}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
