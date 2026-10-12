use crate::state::AppState;
use leptos::prelude::*;

struct ColorPreset {
    name: &'static str,
    hex: &'static str,
}

const COLOR_PRESETS: &[ColorPreset] = &[
    ColorPreset { name: "Indigo", hex: "#6366f1" },
    ColorPreset { name: "Blue", hex: "#3b82f6" },
    ColorPreset { name: "Emerald", hex: "#10b981" },
    ColorPreset { name: "Rose", hex: "#f43f5e" },
    ColorPreset { name: "Amber", hex: "#f59e0b" },
    ColorPreset { name: "Cyan", hex: "#06b6d4" },
    ColorPreset { name: "Slate", hex: "#475569" },
    ColorPreset { name: "Teal", hex: "#14b8a6" },
];

const SIDEBAR_PRESETS: &[ColorPreset] = &[
    ColorPreset { name: "Slate 800", hex: "#1e293b" },
    ColorPreset { name: "Zinc 900", hex: "#18181b" },
    ColorPreset { name: "Navy", hex: "#0f172a" },
    ColorPreset { name: "Midnight", hex: "#020617" },
    ColorPreset { name: "Emerald 900", hex: "#064e3b" },
    ColorPreset { name: "Rose 900", hex: "#4c0519" },
    ColorPreset { name: "Cyan 900", hex: "#164e63" },
    ColorPreset { name: "Black", hex: "#000000" },
];

const FONT_OPTIONS: &[&'static str] = &["Inter", "Roboto", "Outfit", "Lora", "Merriweather"];

#[component]
pub fn StyleCustomizer() -> impl IntoView {
    let state = expect_context::<AppState>();
    let accent_color = state.accent_color;
    let sidebar_color = state.sidebar_color;
    let font_family = state.font_family;
    let hide_photo = state.hide_photo;

    view! {
        <div class="bg-white dark:bg-slate-900 p-5 rounded-2xl border border-transparent shadow-[0_2px_8px_rgb(0,0,0,0.08)] dark:shadow-none space-y-5">
            {/* Accent Color */}
            <div>
                <label class="flex items-center gap-2 text-[11px] font-bold uppercase tracking-widest text-slate-700 dark:text-slate-300 mb-3 block">
                    <i class="bi bi-palette-fill text-[11px] text-cyan-500"></i>
                    "Accent Color"
                </label>
                <div class="flex items-center gap-2 flex-wrap">
                    {COLOR_PRESETS.iter().map(|color| {
                        let hex = color.hex;
                        let title = color.name;
                        view! {
                            <button
                                type="button"
                                title=title
                                on:click=move |_| {
                                    accent_color.set(hex.to_string());
                                    state.save_to_storage();
                                }
                                class="relative w-7 h-7 flex items-center justify-center rounded-full transition-all duration-300 active:scale-90 outline-none hover:shadow-md"
                                style=format!("background-color: {}", hex)
                            >
                                <i class=move || {
                                    if accent_color.get() == hex {
                                        "bi bi-check2 text-white text-sm transition-transform duration-300 scale-100 opacity-100"
                                    } else {
                                        "bi bi-check2 text-white text-sm transition-transform duration-300 scale-0 opacity-0"
                                    }
                                }></i>
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>

            {/* Sidebar Color */}
            <div>
                <label class="flex items-center gap-2 text-[11px] font-bold uppercase tracking-widest text-slate-700 dark:text-slate-300 mb-3 block">
                    <i class="bi bi-layout-sidebar-inset text-[11px] text-cyan-500"></i>
                    "Sidebar Color"
                </label>
                <div class="flex items-center gap-2 flex-wrap">
                    {SIDEBAR_PRESETS.iter().map(|color| {
                        let hex = color.hex;
                        let title = color.name;
                        view! {
                            <button
                                type="button"
                                title=title
                                on:click=move |_| {
                                    sidebar_color.set(hex.to_string());
                                    state.save_to_storage();
                                }
                                class="relative w-7 h-7 flex items-center justify-center rounded-full transition-all duration-300 active:scale-90 outline-none hover:shadow-[0_2px_8px_rgb(0,0,0,0.3)] border border-white/10"
                                style=format!("background-color: {}", hex)
                            >
                                <i class=move || {
                                    if sidebar_color.get() == hex {
                                        "bi bi-check2 text-white text-sm transition-transform duration-300 scale-100 opacity-100"
                                    } else {
                                        "bi bi-check2 text-white text-sm transition-transform duration-300 scale-0 opacity-0"
                                    }
                                }></i>
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>

            {/* Font Family */}
            <div>
                <label class="flex items-center gap-2 text-[11px] font-bold uppercase tracking-widest text-slate-700 dark:text-slate-300 mb-2 block">
                    <i class="bi bi-fonts text-[12px] text-cyan-500"></i>
                    "Typography"
                </label>
                <select
                    prop:value=move || font_family.get()
                    on:change=move |ev| {
                        font_family.set(event_target_value(&ev));
                        state.save_to_storage();
                    }
                    class="w-full text-[13px] font-bold uppercase tracking-widest text-slate-800 dark:text-slate-200 bg-slate-50 dark:bg-slate-800 py-3 px-4 rounded-xl border border-slate-200 dark:border-slate-700 outline-none focus:border-cyan-500 focus:ring-1 focus:ring-cyan-500 appearance-none shadow-sm cursor-pointer transition-all hover:bg-slate-100 dark:hover:bg-slate-700/80"
                >
                    {FONT_OPTIONS.iter().map(|font| {
                        let f = *font;
                        view! {
                            <option value=f class="font-semibold text-sm capitalize tracking-normal">
                                {f}
                            </option>
                        }
                    }).collect::<Vec<_>>()}
                </select>
            </div>

            {/* Hide Photo */}
            <div>
                <label class="flex items-start gap-3 cursor-pointer select-none">
                    <input
                        type="checkbox"
                        prop:checked=move || hide_photo.get()
                        on:change=move |ev| {
                            hide_photo.set(event_target_checked(&ev));
                            state.save_to_storage();
                        }
                        class="mt-0.5 rounded border-slate-300 text-cyan-600 focus:ring-cyan-500 w-4 h-4 cursor-pointer"
                    />
                    <span class="flex-1">
                        <span class="block text-[11px] font-bold uppercase tracking-widest text-slate-700 dark:text-slate-300">
                            "Hide photo"
                        </span>
                        <span class="block text-[11px] text-slate-500 dark:text-slate-400 mt-0.5 leading-snug">
                            "Recommended for US/UK applications. Some ATS flag photos as bias risk."
                        </span>
                    </span>
                </label>
            </div>
        </div>
    }
}
