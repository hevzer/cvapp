use crate::components::ui::material_input::MaterialInput;
use crate::models::{generate_id, Certification};
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn CertificationsForm() -> impl IntoView {
    let state = expect_context::<AppState>();
    let resume = state.resume_data;

    let add_cert = move |_| {
        resume.update(|r| {
            r.certifications.push(Certification {
                id: generate_id(),
                name: String::new(),
                issuer: String::new(),
                date: String::new(),
                url: String::new(),
            });
        });
        state.save_to_storage();
    };

    view! {
        <div class="space-y-4">
            {move || {
                let list = resume.get().certifications;
                list.into_iter().enumerate().map(|(index, cert)| {
                    let id = cert.id.clone();
                    let id_for_remove = id.clone();

                    view! {
                        <div class="p-5 border border-transparent shadow-md dark:border-slate-700/50 rounded-2xl bg-white dark:bg-slate-800/60 space-y-4 relative group">
                            <div class="flex items-center justify-between mb-2">
                                <span class="text-[11px] font-bold tracking-widest uppercase text-slate-400 dark:text-slate-500">
                                    {format!("Certification #{}", index + 1)}
                                </span>
                                <button
                                    type="button"
                                    on:click=move |_| {
                                        let target = id_for_remove.clone();
                                        resume.update(|r| r.certifications.retain(|c| c.id != target));
                                        state.save_to_storage();
                                    }
                                    class="text-[10px] tracking-wider uppercase text-red-500 hover:bg-red-50 dark:hover:bg-red-500/10 px-3 py-1.5 rounded-full font-bold transition-colors"
                                >
                                    "Remove"
                                </button>
                            </div>

                            <MaterialInput
                                label="Certification Name"
                                value=Signal::derive({
                                    let id = id.clone();
                                    move || resume.get().certifications.iter().find(|c| c.id == id).map(|c| c.name.clone()).unwrap_or_default()
                                })
                                on_change=Callback::new({
                                    let id = id.clone();
                                    move |val| {
                                        let target = id.clone();
                                        resume.update(|r| {
                                            if let Some(c) = r.certifications.iter_mut().find(|c| c.id == target) {
                                                c.name = val;
                                            }
                                        });
                                        state.save_to_storage();
                                    }
                                })
                            />

                            <div class="grid grid-cols-2 gap-3">
                                <MaterialInput
                                    label="Issuer / Organization"
                                    value=Signal::derive({
                                        let id = id.clone();
                                        move || resume.get().certifications.iter().find(|c| c.id == id).map(|c| c.issuer.clone()).unwrap_or_default()
                                    })
                                    on_change=Callback::new({
                                        let id = id.clone();
                                        move |val| {
                                            let target = id.clone();
                                            resume.update(|r| {
                                                if let Some(c) = r.certifications.iter_mut().find(|c| c.id == target) {
                                                    c.issuer = val;
                                                }
                                            });
                                            state.save_to_storage();
                                        }
                                    })
                                />
                                <MaterialInput
                                    label="Issue Date"
                                    input_type="month"
                                    value=Signal::derive({
                                        let id = id.clone();
                                        move || resume.get().certifications.iter().find(|c| c.id == id).map(|c| c.date.clone()).unwrap_or_default()
                                    })
                                    on_change=Callback::new({
                                        let id = id.clone();
                                        move |val| {
                                            let target = id.clone();
                                            resume.update(|r| {
                                                if let Some(c) = r.certifications.iter_mut().find(|c| c.id == target) {
                                                    c.date = val;
                                                }
                                            });
                                            state.save_to_storage();
                                        }
                                    })
                                />
                            </div>

                            <MaterialInput
                                label="Credential URL / ID"
                                value=Signal::derive({
                                    let id = id.clone();
                                    move || resume.get().certifications.iter().find(|c| c.id == id).map(|c| c.url.clone()).unwrap_or_default()
                                })
                                on_change=Callback::new({
                                    let id = id.clone();
                                    move |val| {
                                        let target = id.clone();
                                        resume.update(|r| {
                                            if let Some(c) = r.certifications.iter_mut().find(|c| c.id == target) {
                                                c.url = val;
                                            }
                                        });
                                        state.save_to_storage();
                                    }
                                })
                            />
                        </div>
                    }
                }).collect::<Vec<_>>()
            }}

            <button
                type="button"
                on:click=add_cert
                class="w-full py-3.5 border-2 border-dashed border-slate-300 dark:border-slate-600 rounded-full text-[13px] font-bold tracking-wider uppercase text-slate-500 hover:bg-slate-50 hover:text-cyan-600 hover:border-cyan-300 dark:hover:bg-slate-800 transition-colors"
            >
                "+ Add Certification"
            </button>
        </div>
    }
}
