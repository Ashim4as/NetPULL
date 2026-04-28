mod app;
mod components;
mod models;
mod api;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| leptos::view! { <app::App /> });
}
