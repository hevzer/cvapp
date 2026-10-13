use crate::components::about_modal::AboutModal;
use crate::components::home::project_card::ProjectCard;
use crate::components::linkedin_modal::LinkedInModal;
use crate::components::ui::dark_mode_toggle::DarkModeToggle;
use crate::models::{CvProject, PersistedState, ResumeData};
use crate::state::AppState;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{FileReader, HtmlInputElement};

#[component]
pub fn ProjectsHome() -> impl IntoView {
    let state = expect_context::<AppState>();
    let search_query = RwSignal::new(String::new());

    // Rename state
    let rename_target_id = RwSignal::new(Option::<String>::None);
    let rename_input_value = RwSignal::new(String::new());

    // Delete confirmation state
    let delete_target_id = RwSignal::new(Option::<String>::None);

    // LinkedIn modal state
    let show_linkedin = RwSignal::new(false);

    // Open project
    let on_open = move |id: String| {
        state.open_project(&id);
    };

    // Duplicate project
    let on_duplicate = move |id: String| {
        let _ = state.duplicate_project(&id);
    };

    // Start rename
    let on_rename = move |id: String| {
        let current_name = state.projects.with(|projs| {
            projs
                .iter()
                .find(|p| p.id == id)
                .map(|p| p.name.clone())
                .unwrap_or_default()
        });
        rename_input_value.set(current_name);
        rename_target_id.set(Some(id));
    };

    // Start delete
    let on_delete = move |id: String| {
        delete_target_id.set(Some(id));
    };

    // Confirm rename
    let submit_rename = move |_| {
        if let Some(id) = rename_target_id.get() {
            let new_name = rename_input_value.get();
            state.rename_project(&id, &new_name);
            rename_target_id.set(None);
        }
    };

    // Confirm delete
    let submit_delete = move |_| {
        if let Some(id) = delete_target_id.get() {
            state.delete_project(&id);
            delete_target_id.set(None);
        }
    };

    // Import JSON file as a new project
    let on_json_import = move |ev: web_sys::Event| {
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                let filename = file.name();
                let project_name = filename
                    .strip_suffix(".json")
                    .unwrap_or(&filename)
                    .replace(['-', '_'], " ")
                    .trim()
                    .to_string();

                let reader = FileReader::new().unwrap();
                let reader_clone = reader.clone();
                let onload = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    if let Ok(res) = reader_clone.result() {
                        if let Some(content) = res.as_string() {
                            // Try parsing as full CvProject, PersistedState, or ResumeData
                            if let Ok(full_proj) = serde_json::from_str::<CvProject>(&content) {
                                state.projects.update(|p| p.insert(0, full_proj.clone()));
                                state.open_project(&full_proj.id);
                            } else if let Ok(state_data) = serde_json::from_str::<PersistedState>(&content) {
                                state.create_new_project(&project_name, Some(state_data));
                            } else if let Ok(resume_data) = serde_json::from_str::<ResumeData>(&content) {
                                let mut s = PersistedState::default();
                                s.resume_data = resume_data;
                                state.create_new_project(&project_name, Some(s));
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

    let filtered_projects = move || {
        let q = search_query.get().to_lowercase().trim().to_string();
        state.projects.with(|projs| {
            if q.is_empty() {
                projs.clone()
            } else {
                projs
                    .iter()
                    .filter(|p| {
                        p.name.to_lowercase().contains(&q)
                            || p.candidate_name().to_lowercase().contains(&q)
                            || p.target_role().to_lowercase().contains(&q)
                            || p.template_display_name().to_lowercase().contains(&q)
                    })
                    .cloned()
                    .collect()
            }
        })
    };

    let active_id = move || state.active_project_id.get();
    let has_active = move || active_id().is_some();

    view! {
        <div class="min-h-screen bg-slate-50 dark:bg-black text-slate-900 dark:text-white flex flex-col">
            {/* Top Navigation Bar */}
            <header class="sticky top-0 z-30 bg-white/80 dark:bg-[#0a0a0a]/80 backdrop-blur-md border-b border-slate-200/80 dark:border-white/[0.06] px-6 py-4">
                <div class="max-w-7xl mx-auto flex items-center justify-between">
                    {/* Brand */}
                    <div class="flex items-center gap-3.5 select-none">
                        <div class="w-10 h-10 flex items-center justify-center">
                            <img src="/cvapp_macos_logo.png" alt="CVapp Logo" class="w-full h-full object-contain drop-shadow-md" />
                        </div>
                        <div>
                            <h1 class="text-lg font-black tracking-tight text-slate-900 dark:text-white leading-none">
                                "CVapp"
                            </h1>
                            <p class="text-[11px] font-semibold text-slate-400 dark:text-slate-500 uppercase tracking-widest mt-0.5">
                                "Projects Dashboard"
                            </p>
                        </div>
                    </div>

                    {/* Right Actions */}
                    <div class="flex items-center gap-2">
                        {move || if has_active() {
                            view! {
                                <button
                                    type="button"
                                    on:click=move |_| state.is_home_view.set(false)
                                    class="flex items-center gap-2 px-4 py-2 rounded-full text-xs font-bold text-indigo-700 bg-indigo-50 dark:text-indigo-300 dark:bg-indigo-950/50 hover:bg-indigo-100 dark:hover:bg-indigo-900/60 border border-indigo-200 dark:border-indigo-800/40 transition-all cursor-pointer"
                                >
                                    <span>"← Back to Editor"</span>
                                </button>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}

                        <AboutModal />
                        <DarkModeToggle />
                    </div>
                </div>
            </header>

            {/* Main Content Area */}
            <main class="flex-1 max-w-7xl w-full mx-auto px-6 py-8">
                {/* Hero / Action Section */}
                <div class="flex flex-col md:flex-row md:items-center justify-between gap-6 pb-8 border-b border-slate-200/80 dark:border-white/[0.06]">
                    <div>
                        <h2 class="text-2xl sm:text-3xl font-extrabold tracking-tight text-slate-900 dark:text-white">
                            "My Resumes"
                        </h2>
                        <p class="text-sm text-slate-500 dark:text-slate-400 mt-1">
                            "Create, manage, and tailor multiple professional resumes in one place."
                        </p>
                    </div>

                    {/* Action Buttons */}
                    <div class="flex flex-wrap items-center gap-2.5">
                        {/* New Blank Resume */}
                        <button
                            type="button"
                            on:click=move |_| {
                                state.create_new_project("New Resume", None);
                            }
                            class="flex items-center gap-2 px-4.5 py-2.5 rounded-full text-xs font-bold uppercase tracking-wider text-white bg-indigo-600 hover:bg-indigo-700 shadow-md shadow-indigo-500/20 hover:shadow-lg hover:shadow-indigo-500/30 active:scale-95 transition-all cursor-pointer"
                        >
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M12 4v16m8-8H4"></path>
                            </svg>
                            "New Resume"
                        </button>

                        {/* New from Example */}
                        <button
                            type="button"
                            on:click=move |_| {
                                state.create_example_project();
                            }
                            class="flex items-center gap-2 px-4 py-2.5 rounded-full text-xs font-bold uppercase tracking-wider text-cyan-700 bg-cyan-100 dark:text-cyan-300 dark:bg-cyan-950/60 hover:bg-cyan-200 dark:hover:bg-cyan-900/60 active:scale-95 transition-all cursor-pointer"
                        >
                            <span>"✨ Example"</span>
                        </button>

                        {/* Import JSON */}
                        <label
                            class="flex items-center gap-2 px-4 py-2.5 rounded-full text-xs font-bold uppercase tracking-wider text-slate-700 dark:text-slate-300 bg-white dark:bg-[#1a1a1a] hover:bg-slate-100 dark:hover:bg-white/[0.08] border border-slate-200 dark:border-white/[0.1] active:scale-95 transition-all cursor-pointer"
                        >
                            <input
                                type="file"
                                accept=".json"
                                on:change=on_json_import
                                class="hidden"
                            />
                            <svg class="w-4 h-4 text-slate-500 dark:text-slate-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"></path>
                            </svg>
                            "Import JSON"
                        </label>

                        {/* Import LinkedIn */}
                        <button
                            type="button"
                            on:click=move |_| show_linkedin.set(true)
                            class="flex items-center gap-2 px-4 py-2.5 rounded-full text-xs font-bold uppercase tracking-wider text-[#0a66c2] dark:text-[#70b5f9] bg-blue-50 dark:bg-blue-950/40 hover:bg-blue-100 dark:hover:bg-blue-900/50 border border-blue-200 dark:border-blue-800/40 active:scale-95 transition-all cursor-pointer"
                        >
                            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z"/>
                            </svg>
                            "LinkedIn"
                        </button>
                    </div>
                </div>

                {/* Search Bar & Project Count */}
                <div class="mt-6 flex flex-col sm:flex-row items-center justify-between gap-4">
                    <div class="relative w-full sm:w-80">
                        <svg class="w-4 h-4 absolute left-3.5 top-1/2 -translate-y-1/2 text-slate-400 pointer-events-none" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                        </svg>
                        <input
                            type="text"
                            placeholder="Search resumes..."
                            prop:value=search_query
                            on:input=move |ev| search_query.set(event_target_value(&ev))
                            class="w-full pl-10 pr-4 py-2 rounded-xl text-xs bg-white dark:bg-[#141414] border border-slate-200 dark:border-white/[0.08] focus:outline-none focus:ring-2 focus:ring-indigo-500/30 focus:border-indigo-500 transition-all placeholder:text-slate-400 dark:placeholder:text-slate-600"
                        />
                    </div>

                    <span class="text-xs font-semibold text-slate-400 dark:text-slate-500">
                        {move || {
                            let count = filtered_projects().len();
                            format!("{} resume{}", count, if count == 1 { "" } else { "s" })
                        }}
                    </span>
                </div>

                {/* Projects Grid */}
                <div class="mt-6">
                    {move || {
                        let projs = filtered_projects();
                        if projs.is_empty() {
                            view! {
                                <div class="flex flex-col items-center justify-center py-16 text-center bg-white dark:bg-[#121212] rounded-3xl border border-dashed border-slate-200 dark:border-white/[0.08] p-8">
                                    <div class="w-14 h-14 rounded-full bg-indigo-50 dark:bg-indigo-950/40 text-indigo-600 dark:text-indigo-400 flex items-center justify-center mb-4">
                                        <svg class="w-7 h-7" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                                        </svg>
                                    </div>
                                    <h3 class="text-lg font-bold text-slate-800 dark:text-slate-200">
                                        "No resumes found"
                                    </h3>
                                    <p class="text-xs text-slate-500 dark:text-slate-400 max-w-sm mt-1 mb-5">
                                        "Get started by creating a new blank resume or import an existing JSON or LinkedIn file."
                                    </p>
                                    <button
                                        type="button"
                                        on:click=move |_| {
                                            state.create_new_project("New Resume", None);
                                        }
                                        class="px-5 py-2.5 rounded-full text-xs font-bold uppercase tracking-wider text-white bg-indigo-600 hover:bg-indigo-700 transition-all cursor-pointer"
                                    >
                                        "+ Create Your First Resume"
                                    </button>
                                </div>
                            }.into_any()
                        } else {
                            let current_active = active_id();
                            view! {
                                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-5">
                                    {projs.into_iter().map(|proj| {
                                        let is_active = current_active.as_ref() == Some(&proj.id);
                                        view! {
                                            <ProjectCard
                                                project=proj
                                                is_active=is_active
                                                on_open=on_open
                                                on_duplicate=on_duplicate
                                                on_rename=on_rename
                                                on_delete=on_delete
                                            />
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    }}
                </div>
            </main>

            {/* Rename Modal Dialog */}
            {move || {
                if rename_target_id.get().is_some() {
                    view! {
                        <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm animate-fade-in">
                            <div class="bg-white dark:bg-[#141414] rounded-2xl p-6 w-full max-w-md shadow-2xl border border-slate-200 dark:border-white/[0.08] animate-scale-up">
                                <h3 class="text-lg font-bold text-slate-900 dark:text-white mb-2">
                                    "Rename Resume"
                                </h3>
                                <p class="text-xs text-slate-500 dark:text-slate-400 mb-4">
                                    "Give your resume a distinctive title to organize your applications."
                                </p>
                                <input
                                    type="text"
                                    prop:value=rename_input_value
                                    on:input=move |ev| rename_input_value.set(event_target_value(&ev))
                                    on:keydown=move |ev: web_sys::KeyboardEvent| {
                                        if ev.key() == "Enter" {
                                            submit_rename(());
                                        } else if ev.key() == "Escape" {
                                            rename_target_id.set(None);
                                        }
                                    }
                                    class="w-full px-4 py-2.5 rounded-xl text-sm bg-slate-50 dark:bg-black border border-slate-200 dark:border-white/[0.1] focus:outline-none focus:ring-2 focus:ring-indigo-500/40 text-slate-900 dark:text-white mb-5"
                                    autofocus=true
                                />
                                <div class="flex items-center justify-end gap-2.5">
                                    <button
                                        type="button"
                                        on:click=move |_| rename_target_id.set(None)
                                        class="px-4 py-2 rounded-xl text-xs font-semibold text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-white/[0.05] transition-colors cursor-pointer"
                                    >
                                        "Cancel"
                                    </button>
                                    <button
                                        type="button"
                                        on:click=move |_| submit_rename(())
                                        class="px-4.5 py-2 rounded-xl text-xs font-bold text-white bg-indigo-600 hover:bg-indigo-700 transition-colors cursor-pointer"
                                    >
                                        "Save Name"
                                    </button>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            {/* Delete Confirmation Modal Dialog */}
            {move || {
                if let Some(target_id) = delete_target_id.get() {
                    let target_name = state.projects.with(|projs| {
                        projs
                            .iter()
                            .find(|p| p.id == target_id)
                            .map(|p| p.name.clone())
                            .unwrap_or_else(|| "this resume".to_string())
                    });

                    view! {
                        <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm animate-fade-in">
                            <div class="bg-white dark:bg-[#141414] rounded-2xl p-6 w-full max-w-md shadow-2xl border border-slate-200 dark:border-white/[0.08] animate-scale-up">
                                <div class="w-12 h-12 rounded-full bg-red-100 dark:bg-red-950/60 text-red-600 dark:text-red-400 flex items-center justify-center mb-4">
                                    <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                                    </svg>
                                </div>
                                <h3 class="text-lg font-bold text-slate-900 dark:text-white mb-1.5">
                                    "Delete Resume?"
                                </h3>
                                <p class="text-xs text-slate-500 dark:text-slate-400 mb-6 leading-relaxed">
                                    "Are you sure you want to delete \"" <strong class="text-slate-800 dark:text-slate-200">{target_name}</strong> "\"? This action cannot be undone."
                                </p>
                                <div class="flex items-center justify-end gap-2.5">
                                    <button
                                        type="button"
                                        on:click=move |_| delete_target_id.set(None)
                                        class="px-4 py-2 rounded-xl text-xs font-semibold text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-white/[0.05] transition-colors cursor-pointer"
                                    >
                                        "Cancel"
                                    </button>
                                    <button
                                        type="button"
                                        on:click=move |_| submit_delete(())
                                        class="px-4.5 py-2 rounded-xl text-xs font-bold text-white bg-red-600 hover:bg-red-700 transition-colors cursor-pointer shadow-sm shadow-red-600/30"
                                    >
                                        "Delete"
                                    </button>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            {/* LinkedIn Import Modal */}
            {move || if show_linkedin.get() {
                view! {
                    <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm animate-fade-in">
                        <div class="relative w-full max-w-2xl bg-white dark:bg-[#141414] rounded-3xl p-6 shadow-2xl border border-slate-200 dark:border-white/[0.08] max-h-[90vh] overflow-y-auto">
                            <button
                                type="button"
                                on:click=move |_| show_linkedin.set(false)
                                class="absolute top-5 right-5 p-2 text-slate-400 hover:text-slate-600 dark:hover:text-slate-200 rounded-full"
                            >
                                "✕"
                            </button>
                            <LinkedInModal />
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
        </div>
    }
}
