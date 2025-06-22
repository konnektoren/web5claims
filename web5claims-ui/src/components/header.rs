use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let navigator = use_navigator().unwrap();
    let route = use_route::<Route>().unwrap_or(Route::Home);

    let go_home = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Home))
    };

    let go_to_issuer = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Issuer))
    };

    let go_to_lookup = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::CertificateLookup))
    };

    let go_to_verifier = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Verifier))
    };

    // Feature flags - check at compile time
    let has_issuer = cfg!(feature = "issuer");
    let has_verifier = cfg!(feature = "verifier");

    html! {
        <div class="navbar bg-primary text-primary-content shadow-lg">
            <div class="container mx-auto">
                <div class="navbar-start">
                    <div class="dropdown">
                        <div tabindex="0" role="button" class="btn btn-ghost lg:hidden">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h8m-8 6h16" />
                            </svg>
                        </div>
                        <ul tabindex="0" class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-primary rounded-box w-52">
                            <li>
                                <button onclick={go_home.clone()}>
                                    {"🏠 Home"}
                                </button>
                            </li>
                            if has_issuer {
                                <li>
                                    <button onclick={go_to_issuer.clone()}>
                                        {"🎓 Issuer"}
                                    </button>
                                </li>
                                <li>
                                    <button onclick={go_to_lookup.clone()}>
                                        {"🔍 Lookup"}
                                    </button>
                                </li>
                            }
                            if has_verifier {
                                <li>
                                    <button onclick={go_to_verifier.clone()}>
                                        {"✅ Verifier"}
                                    </button>
                                </li>
                            }
                            <li>
                                <a href={Route::zkpass_external_url()} target="_blank">
                                    {"🛂 ZKPass"}
                                </a>
                            </li>
                        </ul>
                    </div>

                    <button
                        class="flex items-center space-x-3 btn btn-ghost text-xl"
                        onclick={&go_home}
                    >
                        <div class="text-3xl">{"🔐"}</div>
                        <div>
                            <div class="font-bold">{"Web5 Claims"}</div>
                            <div class="text-sm opacity-80">{"ZK Language Certificates"}</div>
                        </div>
                    </button>
                </div>

                <div class="navbar-center hidden lg:flex">
                    <ul class="menu menu-horizontal px-1">
                        <li>
                            <button
                                class={classes!(
                                    "btn", "btn-ghost",
                                    if matches!(route, Route::Home) { "btn-active" } else { "" }
                                )}
                                onclick={&go_home}
                            >
                                {"🏠 Home"}
                            </button>
                        </li>
                        if has_issuer {
                            <li>
                                <button
                                    class={classes!(
                                        "btn", "btn-ghost",
                                        if matches!(route, Route::Issuer) { "btn-active" } else { "" }
                                    )}
                                    onclick={go_to_issuer}
                                >
                                    {"🎓 Issuer"}
                                </button>
                            </li>
                            <li>
                                <button
                                    class={classes!(
                                        "btn", "btn-ghost",
                                        if matches!(route, Route::CertificateLookup) { "btn-active" } else { "" }
                                    )}
                                    onclick={go_to_lookup}
                                >
                                    {"🔍 Lookup"}
                                </button>
                            </li>
                        }
                        if has_verifier {
                            <li>
                                <button
                                    class={classes!(
                                        "btn", "btn-ghost",
                                        if matches!(route, Route::Verifier | Route::VerifyProof { .. }) { "btn-active" } else { "" }
                                    )}
                                    onclick={go_to_verifier}
                                >
                                    {"✅ Verifier"}
                                </button>
                            </li>
                        }
                        <li>
                            <a
                                href={Route::zkpass_external_url()}
                                target="_blank"
                                class="btn btn-ghost"
                                title="Advanced identity verification with passport scanning"
                            >
                                {"🛂 ZKPass"}
                            </a>
                        </li>
                    </ul>
                </div>

                <div class="navbar-end">
                    <div class="badge badge-accent">{"ZK Hack Berlin"}</div>
                </div>
            </div>
        </div>
    }
}
