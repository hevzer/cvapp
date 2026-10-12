pub mod app;
pub mod components;
pub mod data;
pub mod date;
pub mod i18n;
pub mod models;
pub mod services;
pub mod state;
pub mod templates;

use app::App;
use leptos::mount::mount_to_body;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
