use crate::date::format_relative_time;
use crate::models::{current_timestamp_secs, CvProject};
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Blob, HtmlAnchorElement, Url};

#[component]
pub fn ProjectCard<F1, F2, F3, F4>(
    project: CvProject,
    is_active: bool,
    on_open: F1,
    on_duplicate: F2,
    on_rename: F3,
    on_delete: F4,
) -> impl IntoView
where
    F1: Fn(String) + 'static + Copy,
    F2: Fn(String) + 'static + Copy,
    F3: Fn(String) + 'static + Copy,
    F4: Fn(String) + 'static + Copy,
{
    let id_for_open = project.id.clone();
    let id_for_dup = project.id.clone();
    let id_for_rename = project.id.clone();
    let id_for_delete = project.id.clone();

    let now = current_timestamp_secs();
    let updated_str = format_relative_time(project.updated_at, now);
    let accent_color = project.data.accent_color.clone();
    let template_name = project.template_display_name();
    let candidate_name = project.candidate_name().to_string();
    let target_role = project.target_role().to_string();
    let project_name = project.name.clone();

    // Export handler
    let project_data = project.clone();
    let handle_export = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
        if let Ok(json_str) = serde_json::to_string_pretty(&project_data.data.resume_data) {
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
                                let filename = format!(
                                    "{}.json",
                                    project_data
                                        .name
                                        .to_lowercase()
                                        .replace(' ', "-")
                                        .replace('/', "-")
                                );
                                a.set_download(&filename);
                                let _ = a.click();
                                let _ = Url::revoke_object_url(&url);
                            }
                        }
                    }
                }
            }
        }
    };

    view! {
        <div
            on:click=move |_| on_open(id_for_open.clone())
            class=format!(
                "group relative flex flex-col justify-between bg-white dark:bg-[#121212] rounded-2xl border transition-all duration-300 hover:shadow-xl hover:-translate-y-1 cursor-pointer overflow-hidden {}",
                if is_active {
                    "border-indigo-500/80 ring-2 ring-indigo-500/20 shadow-md"
                } else {
                    "border-slate-200/80 dark:border-white/[0.08] hover:border-indigo-400/60 dark:hover:border-indigo-500/40"
                }
            )
        >
            {/* Top Color Accent Strip */}
            <div
                class="h-2 w-full transition-all duration-300 group-hover:h-2.5"
                style=format!("background-color: {}", accent_color)
            ></div>

            {/* Card Body */}
            <div class="p-5 flex-1 flex flex-col justify-between">
                <div>
                    {/* Header Row: Template Badge & Active Pill */}
                    <div class="flex items-center justify-between gap-2 mb-3">
                        <span class="inline-flex items-center px-2.5 py-1 rounded-full text-[10px] font-extrabold uppercase tracking-wider bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300">
                            {template_name}
                        </span>

                        {if is_active {
                            view! {
                                <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-[10px] font-bold bg-emerald-100 dark:bg-emerald-950/60 text-emerald-700 dark:text-emerald-400">
                                    <span class="w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
                                    "Active"
                                </span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>

                    {/* Project Title */}
                    <h3 class="text-base font-bold text-slate-900 dark:text-white group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors line-clamp-1">
                        {project_name}
                    </h3>

                    {/* Candidate Name & Role */}
                    <div class="mt-2 space-y-0.5 text-xs text-slate-500 dark:text-slate-400">
                        <p class="font-medium text-slate-700 dark:text-slate-300 truncate">
                            {candidate_name}
                        </p>
                        <p class="text-[11px] truncate text-slate-400 dark:text-slate-500">
                            {target_role}
                        </p>
                    </div>
                </div>

                {/* Footer Row: Timestamp & Quick Action Buttons */}
                <div class="mt-5 pt-3.5 border-t border-slate-100 dark:border-white/[0.06] flex items-center justify-between">
                    <span class="text-[11px] text-slate-400 dark:text-slate-500 font-medium">
                        "Updated " {updated_str}
                    </span>

                    {/* Action Buttons */}
                    <div class="flex items-center gap-1" on:click=move |e| e.stop_propagation()>
                        <button
                            type="button"
                            on:click=move |_| on_duplicate(id_for_dup.clone())
                            title="Duplicate this resume"
                            class="p-1.5 text-slate-400 hover:text-indigo-600 dark:hover:text-indigo-400 hover:bg-slate-100 dark:hover:bg-white/[0.05] rounded-lg transition-colors cursor-pointer"
                        >
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                            </svg>
                        </button>

                        <button
                            type="button"
                            on:click=handle_export
                            title="Export to JSON"
                            class="p-1.5 text-slate-400 hover:text-cyan-600 dark:hover:text-cyan-400 hover:bg-slate-100 dark:hover:bg-white/[0.05] rounded-lg transition-colors cursor-pointer"
                        >
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path>
                            </svg>
                        </button>

                        <button
                            type="button"
                            on:click=move |_| on_rename(id_for_rename.clone())
                            title="Rename"
                            class="p-1.5 text-slate-400 hover:text-amber-600 dark:hover:text-amber-400 hover:bg-slate-100 dark:hover:bg-white/[0.05] rounded-lg transition-colors cursor-pointer"
                        >
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path>
                            </svg>
                        </button>

                        <button
                            type="button"
                            on:click=move |_| on_delete(id_for_delete.clone())
                            title="Delete resume"
                            class="p-1.5 text-slate-400 hover:text-red-600 dark:hover:text-red-400 hover:bg-red-50 dark:hover:bg-red-950/40 rounded-lg transition-colors cursor-pointer"
                        >
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
