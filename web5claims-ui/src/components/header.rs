use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class="navbar bg-primary text-primary-content shadow-lg">
            <div class="container mx-auto">
                <div class="navbar-start">
                    <div class="flex items-center space-x-3">
                        <div class="text-3xl">{"🔐"}</div>
                        <div>
                            <h1 class="text-xl font-bold">{"Web5 Claims"}</h1>
                            <p class="text-sm opacity-80">{"ZK Language Learning Certificates"}</p>
                        </div>
                    </div>
                </div>

                <div class="navbar-end">
                    <div class="badge badge-accent">{"ZK Hack Berlin"}</div>
                </div>
            </div>
        </div>
    }
}
