use crate::components::ui::material_input::MaterialInput;
use crate::components::ui::material_textarea::MaterialTextarea;
use crate::state::AppState;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{FileReader, HtmlInputElement};

#[component]
pub fn PersonalInfoForm() -> impl IntoView {
    let state = expect_context::<AppState>();
    let resume = state.resume_data;

    let on_photo_change = move |ev: web_sys::Event| {
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                let reader = FileReader::new().unwrap();
                let reader_clone = reader.clone();
                let onload = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    if let Ok(res) = reader_clone.result() {
                        if let Some(data_url) = res.as_string() {
                            resume.update(|r| {
                                r.personal_info.photo_url = data_url;
                            });
                            state.save_to_storage();
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                onload.forget();
                let _ = reader.read_as_data_url(&file);
            }
        }
    };

    view! {
        <div class="space-y-3">
            {/* Photo Upload */}
            <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                    "Profile Photo"
                </label>
                <div class="flex items-center gap-4">
                    <label class="relative w-20 h-20 rounded-full overflow-hidden border-2 border-dashed border-gray-300 dark:border-gray-600 cursor-pointer group hover:border-cyan-400 dark:hover:border-cyan-500 transition-all duration-300 flex-shrink-0">
                        {move || {
                            let photo = resume.get().personal_info.photo_url;
                            if !photo.is_empty() {
                                view! {
                                    <img src=photo alt="Profile" class="w-full h-full object-cover" />
                                    <div class="absolute inset-0 bg-black/50 opacity-0 group-hover:opacity-100 transition-opacity duration-200 flex items-center justify-center">
                                        <span class="text-white text-xs font-medium">"Change"</span>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="w-full h-full bg-gray-100 dark:bg-gray-700 flex flex-col items-center justify-center gap-0.5 group-hover:bg-cyan-50 dark:group-hover:bg-cyan-950/30 transition-colors duration-200">
                                        <svg class="w-5 h-5 text-gray-400 group-hover:text-cyan-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
                                        </svg>
                                        <span class="text-[10px] text-gray-400 group-hover:text-cyan-500 transition-colors">"Photo"</span>
                                    </div>
                                }.into_any()
                            }
                        }}
                        <input
                            type="file"
                            accept="image/*"
                            on:change=on_photo_change
                            class="hidden"
                        />
                    </label>
                    <div class="flex flex-col gap-1.5">
                        {move || {
                            let has_photo = !resume.get().personal_info.photo_url.is_empty();
                            if has_photo {
                                view! {
                                    <button
                                        type="button"
                                        on:click=move |_| {
                                            resume.update(|r| r.personal_info.photo_url.clear());
                                            state.save_to_storage();
                                        }
                                        class="text-xs text-red-500 hover:text-red-600 dark:text-red-400 dark:hover:text-red-300 font-medium active:scale-95 transition-all duration-150 text-left"
                                    >
                                        "Remove"
                                    </button>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        }}
                        <span class="text-[10px] text-gray-400">"JPG, PNG. Max 5MB."</span>
                    </div>
                </div>
            </div>

            {/* Objective */}
            <div class="pt-2">
                <MaterialTextarea
                    label="Objective / Looking For"
                    value=Signal::derive(move || resume.get().personal_info.objective)
                    on_change=Callback::new(move |val| {
                        resume.update(|r| r.personal_info.objective = val);
                        state.save_to_storage();
                    })
                    rows=3
                />
            </div>

            {/* Text Fields */}
            <div class="space-y-4 pt-2">
                <MaterialInput
                    label="Full Name"
                    value=Signal::derive(move || resume.get().personal_info.full_name)
                    on_change=Callback::new(move |val| {
                        resume.update(|r| r.personal_info.full_name = val);
                        state.save_to_storage();
                    })
                />
                <MaterialInput
                    label="Professional Title"
                    value=Signal::derive(move || resume.get().personal_info.title)
                    on_change=Callback::new(move |val| {
                        resume.update(|r| r.personal_info.title = val);
                        state.save_to_storage();
                    })
                />
                <MaterialInput
                    label="Email"
                    input_type="email"
                    value=Signal::derive(move || resume.get().personal_info.email)
                    on_change=Callback::new(move |val| {
                        resume.update(|r| r.personal_info.email = val);
                        state.save_to_storage();
                    })
                />
                <MaterialInput
                    label="Phone"
                    input_type="tel"
                    value=Signal::derive(move || resume.get().personal_info.phone)
                    on_change=Callback::new(move |val| {
                        resume.update(|r| r.personal_info.phone = val);
                        state.save_to_storage();
                    })
                />
                <MaterialInput
                    label="Location"
                    value=Signal::derive(move || resume.get().personal_info.location)
                    on_change=Callback::new(move |val| {
                        resume.update(|r| r.personal_info.location = val);
                        state.save_to_storage();
                    })
                />
                <MaterialInput
                    label="LinkedIn"
                    value=Signal::derive(move || resume.get().personal_info.linkedin)
                    on_change=Callback::new(move |val| {
                        resume.update(|r| r.personal_info.linkedin = val);
                        state.save_to_storage();
                    })
                />
                <MaterialInput
                    label="GitHub"
                    value=Signal::derive(move || resume.get().personal_info.github)
                    on_change=Callback::new(move |val| {
                        resume.update(|r| r.personal_info.github = val);
                        state.save_to_storage();
                    })
                />
                <MaterialInput
                    label="Website"
                    value=Signal::derive(move || resume.get().personal_info.website)
                    on_change=Callback::new(move |val| {
                        resume.update(|r| r.personal_info.website = val);
                        state.save_to_storage();
                    })
                />
                <MaterialInput
                    label="Driving License"
                    value=Signal::derive(move || resume.get().personal_info.driving_license)
                    on_change=Callback::new(move |val| {
                        resume.update(|r| r.personal_info.driving_license = val);
                        state.save_to_storage();
                    })
                />
            </div>
        </div>
    }
}
