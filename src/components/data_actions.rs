use crate::models::ResumeData;
use crate::services::file_sync::{
    is_direct_file_sync_supported, link_new_file, open_and_link_file,
};
use crate::state::{AppState, SyncStatus};
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{Blob, FileReader, HtmlAnchorElement, HtmlInputElement, Url};

#[component]
pub fn DataActions() -> impl IntoView {
    let state = expect_context::<AppState>();
    let resume = state.resume_data;

    let handle_export = move |_| {
        let data = resume.get_untracked();
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
                                state.schedule_save();
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

    let handle_link_file = move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            match link_new_file("cvapp-data.json").await {
                Ok(file) => {
                    let name = file.name().to_string();
                    state.linked_file.set(Some(file));
                    state.sync_status.set(SyncStatus::Synced(name));
                    state.save_all();
                }
                Err(e) => {
                    if !e.contains("cancelled") {
                        state.sync_status.set(SyncStatus::Error(e));
                    }
                }
            }
        });
    };

    let handle_open_and_link = move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            match open_and_link_file().await {
                Ok((file, content)) => {
                    if let Ok(parsed) = serde_json::from_str::<ResumeData>(&content) {
                        let name = file.name().to_string();
                        resume.set(parsed);
                        state.linked_file.set(Some(file));
                        state.sync_status.set(SyncStatus::Synced(name));
                        state.schedule_save();
                    } else {
                        state
                            .sync_status
                            .set(SyncStatus::Error("Invalid JSON file".to_string()));
                    }
                }
                Err(e) => {
                    if !e.contains("cancelled") {
                        state.sync_status.set(SyncStatus::Error(e));
                    }
                }
            }
        });
    };

    let handle_unlink = move |_| {
        state.linked_file.set(None);
        state.sync_status.set(SyncStatus::LocalCache);
    };

    let direct_supported = is_direct_file_sync_supported();

    view! {
        <div class="flex flex-col gap-3 mb-5">
            {/* Live Sync Status Pill */}
            <div class="flex items-center justify-between px-3.5 py-2.5 rounded-xl border text-xs transition-all duration-200">
                {move || match state.sync_status.get() {
                    SyncStatus::LocalCache => view! {
                        <div class="flex items-center gap-2 text-slate-600 dark:text-slate-400">
                            <span class="w-2 h-2 rounded-full bg-slate-400"></span>
                            <span>"Saving live to local cache"</span>
                        </div>
                        <span class="text-[10px] text-slate-400 dark:text-slate-500 uppercase tracking-wider font-semibold">
                            "Unlinked"
                        </span>
                    }.into_any(),
                    SyncStatus::Saving => view! {
                        <div class="flex items-center gap-2 text-amber-700 dark:text-amber-300">
                            <span class="w-2 h-2 rounded-full bg-amber-500 animate-pulse"></span>
                            <span class="font-medium">"Syncing to file..."</span>
                        </div>
                        <span class="text-[10px] text-amber-500 uppercase tracking-wider font-bold animate-pulse">
                            "Writing"
                        </span>
                    }.into_any(),
                    SyncStatus::Synced(name) => view! {
                        <div class="flex items-center gap-2 text-emerald-700 dark:text-emerald-400 font-medium">
                            <span class="w-2 h-2 rounded-full bg-emerald-500"></span>
                            <span class="truncate max-w-[170px]">"Synced: " {name}</span>
                        </div>
                        <button
                            type="button"
                            on:click=handle_unlink
                            title="Unlink file and return to local storage"
                            class="px-2 py-0.5 text-[10px] font-bold text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-950/40 rounded transition-colors"
                        >
                            "Unlink ✕"
                        </button>
                    }.into_any(),
                    SyncStatus::Error(err) => {
                        let err_msg = err.clone();
                        view! {
                            <div class="flex items-center gap-2 text-red-600 dark:text-red-400">
                                <span class="w-2 h-2 rounded-full bg-red-500"></span>
                                <span class="truncate max-w-[190px]" title=err_msg>{err}</span>
                            </div>
                            <button
                                type="button"
                                on:click=handle_unlink
                                class="text-[10px] font-bold text-slate-500 hover:text-slate-700"
                            >
                                "Dismiss"
                            </button>
                        }.into_any()
                    }
                }}
            </div>

            {/* Direct File Sync Actions (Project File Workflow) */}
            {if direct_supported {
                view! {
                    <div class="grid grid-cols-2 gap-3">
                        <button
                            type="button"
                            on:click=handle_link_file
                            class="group flex flex-col items-center justify-center gap-2 p-3.5 rounded-2xl bg-indigo-50 dark:bg-indigo-900/30 text-indigo-700 dark:text-indigo-300 hover:bg-indigo-100 hover:shadow-md dark:hover:bg-indigo-900/50 transition-all active:scale-[0.98] border border-transparent hover:border-indigo-200 dark:hover:border-indigo-500/30 cursor-pointer"
                        >
                            <svg class="w-5 h-5 text-indigo-600 dark:text-indigo-400 group-hover:-translate-y-0.5 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4"></path>
                            </svg>
                            <span class="text-[10.5px] font-extrabold uppercase tracking-widest text-center">"Link JSON File"</span>
                        </button>

                        <button
                            type="button"
                            on:click=handle_open_and_link
                            class="group flex flex-col items-center justify-center gap-2 p-3.5 rounded-2xl bg-violet-50 dark:bg-violet-900/30 text-violet-700 dark:text-violet-300 hover:bg-violet-100 hover:shadow-md dark:hover:bg-violet-900/50 transition-all active:scale-[0.98] border border-transparent hover:border-violet-200 dark:hover:border-violet-500/30 cursor-pointer"
                        >
                            <svg class="w-5 h-5 text-violet-600 dark:text-violet-400 group-hover:-translate-y-0.5 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"></path>
                            </svg>
                            <span class="text-[10.5px] font-extrabold uppercase tracking-widest text-center">"Open & Link File"</span>
                        </button>
                    </div>
                }.into_any()
            } else {
                view! {
                    <div class="text-[11px] text-center text-slate-500 dark:text-slate-400 py-1">
                        "Browser cache auto-save active. (Use Chrome, Edge, or Desktop App for direct file linking)"
                    </div>
                }.into_any()
            }}

            {/* Manual Export / Import */}
            <div class="grid grid-cols-2 gap-3">
                <button
                    type="button"
                    on:click=handle_export
                    class="group flex flex-col items-center justify-center gap-2 p-3.5 rounded-2xl bg-cyan-50 dark:bg-cyan-900/30 text-cyan-700 dark:text-cyan-300 hover:bg-cyan-100 hover:shadow-md dark:hover:bg-cyan-900/50 transition-all active:scale-[0.98] border border-transparent hover:border-cyan-200 dark:hover:border-cyan-500/30 cursor-pointer"
                >
                    <svg class="w-5 h-5 text-cyan-600 dark:text-cyan-400 group-hover:-translate-y-0.5 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path>
                    </svg>
                    <span class="text-[10.5px] font-extrabold uppercase tracking-widest">"Save Data"</span>
                </button>

                <label
                    class="group flex flex-col items-center justify-center gap-2 p-3.5 rounded-2xl bg-emerald-50 dark:bg-emerald-900/20 text-emerald-700 dark:text-emerald-400 hover:bg-emerald-100 hover:shadow-md dark:hover:bg-emerald-900/40 transition-all active:scale-[0.98] border border-transparent hover:border-emerald-200 dark:hover:border-emerald-500/30 cursor-pointer"
                >
                    <input
                        type="file"
                        accept=".json"
                        on:change=on_file_input
                        class="hidden"
                    />
                    <svg class="w-5 h-5 text-emerald-600 dark:text-emerald-400 group-hover:-translate-y-0.5 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"></path>
                    </svg>
                    <span class="text-[10.5px] font-extrabold uppercase tracking-widest">"Load Data"</span>
                </label>
            </div>
        </div>
    }
}
