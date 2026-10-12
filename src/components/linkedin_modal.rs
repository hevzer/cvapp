use crate::services::linkedin::{parse_linkedin_json, parse_linkedin_zip};
use crate::state::AppState;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{FileReader, HtmlInputElement};

#[component]
pub fn LinkedInModal() -> impl IntoView {
    let state = expect_context::<AppState>();
    let resume = state.resume_data;
    let (show_guide, set_show_guide) = signal(false);

    let on_file_input = move |ev: web_sys::Event| {
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                let name = file.name();
                let is_zip = name.ends_with(".zip");
                let is_json = name.ends_with(".json");

                let reader = FileReader::new().unwrap();
                let reader_clone = reader.clone();

                let onload = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    if is_zip {
                        if let Ok(res) = reader_clone.result() {
                            let array_buf = js_sys::Uint8Array::new(&res);
                            let bytes = array_buf.to_vec();
                            if let Ok(update) = parse_linkedin_zip(&bytes) {
                                resume.update(|r| update.apply_to(r));
                                state.save_to_storage();
                            }
                        }
                    } else if is_json {
                        if let Ok(res) = reader_clone.result() {
                            if let Some(text) = res.as_string() {
                                if let Ok(update) = parse_linkedin_json(&text) {
                                    resume.update(|r| update.apply_to(r));
                                    state.save_to_storage();
                                }
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                onload.forget();

                if is_zip {
                    let _ = reader.read_as_array_buffer(&file);
                } else if is_json {
                    let _ = reader.read_as_text(&file);
                }
            }
        }
    };

    view! {
        <div class="space-y-2">
            <label class="w-full flex items-center justify-center gap-2 px-5 py-3.5 bg-blue-50 dark:bg-[#0A66C2]/10 hover:bg-blue-100 dark:hover:bg-[#0A66C2]/20 text-[#0A66C2] dark:text-blue-300 text-[12px] font-bold uppercase tracking-widest rounded-full active:scale-[0.98] transition-all duration-300 shadow-sm border border-[#0A66C2]/20 dark:border-[#0A66C2]/30 cursor-pointer">
                <input
                    type="file"
                    accept=".zip,.json"
                    on:change=on_file_input
                    class="hidden"
                />
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                    <path d="M19 0H5a5 5 0 00-5 5v14a5 5 0 005 5h14a5 5 0 005-5V5a5 5 0 00-5-5zM8 19H5v-9h3v9zM6.5 8.25A1.75 1.75 0 118.25 6.5 1.75 1.75 0 016.5 8.25zM20 19h-3v-4.74c0-1.42-.6-1.93-1.38-1.93A1.74 1.74 0 0014 14.19V19h-3v-9h2.9v1.3a3.11 3.11 0 012.7-1.4c1.55 0 3.36.86 3.36 3.66V19z"></path>
                </svg>
                "LinkedIn Import"
            </label>

            <button
                type="button"
                on:click=move |_| set_show_guide.update(|g| *g = !*g)
                class="w-full text-xs text-gray-500 dark:text-gray-400 hover:text-cyan-600 dark:hover:text-cyan-400 font-medium transition-colors flex items-center justify-center gap-1"
            >
                <svg
                    class=move || {
                        if show_guide.get() {
                            "w-3.5 h-3.5 transition-transform duration-300 rotate-180"
                        } else {
                            "w-3.5 h-3.5 transition-transform duration-300"
                        }
                    }
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                </svg>
                {move || if show_guide.get() { "Hide export guide" } else { "How to export your LinkedIn data?" }}
            </button>

            {move || {
                if show_guide.get() {
                    view! {
                        <div class="p-4 bg-white/60 dark:bg-gray-800/60 backdrop-blur-lg border border-gray-200/50 dark:border-gray-700/50 rounded-xl text-xs space-y-3">
                            <h4 class="font-bold text-gray-800 dark:text-gray-100 text-sm flex items-center gap-2">
                                <span class="w-6 h-6 bg-[#0A66C2] rounded-md flex items-center justify-center text-white text-[10px] font-bold">"in"</span>
                                "Export your LinkedIn data"
                            </h4>
                            <ol class="space-y-2.5 text-gray-600 dark:text-gray-400 leading-relaxed">
                                <li class="flex gap-2.5">
                                    <span class="flex-shrink-0 w-5 h-5 bg-cyan-100 dark:bg-cyan-900/40 text-cyan-600 dark:text-cyan-400 rounded-full flex items-center justify-center text-[10px] font-bold mt-0.5">"1"</span>
                                    <span>"Go to "<strong class="text-gray-800 dark:text-gray-200">"LinkedIn.com"</strong>" → Settings & Privacy"</span>
                                </li>
                                <li class="flex gap-2.5">
                                    <span class="flex-shrink-0 w-5 h-5 bg-cyan-100 dark:bg-cyan-900/40 text-cyan-600 dark:text-cyan-400 rounded-full flex items-center justify-center text-[10px] font-bold mt-0.5">"2"</span>
                                    <span>"Under "<strong class="text-gray-800 dark:text-gray-200">"Data Privacy"</strong>", click 'Get a copy of your data'"</span>
                                </li>
                                <li class="flex gap-2.5">
                                    <span class="flex-shrink-0 w-5 h-5 bg-cyan-100 dark:bg-cyan-900/40 text-cyan-600 dark:text-cyan-400 rounded-full flex items-center justify-center text-[10px] font-bold mt-0.5">"3"</span>
                                    <span>"Select 'Want something in particular?' and choose Profile, Positions, Education, Skills"</span>
                                </li>
                                <li class="flex gap-2.5">
                                    <span class="flex-shrink-0 w-5 h-5 bg-cyan-100 dark:bg-cyan-900/40 text-cyan-600 dark:text-cyan-400 rounded-full flex items-center justify-center text-[10px] font-bold mt-0.5">"4"</span>
                                    <span>"Download the .zip archive and upload it above."</span>
                                </li>
                            </ol>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}
        </div>
    }
}
