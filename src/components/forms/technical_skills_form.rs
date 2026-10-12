use crate::components::ui::material_input::MaterialInput;
use crate::components::ui::material_textarea::MaterialTextarea;
use crate::i18n::get_dictionary;
use crate::models::{generate_id, TechnicalSkill};
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn TechnicalSkillsForm() -> impl IntoView {
    let state = expect_context::<AppState>();
    let resume = state.resume_data;

    let add_skill = move |_| {
        resume.update(|r| {
            r.technical_skills.push(TechnicalSkill {
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
                let dict = get_dictionary(&resume.get().cv_language);
                let list = resume.get().technical_skills;
                list.into_iter().enumerate().map(|(index, skill)| {
                    let id = skill.id.clone();
                    let id_for_remove = id.clone();

                    view! {
                        <div class="p-5 border border-transparent shadow-md dark:border-slate-700/50 rounded-2xl bg-white dark:bg-slate-800/60 space-y-4 relative group">
                            <div class="flex items-center justify-between mb-2">
                                <span class="text-[11px] font-bold tracking-widest uppercase text-slate-400 dark:text-slate-500">
                                    {format!("{} #{}", dict.technical_skills, index + 1)}
                                </span>
                                <button
                                    type="button"
                                    on:click=move |_| {
                                        let target = id_for_remove.clone();
                                        resume.update(|r| r.technical_skills.retain(|s| s.id != target));
                                        state.save_to_storage();
                                    }
                                    class="text-[10px] tracking-wider uppercase text-red-500 hover:bg-red-50 dark:hover:bg-red-500/10 px-3 py-1.5 rounded-full font-bold transition-colors"
                                >
                                    "Remove"
                                </button>
                            </div>

                            <MaterialInput
                                label="Skill Name (e.g. React, Rust)"
                                value=Signal::derive({
                                    let id = id.clone();
                                    move || resume.get().technical_skills.iter().find(|s| s.id == id).map(|s| s.name.clone()).unwrap_or_default()
                                })
                                on_change=Callback::new({
                                    let id = id.clone();
                                    move |val| {
                                        let target = id.clone();
                                        resume.update(|r| {
                                            if let Some(s) = r.technical_skills.iter_mut().find(|s| s.id == target) {
                                                s.name = val;
                                            }
                                        });
                                        state.save_to_storage();
                                    }
                                })
                            />

                            <MaterialTextarea
                                label="Details & Proficiencies (Optional)"
                                value=Signal::derive({
                                    let id = id.clone();
                                    move || resume.get().technical_skills.iter().find(|s| s.id == id).map(|s| s.description.clone()).unwrap_or_default()
                                })
                                on_change=Callback::new({
                                    let id = id.clone();
                                    move |val| {
                                        let target = id.clone();
                                        resume.update(|r| {
                                            if let Some(s) = r.technical_skills.iter_mut().find(|s| s.id == target) {
                                                s.description = val;
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
                on:click=add_skill
                class="w-full py-3.5 border-2 border-dashed border-slate-300 dark:border-slate-600 rounded-full text-[13px] font-bold tracking-wider uppercase text-slate-500 hover:bg-slate-50 hover:text-cyan-600 hover:border-cyan-300 dark:hover:bg-slate-800 transition-colors"
            >
                {move || format!("+ Add {}", get_dictionary(&resume.get().cv_language).technical_skills)}
            </button>
        </div>
    }
}
