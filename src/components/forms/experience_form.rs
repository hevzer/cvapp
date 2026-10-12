use crate::components::ui::material_input::MaterialInput;
use crate::components::ui::material_textarea::MaterialTextarea;
use crate::models::{generate_id, Experience};
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn ExperienceForm() -> impl IntoView {
    let state = expect_context::<AppState>();
    let resume = state.resume_data;

    let add_experience = move |_| {
        resume.update(|r| {
            r.experience.push(Experience {
                id: generate_id(),
                company: String::new(),
                position: String::new(),
                start_date: String::new(),
                end_date: String::new(),
                current: false,
                description: String::new(),
            });
        });
        state.save_to_storage();
    };

    view! {
        <div class="space-y-4">
            {move || {
                let experiences = resume.get().experience;
                experiences.into_iter().enumerate().map(|(index, exp)| {
                    let id = exp.id.clone();
                    let id_for_remove = id.clone();
                    let id_for_curr = id.clone();

                    view! {
                        <div class="p-5 border border-transparent shadow-md dark:border-slate-700/50 rounded-2xl bg-white dark:bg-slate-800/60 space-y-4 relative group">
                            <div class="flex items-center justify-between mb-2">
                                <span class="text-[11px] font-bold tracking-widest uppercase text-slate-400 dark:text-slate-500">
                                    {format!("Experience #{}", index + 1)}
                                </span>
                                <button
                                    type="button"
                                    on:click=move |_| {
                                        let target_id = id_for_remove.clone();
                                        resume.update(|r| r.experience.retain(|e| e.id != target_id));
                                        state.save_to_storage();
                                    }
                                    class="text-[10px] tracking-wider uppercase text-red-500 hover:bg-red-50 dark:hover:bg-red-500/10 px-3 py-1.5 rounded-full font-bold transition-colors"
                                >
                                    "Remove"
                                </button>
                            </div>

                            <div class="grid grid-cols-2 gap-3">
                                {
                                    let id_c1 = id.clone();
                                    let id_c2 = id.clone();
                                    view! {
                                        <MaterialInput
                                            label="Company"
                                            value=Signal::derive(move || {
                                                resume.get().experience.iter().find(|e| e.id == id_c1).map(|e| e.company.clone()).unwrap_or_default()
                                            })
                                            on_change=Callback::new(move |val| {
                                                let target = id_c2.clone();
                                                resume.update(|r| {
                                                    if let Some(e) = r.experience.iter_mut().find(|e| e.id == target) {
                                                        e.company = val;
                                                    }
                                                });
                                                state.save_to_storage();
                                            })
                                        />
                                    }
                                }
                                {
                                    let id_p1 = id.clone();
                                    let id_p2 = id.clone();
                                    view! {
                                        <MaterialInput
                                            label="Position"
                                            value=Signal::derive(move || {
                                                resume.get().experience.iter().find(|e| e.id == id_p1).map(|e| e.position.clone()).unwrap_or_default()
                                            })
                                            on_change=Callback::new(move |val| {
                                                let target = id_p2.clone();
                                                resume.update(|r| {
                                                    if let Some(e) = r.experience.iter_mut().find(|e| e.id == target) {
                                                        e.position = val;
                                                    }
                                                });
                                                state.save_to_storage();
                                            })
                                        />
                                    }
                                }
                            </div>

                            <div class="grid grid-cols-2 gap-3">
                                {
                                    let id_s1 = id.clone();
                                    let id_s2 = id.clone();
                                    view! {
                                        <MaterialInput
                                            label="Start Date"
                                            input_type="month"
                                            value=Signal::derive(move || {
                                                resume.get().experience.iter().find(|e| e.id == id_s1).map(|e| e.start_date.clone()).unwrap_or_default()
                                            })
                                            on_change=Callback::new(move |val| {
                                                let target = id_s2.clone();
                                                resume.update(|r| {
                                                    if let Some(e) = r.experience.iter_mut().find(|e| e.id == target) {
                                                        e.start_date = val;
                                                    }
                                                });
                                                state.save_to_storage();
                                            })
                                        />
                                    }
                                }
                                {
                                    let id_e1 = id.clone();
                                    let id_e2 = id.clone();
                                    view! {
                                        <MaterialInput
                                            label="End Date"
                                            input_type="month"
                                            value=Signal::derive(move || {
                                                resume.get().experience.iter().find(|e| e.id == id_e1).map(|e| e.end_date.clone()).unwrap_or_default()
                                            })
                                            on_change=Callback::new(move |val| {
                                                let target = id_e2.clone();
                                                resume.update(|r| {
                                                    if let Some(e) = r.experience.iter_mut().find(|e| e.id == target) {
                                                        e.end_date = val;
                                                    }
                                                });
                                                state.save_to_storage();
                                            })
                                        />
                                    }
                                }
                            </div>

                            <label class="flex items-center gap-2 text-sm text-slate-600 dark:text-slate-300 cursor-pointer pt-1 pb-2">
                                <input
                                    type="checkbox"
                                    prop:checked=move || exp.current
                                    on:change=move |ev| {
                                        let target = id_for_curr.clone();
                                        let is_checked = event_target_checked(&ev);
                                        resume.update(|r| {
                                            if let Some(e) = r.experience.iter_mut().find(|e| e.id == target) {
                                                e.current = is_checked;
                                                if is_checked {
                                                    e.end_date.clear();
                                                }
                                            }
                                        });
                                        state.save_to_storage();
                                    }
                                    class="rounded border-slate-300 text-cyan-600 focus:ring-cyan-500 w-4 h-4 cursor-pointer"
                                />
                                <span class="font-medium text-[13px]">"Currently working here"</span>
                            </label>

                            {
                                let id_d1 = id.clone();
                                let id_d2 = id.clone();
                                view! {
                                    <MaterialTextarea
                                        label="Description & Achievements"
                                        value=Signal::derive(move || {
                                            resume.get().experience.iter().find(|e| e.id == id_d1).map(|e| e.description.clone()).unwrap_or_default()
                                        })
                                        on_change=Callback::new(move |val| {
                                            let target = id_d2.clone();
                                            resume.update(|r| {
                                                if let Some(e) = r.experience.iter_mut().find(|e| e.id == target) {
                                                    e.description = val;
                                                }
                                            });
                                            state.save_to_storage();
                                        })
                                        rows=4
                                    />
                                }
                            }
                        </div>
                    }
                }).collect::<Vec<_>>()
            }}

            <button
                type="button"
                on:click=add_experience
                class="w-full py-3.5 border-2 border-dashed border-slate-300 dark:border-slate-600 rounded-full text-[13px] font-bold tracking-wider uppercase text-slate-500 hover:bg-slate-50 hover:text-cyan-600 hover:border-cyan-300 dark:hover:bg-slate-800 transition-colors"
            >
                "+ Add Experience"
            </button>
        </div>
    }
}
