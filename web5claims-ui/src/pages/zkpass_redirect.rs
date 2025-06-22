use crate::components::layout::PageLayout;
use crate::router::Route;
use yew::prelude::*;

#[function_component(ZkPassRedirectPage)]
pub fn zkpass_redirect_page() -> Html {
    // Redirect to external ZKPass app
    use_effect_with((), |_| {
        let zkpass_url = Route::zkpass_external_url();

        // Give user a moment to see the redirect message, then redirect
        gloo::timers::callback::Timeout::new(2000, move || {
            if let Some(window) = web_sys::window() {
                let _ = window.location().set_href(&zkpass_url);
            }
        })
        .forget();

        || ()
    });

    html! {
        <PageLayout>
            <div class="hero min-h-96">
                <div class="hero-content text-center">
                    <div class="max-w-md">
                        <div class="text-6xl mb-4">{"🛂"}</div>
                        <h1 class="text-3xl font-bold mb-4">{"Redirecting to ZKPass"}</h1>
                        <p class="mb-6">
                            {"You're being redirected to the advanced ZKPass identity verification system..."}
                        </p>

                        <div class="loading loading-spinner loading-lg text-primary mb-4"></div>

                        <div class="space-y-4">
                            <p class="text-sm text-base-content/70">
                                {"If you're not redirected automatically, "}
                                <a
                                    href={Route::zkpass_external_url()}
                                    class="link link-primary"
                                    target="_blank"
                                >
                                    {"click here"}
                                </a>
                            </p>

                            <div class="divider">{"OR"}</div>

                            <a
                                href="/"
                                class="btn btn-outline"
                            >
                                {"← Back to Web5 Claims"}
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </PageLayout>
    }
}
