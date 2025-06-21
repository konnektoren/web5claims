mod components;
mod pages;
mod router;
mod services;
mod types;
mod utils;

use components::app::App;
use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() {
    // Initialize console logging
    console_log::init_with_level(log::Level::Info).expect("error initializing log");

    // Log that the app is starting
    log::info!("Starting Web5 Claims application");

    // Start the Yew app
    yew::Renderer::<App>::new().render();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_app_creation() {
        assert_eq!(2 + 2, 4);
    }
}
