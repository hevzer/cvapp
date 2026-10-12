use crate::components::ui::material_input::MaterialInput;
use crate::components::ui::material_textarea::MaterialTextarea;
use crate::models::{generate_id, Education};
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn EducationForm() -> impl IntoView {
    let state = expect_context::<AppState>();
    let resume = state.resume_data;

    let add_education = move |_| {
        resume.update(|r| {
            r.education.push(Education {
                id: generate_id(),
                institution: String::new(),
                degree: String::new(),
                field: String::new(),
                start_date: String::new(),
                end_date: String::new(),
                description: String::new(),
            });
        });
        state.save_to_storage();
    };

    view! {
        <div class="space-y-4">
            {move || {
                let list = resume.get().education;
                list.into_iter().enumerate().map(|(index, edu)| {
                    let id = edu.id.clone();
                    let id_for_remove = id.clone();

                    view! {
                        <div class="p-5 border border-transparent shadow-md dark:border-slate-700/50 rounded-2xl bg-white dark:bg-slate-800/60 space-y-4 relative group">
                            <div class="flex items-center justify-between mb-2">
                                <span class="text-[11px] font-bold tracking-widest uppercase text-slate-400 dark:text-slate-500">
                                    {format!("Education #{}", index + 1)}
                                </span>
                                <button
                                    type="button"
                                    on:click=move |_| {
                                        let target = id_for_remove.clone();
                                        resume.update(|r| r.education.retain(|e| e.id != target));
                                        state.save_to_storage();
                                    }
                                    class="text-[10px] tracking-wider uppercase text-red-500 hover:bg-red-50 dark:hover:bg-red-500/10 px-3 py-1.5 rounded-full font-bold transition-colors"
                                >
                                    "Remove"
                                </button>
                            </div>

                            <MaterialInput
                                label="School / University"
                                value=Signal::derive({
                                    let id = id.clone();
                                    move || resume.get().education.iter().find(|e| e.id == id).map(|e| e.institution.clone()).unwrap_or_default()
                                })
                                on_change=Callback::new({
                                    let id = id.clone();
                                    move |val| {
                                        let target = id.clone();
                                        resume.update(|r| {
                                            if let Some(e) = r.education.iter_mut().find(|e| e.id == target) {
                                                e.institution = val;
                                            }
                                        });
                                        state.save_to_storage();
                                    }
                                })
                            />

                            <div class="grid grid-cols-2 gap-3">
                                <MaterialInput
                                    label="Degree"
                                    value=Signal::derive({
                                        let id = id.clone();
                                        move || resume.get().education.iter().find(|e| e.id == id).map(|e| e.degree.clone()).unwrap_or_default()
                                    })
                                    on_change=Callback::new({
                                        let id = id.clone();
                                        move |val| {
                                            let target = id.clone();
                                            resume.update(|r| {
                                                if let Some(e) = r.education.iter_mut().find(|e| e.id == target) {
                                                    e.degree = val;
                                                }
                                            });
                                            state.save_to_storage();
                                        }
                                    })
                                />
                                <MaterialInput
                                    label="Field of Study"
                                    value=Signal::derive({
                                        let id = id.clone();
                                        move || resume.get().education.iter().find(|e| e.id == id).map(|e| e.field.clone()).unwrap_or_default()
                                    })
                                    on_change=Callback::new({
                                        let id = id.clone();
                                        move |val| {
                                            let target = id.clone();
                                            resume.update(|r| {
                                                if let Some(e) = r.education.iter_mut().find(|e| e.id == target) {
                                                    e.field = val;
                                                }
                                            });
                                            state.save_to_storage();
                                        }
                                    })
                                />
                            </div>

                            <div class="grid grid-cols-2 gap-3">
                                <MaterialInput
                                    label="Start Date"
                                    input_type="month"
                                    value=Signal::derive({
                                        let id = id.clone();
                                        move || resume.get().education.iter().find(|e| e.id == id).map(|e| e.start_date.clone()).unwrap_or_default()
                                    })
                                    on_change=Callback::new({
                                        let id = id.clone();
                                        move |val| {
                                            let target = id.clone();
                                            resume.update(|r| {
                                                if let Some(e) = r.education.iter_mut().find(|e| e.id == target) {
                                                    e.start_date = val;
                                                }
                                            });
                                            state.save_to_storage();
                                        }
                                    })
                                />
                                <MaterialInput
                                    label="End Date"
                                    input_type="month"
                                    value=Signal::derive({
                                        let id = id.clone();
                                        move || resume.get().education.iter().find(|e| e.id == id).map(|e| e.end_date.clone()).unwrap_or_default()
                                    })
                                    on_change=Callback::new({
                                        let id = id.clone();
                                        move |val| {
                                            let target = id.clone();
                                            resume.update(|r| {
                                                if let Some(e) = r.education.iter_mut().find(|e| e.id == target) {
                                                    e.end_date = val;
                                                }
                                            });
                                            state.save_to_storage();
                                        }
                                    })
                                />
                            </div>

                            <MaterialTextarea
                                label="Description & Honors"
                                value=Signal::derive({
                                    let id = id.clone();
                                    move || resume.get().education.iter().find(|e| e.id == id).map(|e| e.description.clone()).unwrap_or_default()
                                })
                                on_change=Callback::new({
                                    let id = id.clone();
                                    move |val| {
                                        let target = id.clone();
                                        resume.update(|r| {
                                            if let Some(e) = r.education.iter_mut().find(|e| e.id == target) {
                                                e.description = val;
                                            }
                                        });
                                        state.save_to_storage();
                                    }
                                })
                                rows=3
                            />
                        </div>
                    }
                }).collect::<Vec<_>>()
            }}

            <button
                type="button"
                on:click=add_education
                class="w-full py-3.5 border-2 border-dashed border-slate-300 dark:border-slate-600 rounded-full text-[13px] font-bold tracking-wider uppercase text-slate-500 hover:bg-slate-50 hover:text-cyan-600 hover:border-cyan-300 dark:hover:bg-slate-800 transition-colors"
            >
                "+ Add Education"
            </button>
        </div>
    }
}
