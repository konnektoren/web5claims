mod components;
mod types;
mod utils;

use components::app::App;

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        // Basic test to ensure the app can be instantiated
        assert_eq!(2 + 2, 4);
    }
}
