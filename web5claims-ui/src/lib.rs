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
