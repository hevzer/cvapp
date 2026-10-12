use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn TextScaleSlider() -> impl IntoView {
    let state = expect_context::<AppState>();
    let text_scale = state.text_scale;

    view! {
        <div class="flex items-center gap-3 px-1 pt-2">
            <i class="bi bi-type text-slate-400 dark:text-slate-500 text-sm"></i>
            <input
                type="range"
                min="80"
                max="120"
                step="5"
                prop:value=move || (text_scale.get() * 100.0).round().to_string()
                on:input=move |ev| {
                    if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                        text_scale.set(val / 100.0);
                        state.save_to_storage();
                    }
                }
                class="flex-1 h-2 bg-slate-200 dark:bg-slate-700/50 rounded-full appearance-none cursor-pointer accent-cyan-600 [&::-webkit-slider-thumb]:w-5 [&::-webkit-slider-thumb]:h-5 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-cyan-600 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:shadow-lg hover:[&::-webkit-slider-thumb]:scale-110 [&::-webkit-slider-thumb]:transition-transform"
            />
            <span class="text-xs font-bold text-slate-500 dark:text-slate-400 min-w-[36px] text-right tabular-nums">
                {move || format!("{}%", (text_scale.get() * 100.0).round())}
            </span>
            {move || {
                let pct = (text_scale.get() * 100.0).round() as i64;
                if pct != 100 {
                    view! {
                        <button
                            type="button"
                            on:click=move |_| {
                                text_scale.set(1.0);
                                state.save_to_storage();
                            }
                            class="text-[10px] text-slate-400 hover:text-cyan-600 transition-colors"
                            title="Reset to 100%"
                        >
                            <i class="bi bi-arrow-counterclockwise"></i>
                        </button>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}
        </div>
    }
}

#[component]
pub fn SpacingSlider() -> impl IntoView {
    let state = expect_context::<AppState>();
    let spacing_scale = state.spacing_scale;

    view! {
        <div class="bg-white dark:bg-slate-900 p-4 rounded-2xl border border-transparent shadow-[0_2px_8px_rgb(0,0,0,0.08)] dark:shadow-none space-y-3">
            <div class="flex items-center justify-between pointer-events-none">
                <label class="text-[11px] font-bold uppercase tracking-widest text-slate-700 dark:text-slate-300 flex items-center gap-2">
                    <i class="bi bi-distribute-vertical text-[10px] text-cyan-500"></i>
                    "Content Spacing"
                </label>
                <span class="text-[10px] font-bold tracking-wider bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400 px-2 py-0.5 rounded-md">
                    {move || format!("{}%", (spacing_scale.get() * 100.0).round())}
                </span>
            </div>
            <div class="flex items-center gap-3">
                <i class="bi bi-dash text-slate-400 text-xs"></i>
                <input
                    type="range"
                    min="0.1"
                    max="2.0"
                    step="0.05"
                    prop:value=move || spacing_scale.get().to_string()
                    on:input=move |ev| {
                        if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                            spacing_scale.set(val);
                            state.save_to_storage();
                        }
                    }
                    class="w-full h-2 bg-slate-200 dark:bg-slate-700/50 rounded-full appearance-none cursor-pointer accent-cyan-600 [&::-webkit-slider-thumb]:w-5 [&::-webkit-slider-thumb]:h-5 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-cyan-600 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:shadow-lg hover:[&::-webkit-slider-thumb]:scale-110 [&::-webkit-slider-thumb]:transition-transform"
                    aria-label="Adjust CV content spacing"
                />
                <i class="bi bi-plus text-slate-400 text-xs"></i>
            </div>
            <div class="flex justify-between text-[10px] text-slate-400 font-medium px-1">
                <span>"Compact"</span>
                <span>"Standard"</span>
                <span>"Spacious"</span>
            </div>
        </div>
    }
}

#[component]
pub fn FontScaleSlider() -> impl IntoView {
    let state = expect_context::<AppState>();
    let font_scale = state.font_scale;

    view! {
        <div class="bg-white dark:bg-slate-900 p-4 rounded-2xl border border-transparent shadow-[0_2px_8px_rgb(0,0,0,0.08)] dark:shadow-none space-y-3">
            <div class="flex items-center justify-between pointer-events-none">
                <label class="text-[11px] font-bold uppercase tracking-widest text-slate-700 dark:text-slate-300 flex items-center gap-2">
                    <i class="bi bi-type text-[10px] text-cyan-500"></i>
                    "Font Size"
                </label>
                <span class="text-[10px] font-bold tracking-wider bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400 px-2 py-0.5 rounded-md">
                    {move || format!("{}%", (font_scale.get() * 100.0).round())}
                </span>
            </div>
            <div class="flex items-center gap-3">
                <i class="bi bi-type text-slate-400 text-[10px]"></i>
                <input
                    type="range"
                    min="0.5"
                    max="1.5"
                    step="0.05"
                    prop:value=move || font_scale.get().to_string()
                    on:input=move |ev| {
                        if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                            font_scale.set(val);
                            state.save_to_storage();
                        }
                    }
                    class="w-full h-2 bg-slate-200 dark:bg-slate-700/50 rounded-full appearance-none cursor-pointer accent-cyan-600 [&::-webkit-slider-thumb]:w-5 [&::-webkit-slider-thumb]:h-5 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-cyan-600 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:shadow-lg hover:[&::-webkit-slider-thumb]:scale-110 [&::-webkit-slider-thumb]:transition-transform"
                    aria-label="Adjust CV font size"
                />
                <i class="bi bi-type text-slate-400 text-sm"></i>
            </div>
            <div class="flex justify-between text-[10px] text-slate-400 font-medium px-1">
                <span>"Small"</span>
                <span>"Standard"</span>
                <span>"Large"</span>
            </div>
        </div>
    }
}
