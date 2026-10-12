use crate::models::TemplateType;
use crate::state::AppState;
use leptos::prelude::*;

struct TemplateMeta {
    id: TemplateType,
    label: &'static str,
    desc: &'static str,
    icon: &'static str,
}

const TEMPLATES: &[TemplateMeta] = &[
    TemplateMeta {
        id: TemplateType::Ats,
        label: "ATS Minimalist",
        desc: "Strict semantic flow",
        icon: "file-text",
    },
    TemplateMeta {
        id: TemplateType::Modern,
        label: "Modern Creative",
        desc: "Sleek two-column",
        icon: "columns-gap",
    },
    TemplateMeta {
        id: TemplateType::Timeline,
        label: "Timeline",
        desc: "Vertical chronographics",
        icon: "clock-history",
    },
    TemplateMeta {
        id: TemplateType::TimelineTwoColumn,
        label: "Split Timeline",
        desc: "Sidebar & Timeline",
        icon: "layout-sidebar",
    },
];

#[component]
pub fn TemplateToggle() -> impl IntoView {
    let state = expect_context::<AppState>();
    let active_template = state.active_template;

    view! {
        <div class="grid grid-cols-2 gap-3 mb-1">
            {TEMPLATES.iter().map(|t| {
                let tid = t.id;
                let label = t.label;
                let desc = t.desc;
                let icon = t.icon;

                view! {
                    <button
                        type="button"
                        on:click=move |_| {
                            active_template.set(tid);
                            state.save_to_storage();
                        }
                        class=move || {
                            if active_template.get() == tid {
                                "relative flex flex-col items-start p-4 rounded-2xl text-left border transition-all duration-300 outline-none active:scale-[0.98] bg-cyan-50 border-cyan-200 dark:bg-cyan-900/40 dark:border-cyan-500/50 shadow-sm"
                            } else {
                                "relative flex flex-col items-start p-4 rounded-2xl text-left border transition-all duration-300 outline-none active:scale-[0.98] bg-white border-transparent shadow-[0_2px_8px_rgb(0,0,0,0.06)] hover:shadow-md dark:shadow-none dark:bg-slate-800 dark:hover:bg-slate-800/80 hover:border-slate-200 dark:hover:border-slate-700"
                            }
                        }
                    >
                        <div class=move || {
                            if active_template.get() == tid {
                                "mb-2 w-8 h-8 rounded-full flex items-center justify-center transition-colors bg-cyan-600 text-white"
                            } else {
                                "mb-2 w-8 h-8 rounded-full flex items-center justify-center transition-colors bg-slate-100 dark:bg-slate-700 text-slate-500 dark:text-slate-400"
                            }
                        }>
                            <i class=format!("bi bi-{} text-[13px]", icon)></i>
                        </div>
                        <div class=move || {
                            if active_template.get() == tid {
                                "text-[12px] font-bold tracking-wide uppercase transition-colors text-cyan-900 dark:text-cyan-200"
                            } else {
                                "text-[12px] font-bold tracking-wide uppercase transition-colors text-slate-700 dark:text-slate-200"
                            }
                        }>
                            {label}
                        </div>
                        <div class=move || {
                            if active_template.get() == tid {
                                "text-[10px] mt-1 font-medium transition-colors text-cyan-700/80 dark:text-cyan-300/80"
                            } else {
                                "text-[10px] mt-1 font-medium transition-colors text-slate-500 dark:text-slate-400"
                            }
                        }>
                            {desc}
                        </div>
                        {move || {
                            if active_template.get() == tid {
                                view! {
                                    <div class="absolute top-4 right-4 w-4 h-4 bg-cyan-600 rounded-full flex items-center justify-center shadow-sm">
                                        <i class="bi bi-check2 text-white text-[10px]"></i>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        }}
                    </button>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
