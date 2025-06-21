use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    let navigator = use_navigator().unwrap();

    let go_home = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Home))
    };

    html! {
        <div class="min-h-screen bg-base-200 flex items-center justify-center">
            <div class="card bg-base-100 shadow-xl max-w-lg">
                <div class="card-body text-center">
                    <div class="text-6xl mb-4">{"🔍"}</div>
                    <h1 class="card-title justify-center text-3xl mb-2">
                        {"404 - Page Not Found"}
                    </h1>
                    <p class="mb-4">
                        {"The page you're looking for doesn't exist in the Web5 Claims application."}
                    </p>
                    <div class="card-actions justify-center">
                        <button
                            class="btn btn-primary"
                            onclick={go_home}
                        >
                            {"🏠 Go Home"}
                        </button>
                        <a
                            href="https://github.com/your-username/web5claims"
                            class="btn btn-outline"
                            target="_blank"
                        >
                            {"📚 View on GitHub"}
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}
