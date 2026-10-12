use crate::models::ResumeData;
use crate::state::AppState;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Blob, FileReader, HtmlAnchorElement, HtmlInputElement, Url};

#[component]
pub fn DataActions() -> impl IntoView {
    let state = expect_context::<AppState>();
    let resume = state.resume_data;

    let handle_export = move |_| {
        let data = resume.get();
        if let Ok(json_str) = serde_json::to_string_pretty(&data) {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    let parts = js_sys::Array::new();
                    parts.push(&wasm_bindgen::JsValue::from_str(&json_str));

                    let blob_props = web_sys::BlobPropertyBag::new();
                    blob_props.set_type("application/json");

                    if let Ok(blob) = Blob::new_with_str_sequence_and_options(&parts, &blob_props) {
                        if let Ok(url) = Url::create_object_url_with_blob(&blob) {
                            if let Ok(element) = document.create_element("a") {
                                let a: HtmlAnchorElement = element.unchecked_into();
                                a.set_href(&url);
                                a.set_download("cvapp-data.json");
                                let _ = a.click();
                                let _ = Url::revoke_object_url(&url);
                            }
                        }
                    }
                }
            }
        }
    };

    let on_file_input = move |ev: web_sys::Event| {
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                let reader = FileReader::new().unwrap();
                let reader_clone = reader.clone();
                let onload = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    if let Ok(res) = reader_clone.result() {
                        if let Some(content) = res.as_string() {
                            if let Ok(parsed) = serde_json::from_str::<ResumeData>(&content) {
                                resume.set(parsed);
                                state.save_to_storage();
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                onload.forget();
                let _ = reader.read_as_text(&file);
            }
        }
    };

    view! {
        <div class="grid grid-cols-2 gap-3 mb-5">
            <button
                type="button"
                on:click=handle_export
                class="group flex flex-col items-center justify-center gap-2 p-4 rounded-2xl bg-cyan-50 dark:bg-cyan-900/30 text-cyan-700 dark:text-cyan-300 hover:bg-cyan-100 hover:shadow-md dark:hover:bg-cyan-900/50 transition-all active:scale-[0.98] border border-transparent hover:border-cyan-200 dark:hover:border-cyan-500/30"
            >
                <svg class="w-6 h-6 text-cyan-600 dark:text-cyan-400 group-hover:-translate-y-0.5 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path>
                </svg>
                <span class="text-[11px] font-extrabold uppercase tracking-widest">"Save Data"</span>
            </button>

            <label
                class="group flex flex-col items-center justify-center gap-2 p-4 rounded-2xl bg-emerald-50 dark:bg-emerald-900/20 text-emerald-700 dark:text-emerald-400 hover:bg-emerald-100 hover:shadow-md dark:hover:bg-emerald-900/40 transition-all active:scale-[0.98] border border-transparent hover:border-emerald-200 dark:hover:border-emerald-500/30 cursor-pointer"
            >
                <input
                    type="file"
                    accept=".json"
                    on:change=on_file_input
                    class="hidden"
                />
                <svg class="w-6 h-6 text-emerald-600 dark:text-emerald-400 group-hover:-translate-y-0.5 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"></path>
                </svg>
                <span class="text-[11px] font-extrabold uppercase tracking-widest">"Load Data"</span>
            </label>
        </div>
    }
}
