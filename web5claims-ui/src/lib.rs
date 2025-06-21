mod components;
mod services;
mod types;
mod utils;

use components::app::App;

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_app_creation() {
        assert_eq!(2 + 2, 4);
    }
}
