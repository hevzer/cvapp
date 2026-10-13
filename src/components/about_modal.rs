use leptos::portal::Portal;
use leptos::prelude::*;

#[component]
pub fn AboutModal() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    let _ = window_event_listener(leptos::ev::keydown, move |ev: web_sys::KeyboardEvent| {
        if is_open.get() && ev.key() == "Escape" {
            set_is_open.set(false);
        }
    });

    view! {
        <button
            type="button"
            on:click=move |e| {
                e.prevent_default();
                e.stop_propagation();
                set_is_open.set(true);
            }
            class="px-3.5 py-1.5 text-xs font-bold uppercase tracking-wider rounded-full text-slate-500 hover:bg-slate-100 dark:hover:bg-white/[0.08] hover:text-slate-700 dark:text-slate-400 dark:hover:text-slate-200 active:scale-95 transition-all duration-200 cursor-pointer flex items-center gap-1.5"
        >
            <i class="bi bi-info-circle text-[13px]"></i>
            <span>"About"</span>
        </button>

        <Portal>
            {move || is_open.get().then(|| view! {
                <div
                    on:click=move |_| set_is_open.set(false)
                    class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm animate-fade-in print:hidden"
                >
                    <div
                        on:click=move |e| e.stop_propagation()
                        class="bg-white dark:bg-[#141414] rounded-2xl shadow-2xl w-full max-w-lg max-h-[90vh] flex flex-col overflow-hidden animate-slide-up border border-slate-200 dark:border-white/[0.08]"
                    >
                        {/* Header */}
                        <div class="flex-shrink-0 flex items-center justify-between px-6 py-4 border-b border-slate-100 dark:border-white/[0.06] bg-slate-50/80 dark:bg-white/[0.02]">
                            <h2 class="text-base font-extrabold text-slate-900 dark:text-white flex items-center gap-2.5">
                                <div class="w-7 h-7 rounded-lg flex items-center justify-center overflow-hidden bg-white shadow-sm border border-slate-200/60 dark:border-white/[0.1]">
                                    <img src="/cvapp_macos_logo.png" alt="CVapp Logo" class="w-full h-full object-contain" />
                                </div>
                                <span>"About CVapp"</span>
                            </h2>
                            <button
                                type="button"
                                on:click=move |_| set_is_open.set(false)
                                class="w-7 h-7 rounded-full flex items-center justify-center text-slate-400 hover:text-slate-600 dark:hover:text-slate-200 hover:bg-slate-200/60 dark:hover:bg-white/[0.08] transition-colors cursor-pointer"
                                aria-label="Close modal"
                            >
                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M6 18L18 6M6 6l12 12" />
                                </svg>
                            </button>
                        </div>

                        {/* Body */}
                        <div class="p-6 overflow-y-auto space-y-5">
                            <p class="text-xs text-slate-600 dark:text-slate-300 leading-relaxed">
                                "CVapp is a fast, privacy-first resume and CV builder written in Rust. All data parsing, persistence, and PDF formatting happen entirely on your local machine with zero backend tracking or server dependencies."
                            </p>

                            <div>
                                <h3 class="text-[11px] tracking-widest font-bold uppercase text-slate-400 dark:text-slate-500 mb-3">
                                    "Technologies & Architecture"
                                </h3>

                                <div class="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
                                    <div class="p-3 rounded-xl bg-slate-50 dark:bg-white/[0.03] border border-slate-100 dark:border-white/[0.05]">
                                        <div class="text-xs font-bold text-slate-900 dark:text-white flex items-center gap-1.5">
                                            <span class="text-indigo-500">"🦀"</span> "Rust & Leptos"
                                        </div>
                                        <p class="text-[11px] text-slate-500 dark:text-slate-400 mt-1 leading-normal">
                                            "Reactive fine-grained frontend compiled directly to high-speed WebAssembly."
                                        </p>
                                    </div>

                                    <div class="p-3 rounded-xl bg-slate-50 dark:bg-white/[0.03] border border-slate-100 dark:border-white/[0.05]">
                                        <div class="text-xs font-bold text-slate-900 dark:text-white flex items-center gap-1.5">
                                            <span class="text-cyan-500">"⚡"</span> "Tauri 2"
                                        </div>
                                        <p class="text-[11px] text-slate-500 dark:text-slate-400 mt-1 leading-normal">
                                            "Native desktop distribution for Windows and Linux with native file dialogues."
                                        </p>
                                    </div>

                                    <div class="p-3 rounded-xl bg-slate-50 dark:bg-white/[0.03] border border-slate-100 dark:border-white/[0.05]">
                                        <div class="text-xs font-bold text-slate-900 dark:text-white flex items-center gap-1.5">
                                            <span class="text-teal-500">"🎨"</span> "Tailwind CSS v4"
                                        </div>
                                        <p class="text-[11px] text-slate-500 dark:text-slate-400 mt-1 leading-normal">
                                            "Next-gen CSS engine with native print page constraints and dark theme."
                                        </p>
                                    </div>

                                    <div class="p-3 rounded-xl bg-slate-50 dark:bg-white/[0.03] border border-slate-100 dark:border-white/[0.05]">
                                        <div class="text-xs font-bold text-slate-900 dark:text-white flex items-center gap-1.5">
                                            <span class="text-orange-500">"📦"</span> "Trunk Bundler"
                                        </div>
                                        <p class="text-[11px] text-slate-500 dark:text-slate-400 mt-1 leading-normal">
                                            "Asset pipeline with automated pre-build CSS hooks and offline PWA assets."
                                        </p>
                                    </div>

                                    <div class="p-3 rounded-xl bg-slate-50 dark:bg-white/[0.03] border border-slate-100 dark:border-white/[0.05]">
                                        <div class="text-xs font-bold text-slate-900 dark:text-white flex items-center gap-1.5">
                                            <span class="text-emerald-500">"🔒"</span> "100% Privacy"
                                        </div>
                                        <p class="text-[11px] text-slate-500 dark:text-slate-400 mt-1 leading-normal">
                                            "Zero telemetry, zero external APIs. Your career data never leaves your device."
                                        </p>
                                    </div>

                                    <div class="p-3 rounded-xl bg-slate-50 dark:bg-white/[0.03] border border-slate-100 dark:border-white/[0.05]">
                                        <div class="text-xs font-bold text-slate-900 dark:text-white flex items-center gap-1.5">
                                            <span class="text-blue-500">"📥"</span> "LinkedIn & JSON Sync"
                                        </div>
                                        <p class="text-[11px] text-slate-500 dark:text-slate-400 mt-1 leading-normal">
                                            "Client-side zip archive parsing and reactive auto-saving to local storage."
                                        </p>
                                    </div>
                                </div>
                            </div>

                            <div class="pt-4 border-t border-slate-100 dark:border-white/[0.06]">
                                <h3 class="text-[11px] tracking-widest font-bold uppercase text-slate-400 dark:text-slate-500 mb-1.5">
                                    "License"
                                </h3>
                                <p class="text-xs text-slate-600 dark:text-slate-300 leading-relaxed">
                                    "Licensed under the "
                                    <a
                                        href="https://www.gnu.org/licenses/gpl-3.0.en.html"
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="text-indigo-600 dark:text-indigo-400 hover:underline font-semibold"
                                    >
                                        "GNU General Public License v3.0"
                                    </a>
                                </p>
                            </div>
                        </div>

                        {/* Footer */}
                        <div class="flex-shrink-0 px-6 py-3.5 bg-slate-50/80 dark:bg-white/[0.02] border-t border-slate-100 dark:border-white/[0.06] flex items-center justify-between">
                            <span class="text-[11px] font-semibold text-slate-400 dark:text-slate-500">
                                "Version 0.2.0"
                            </span>
                            <button
                                type="button"
                                on:click=move |_| set_is_open.set(false)
                                class="px-4 py-2 text-xs font-bold text-white bg-indigo-600 hover:bg-indigo-700 active:scale-95 rounded-xl shadow-sm shadow-indigo-500/20 transition-all cursor-pointer"
                            >
                                "Close"
                            </button>
                        </div>
                    </div>
                </div>
            })}
        </Portal>
    }
}
