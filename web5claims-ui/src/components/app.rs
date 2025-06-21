use crate::components::layout::PageLayout;
use crate::pages::{HomePage, IssuerPage, NotFoundPage, VerifierPage, VerifyProofPage};
use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    // Debug logging
    log::info!("Current route: {:?}", route);

    // Log current URL for debugging
    if let Some(window) = web_sys::window() {
        if let Ok(href) = window.location().href() {
            log::info!("Current URL: {}", href);
        }
        if let Ok(hash) = window.location().hash() {
            log::info!("Current hash: {}", hash);
        }
    }

    // Update document title
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        document.set_title(route.title());
    }

    match route {
        Route::Home => {
            log::info!("Rendering HomePage");
            html! { <HomePage /> }
        }
        Route::Issuer => {
            log::info!("Rendering IssuerPage");
            #[cfg(feature = "issuer")]
            {
                html! { <IssuerPage /> }
            }
            #[cfg(not(feature = "issuer"))]
            {
                html! { <FeatureNotEnabled feature="issuer" /> }
            }
        }
        Route::Verifier => {
            log::info!("Rendering VerifierPage");
            #[cfg(feature = "verifier")]
            {
                html! { <VerifierPage /> }
            }
            #[cfg(not(feature = "verifier"))]
            {
                html! { <FeatureNotEnabled feature="verifier" /> }
            }
        }
        Route::VerifyProof => {
            log::info!("Rendering VerifyProofPage");
            log::info!("Verifier feature enabled: {}", cfg!(feature = "verifier"));
            #[cfg(feature = "verifier")]
            {
                log::info!("About to render VerifyProofPage component");
                html! { <VerifyProofPage /> }
            }
            #[cfg(not(feature = "verifier"))]
            {
                log::warn!("Verifier feature not enabled, showing FeatureNotEnabled");
                html! { <FeatureNotEnabled feature="verifier" /> }
            }
        }
        Route::NotFound => {
            log::info!("Rendering NotFoundPage");
            html! { <NotFoundPage /> }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct FeatureNotEnabledProps {
    pub feature: String,
}

#[function_component(FeatureNotEnabled)]
pub fn feature_not_enabled(props: &FeatureNotEnabledProps) -> Html {
    html! {
        <PageLayout>
            <div class="hero min-h-96">
                <div class="hero-content text-center">
                    <div class="max-w-md">
                        <div class="text-6xl mb-4">{"⚠️"}</div>
                        <h1 class="text-3xl font-bold">{"Feature Not Enabled"}</h1>
                        <p class="py-6">
                            {format!("The '{}' feature is not enabled in this build.", props.feature)}
                        </p>
                        <p class="text-sm text-base-content/70">
                            {"Enable the feature in Cargo.toml to access this functionality."}
                        </p>
                    </div>
                </div>
            </div>
        </PageLayout>
    }
}
