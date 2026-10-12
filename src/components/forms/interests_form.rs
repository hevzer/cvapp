use crate::components::ui::material_input::MaterialInput;
use crate::components::ui::material_textarea::MaterialTextarea;
use crate::models::{generate_id, Interest};
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn InterestsForm() -> impl IntoView {
    let state = expect_context::<AppState>();
    let resume = state.resume_data;

    let add_interest = move |_| {
        resume.update(|r| {
            r.interests.push(Interest {
                id: generate_id(),
                name: String::new(),
                description: String::new(),
            });
        });
        state.save_to_storage();
    };

    view! {
        <div class="space-y-4">
            {move || {
                let list = resume.get().interests;
                list.into_iter().enumerate().map(|(index, item)| {
                    let id = item.id.clone();
                    let id_for_remove = id.clone();

                    view! {
                        <div class="p-5 border border-transparent shadow-md dark:border-slate-700/50 rounded-2xl bg-white dark:bg-slate-800/60 space-y-4 relative group">
                            <div class="flex items-center justify-between mb-2">
                                <span class="text-[11px] font-bold tracking-widest uppercase text-slate-400 dark:text-slate-500">
                                    {format!("Interest #{}", index + 1)}
                                </span>
                                <button
                                    type="button"
                                    on:click=move |_| {
                                        let target = id_for_remove.clone();
                                        resume.update(|r| r.interests.retain(|i| i.id != target));
                                        state.save_to_storage();
                                    }
                                    class="text-[10px] tracking-wider uppercase text-red-500 hover:bg-red-50 dark:hover:bg-red-500/10 px-3 py-1.5 rounded-full font-bold transition-colors"
                                >
                                    "Remove"
                                </button>
                            </div>

                            <MaterialInput
                                label="Name (e.g. Photography)"
                                value=Signal::derive({
                                    let id = id.clone();
                                    move || resume.get().interests.iter().find(|i| i.id == id).map(|i| i.name.clone()).unwrap_or_default()
                                })
                                on_change=Callback::new({
                                    let id = id.clone();
                                    move |val| {
                                        let target = id.clone();
                                        resume.update(|r| {
                                            if let Some(i) = r.interests.iter_mut().find(|i| i.id == target) {
                                                i.name = val;
                                            }
                                        });
                                        state.save_to_storage();
                                    }
                                })
                            />

                            <MaterialTextarea
                                label="Description (Optional)"
                                value=Signal::derive({
                                    let id = id.clone();
                                    move || resume.get().interests.iter().find(|i| i.id == id).map(|i| i.description.clone()).unwrap_or_default()
                                })
                                on_change=Callback::new({
                                    let id = id.clone();
                                    move |val| {
                                        let target = id.clone();
                                        resume.update(|r| {
                                            if let Some(i) = r.interests.iter_mut().find(|i| i.id == target) {
                                                i.description = val;
                                            }
                                        });
                                        state.save_to_storage();
                                    }
                                })
                                rows=2
                            />
                        </div>
                    }
                }).collect::<Vec<_>>()
            }}

            <button
                type="button"
                on:click=add_interest
                class="w-full py-3.5 border-2 border-dashed border-slate-300 dark:border-slate-600 rounded-full text-[13px] font-bold tracking-wider uppercase text-slate-500 hover:bg-slate-50 hover:text-cyan-600 hover:border-cyan-300 dark:hover:bg-slate-800 transition-colors"
            >
                "+ Add Interest"
            </button>
        </div>
    }
}
