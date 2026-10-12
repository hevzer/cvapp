use leptos::prelude::*;

#[component]
pub fn AboutModal() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    view! {
        <button
            type="button"
            on:click=move |e| {
                e.prevent_default();
                e.stop_propagation();
                set_is_open.set(true);
            }
            class="px-4 py-2 text-[11px] font-bold uppercase tracking-widest rounded-full text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-slate-700 dark:text-slate-400 dark:hover:text-slate-300 active:scale-[0.98] transition-all duration-200 cursor-pointer"
        >
            <span class="flex items-center gap-1.5"><i class="bi bi-info-circle text-[13px]"></i> " About"</span>
        </button>

        {move || is_open.get().then(|| view! {
            <div class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-gray-900/50 backdrop-blur-sm animate-fade-in print:hidden">
                <div class="bg-white dark:bg-gray-800 rounded-2xl shadow-2xl w-full max-w-md max-h-[90vh] flex flex-col overflow-hidden animate-slide-up border border-gray-200 dark:border-gray-700">
                    <div class="flex-shrink-0 flex items-center justify-between px-6 py-4 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50">
                        <h2 class="text-lg font-bold text-gray-900 dark:text-white flex items-center gap-2">
                            <div class="w-8 h-8 rounded-lg flex items-center justify-center overflow-hidden bg-white shadow-sm">
                                <img src="/cvapp_macos_logo.png" alt="CVapp Logo" class="w-full h-full object-contain" />
                            </div>
                            "About CVapp"
                        </h2>
                        <button
                            on:click=move |_| set_is_open.set(false)
                            class="text-gray-400 hover:text-gray-500 dark:hover:text-gray-300 transition-colors cursor-pointer"
                        >
                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>

                    <div class="p-6 overflow-y-auto">
                        <p class="text-sm text-gray-600 dark:text-gray-300 mb-6 leading-relaxed">
                            "CVapp is a fast, privacy-first resume builder written in Rust. All data processing and PDF generation happens entirely in your local browser or desktop app with zero backend dependencies."
                        </p>

                        <h3 class="text-xs tracking-widest font-bold uppercase text-gray-400 dark:text-gray-500 mb-4">
                            "Technologies Used"
                        </h3>

                        <ul class="space-y-4">
                            <li class="flex flex-col">
                                <span class="text-sm font-semibold text-gray-900 dark:text-gray-100">"Rust & Leptos"</span>
                                <span class="text-sm text-gray-500 dark:text-gray-400">"High-performance reactive frontend compiled directly to WebAssembly"</span>
                            </li>
                            <li class="flex flex-col">
                                <span class="text-sm font-semibold text-gray-900 dark:text-gray-100">"Tauri 2"</span>
                                <span class="text-sm text-gray-500 dark:text-gray-400">"Lightweight, secure native desktop distribution for Windows and Linux"</span>
                            </li>
                            <li class="flex flex-col">
                                <span class="text-sm font-semibold text-gray-900 dark:text-gray-100">"Tailwind CSS v4"</span>
                                <span class="text-sm text-gray-500 dark:text-gray-400">"Next-generation styling engine with seamless dark mode and print variant support"</span>
                            </li>
                            <li class="flex flex-col">
                                <span class="text-sm font-semibold text-gray-900 dark:text-gray-100">"Trunk"</span>
                                <span class="text-sm text-gray-500 dark:text-gray-400">"WASM web application bundler and build pipeline"</span>
                            </li>
                            <li class="flex flex-col">
                                <span class="text-sm font-semibold text-gray-900 dark:text-gray-100">"PWA Ready"</span>
                                <span class="text-sm text-gray-500 dark:text-gray-400">"Installable Progressive Web App with offline service worker support"</span>
                            </li>
                            <li class="flex flex-col">
                                <span class="text-sm font-semibold text-gray-900 dark:text-gray-100">"100% Client-Side Privacy"</span>
                                <span class="text-sm text-gray-500 dark:text-gray-400">"Zero telemetry, zero server storage, your career data never leaves your device"</span>
                            </li>
                        </ul>

                        <div class="mt-6 pt-5 border-t border-gray-200 dark:border-gray-700">
                            <h3 class="text-xs tracking-widest font-bold uppercase text-gray-400 dark:text-gray-500 mb-2">
                                "License"
                            </h3>
                            <p class="text-sm text-gray-600 dark:text-gray-300 leading-relaxed">
                                "Licensed under the "
                                <a
                                    href="https://www.gnu.org/licenses/gpl-3.0.en.html"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="text-cyan-600 dark:text-cyan-400 hover:underline font-medium"
                                >
                                    "GNU General Public License v3.0"
                                </a>
                            </p>
                        </div>
                    </div>

                    <div class="flex-shrink-0 px-6 py-4 bg-gray-50 dark:bg-gray-800/50 border-t border-gray-200 dark:border-gray-700 flex justify-end">
                        <button
                            on:click=move |_| set_is_open.set(false)
                            class="px-5 py-2 text-sm font-semibold text-white bg-cyan-600 hover:bg-cyan-700 rounded-lg shadow-sm transition-colors cursor-pointer"
                        >
                            "Close"
                        </button>
                    </div>
                </div>
            </div>
        })}
    }
}
