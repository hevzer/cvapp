use crate::components::ui::material_input::MaterialInput;
use crate::components::ui::material_textarea::MaterialTextarea;
use crate::models::{generate_id, Volunteering};
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn VolunteeringForm() -> impl IntoView {
    let state = expect_context::<AppState>();
    let resume = state.resume_data;

    let add_volunteering = move |_| {
        resume.update(|r| {
            r.volunteering.push(Volunteering {
                id: generate_id(),
                organization: String::new(),
                role: String::new(),
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
                let list = resume.get().volunteering;
                list.into_iter().enumerate().map(|(index, item)| {
                    let id = item.id.clone();
                    let id_for_remove = id.clone();
                    let id_for_curr = id.clone();

                    view! {
                        <div class="p-5 border border-transparent shadow-md dark:border-slate-700/50 rounded-2xl bg-white dark:bg-slate-800/60 space-y-4 relative group">
                            <div class="flex items-center justify-between mb-2">
                                <span class="text-[11px] font-bold tracking-widest uppercase text-slate-400 dark:text-slate-500">
                                    {format!("Volunteering #{}", index + 1)}
                                </span>
                                <button
                                    type="button"
                                    on:click=move |_| {
                                        let target = id_for_remove.clone();
                                        resume.update(|r| r.volunteering.retain(|v| v.id != target));
                                        state.save_to_storage();
                                    }
                                    class="text-[10px] tracking-wider uppercase text-red-500 hover:bg-red-50 dark:hover:bg-red-500/10 px-3 py-1.5 rounded-full font-bold transition-colors"
                                >
                                    "Remove"
                                </button>
                            </div>

                            <div class="grid grid-cols-2 gap-3">
                                {
                                    let id_org1 = id.clone();
                                    let id_org2 = id.clone();
                                    view! {
                                        <MaterialInput
                                            label="Organization"
                                            value=Signal::derive(move || {
                                                resume.get().volunteering.iter().find(|v| v.id == id_org1).map(|v| v.organization.clone()).unwrap_or_default()
                                            })
                                            on_change=Callback::new(move |val| {
                                                let target = id_org2.clone();
                                                resume.update(|r| {
                                                    if let Some(v) = r.volunteering.iter_mut().find(|v| v.id == target) {
                                                        v.organization = val;
                                                    }
                                                });
                                                state.save_to_storage();
                                            })
                                        />
                                    }
                                }
                                {
                                    let id_role1 = id.clone();
                                    let id_role2 = id.clone();
                                    view! {
                                        <MaterialInput
                                            label="Role"
                                            value=Signal::derive(move || {
                                                resume.get().volunteering.iter().find(|v| v.id == id_role1).map(|v| v.role.clone()).unwrap_or_default()
                                            })
                                            on_change=Callback::new(move |val| {
                                                let target = id_role2.clone();
                                                resume.update(|r| {
                                                    if let Some(v) = r.volunteering.iter_mut().find(|v| v.id == target) {
                                                        v.role = val;
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
                                                resume.get().volunteering.iter().find(|v| v.id == id_s1).map(|v| v.start_date.clone()).unwrap_or_default()
                                            })
                                            on_change=Callback::new(move |val| {
                                                let target = id_s2.clone();
                                                resume.update(|r| {
                                                    if let Some(v) = r.volunteering.iter_mut().find(|v| v.id == target) {
                                                        v.start_date = val;
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
                                                resume.get().volunteering.iter().find(|v| v.id == id_e1).map(|v| v.end_date.clone()).unwrap_or_default()
                                            })
                                            on_change=Callback::new(move |val| {
                                                let target = id_e2.clone();
                                                resume.update(|r| {
                                                    if let Some(v) = r.volunteering.iter_mut().find(|v| v.id == target) {
                                                        v.end_date = val;
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
                                    prop:checked=move || item.current
                                    on:change=move |ev| {
                                        let target = id_for_curr.clone();
                                        let is_checked = event_target_checked(&ev);
                                        resume.update(|r| {
                                            if let Some(v) = r.volunteering.iter_mut().find(|v| v.id == target) {
                                                v.current = is_checked;
                                                if is_checked {
                                                    v.end_date.clear();
                                                }
                                            }
                                        });
                                        state.save_to_storage();
                                    }
                                    class="rounded border-slate-300 text-cyan-600 focus:ring-cyan-500 w-4 h-4 cursor-pointer"
                                />
                                <span class="font-medium text-[13px]">"Currently volunteering here"</span>
                            </label>

                            {
                                let id_d1 = id.clone();
                                let id_d2 = id.clone();
                                view! {
                                    <MaterialTextarea
                                        label="Description & Impact"
                                        value=Signal::derive(move || {
                                            resume.get().volunteering.iter().find(|v| v.id == id_d1).map(|v| v.description.clone()).unwrap_or_default()
                                        })
                                        on_change=Callback::new(move |val| {
                                            let target = id_d2.clone();
                                            resume.update(|r| {
                                                if let Some(v) = r.volunteering.iter_mut().find(|v| v.id == target) {
                                                    v.description = val;
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
                on:click=add_volunteering
                class="w-full py-3.5 border-2 border-dashed border-slate-300 dark:border-slate-600 rounded-full text-[13px] font-bold tracking-wider uppercase text-slate-500 hover:bg-slate-50 hover:text-cyan-600 hover:border-cyan-300 dark:hover:bg-slate-800 transition-colors"
            >
                "+ Add Volunteering"
            </button>
        </div>
    }
}
