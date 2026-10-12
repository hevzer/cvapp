use crate::date::format_date_numeric;
use crate::i18n::get_dictionary;
use crate::state::AppState;
use crate::templates::contact_block::ContactBlock;
use leptos::prelude::*;

#[component]
pub fn ModernCreative() -> impl IntoView {
    let state = expect_context::<AppState>();
    let resume = state.resume_data;
    let hide_photo = state.hide_photo;

    view! {
        {move || {
            let data = resume.get();
            let p = data.personal_info.clone();
            let lang = data.cv_language.clone();
            let dict = get_dictionary(&lang);

            let has_content = !p.full_name.is_empty()
                || !data.summary.is_empty()
                || !data.experience.is_empty()
                || !data.volunteering.is_empty()
                || !data.education.is_empty()
                || !data.certifications.is_empty()
                || !data.technical_skills.is_empty()
                || !data.soft_skills.is_empty()
                || !data.languages.is_empty()
                || !data.interests.is_empty();

            if !has_content {
                return view! {
                    <div class="flex items-center justify-center h-full text-gray-400 dark:text-gray-500 min-h-[400px]">
                        <div class="text-center">
                            <div class="text-5xl mb-4">"📄"</div>
                            <p class="text-lg font-medium">"Your CV preview will appear here"</p>
                            <p class="text-sm mt-1">"Start filling in the form on the left"</p>
                        </div>
                    </div>
                }.into_any();
            }

            view! {
                <article class="bg-white text-gray-900 max-w-[210mm] mx-auto text-[13px] leading-relaxed min-h-[297mm] max-h-[297mm] overflow-hidden flex">
                    {/* Left Sidebar */}
                    <aside class="w-[35%] text-white p-6 flex flex-col" style="background-color: var(--sidebar, #1e293b);">
                        {if !hide_photo.get() && !p.photo_url.is_empty() {
                            view! {
                                <div class="flex justify-center mb-5">
                                    <img src=p.photo_url.clone() alt=p.full_name.clone() class="w-24 h-24 rounded-full object-cover ring-3 ring-slate-600" />
                                </div>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}

                        <header class="mb-6">
                            {if !p.full_name.is_empty() {
                                view! { <h1 class="text-xl font-bold leading-tight">{p.full_name.clone()}</h1> }.into_any()
                            } else { view! { <span></span> }.into_any() }}
                            {if !p.title.is_empty() {
                                view! { <p class="text-sm text-slate-300 mt-1 font-medium">{p.title.clone()}</p> }.into_any()
                            } else { view! { <span></span> }.into_any() }}
                        </header>

                        <section class="mb-6">
                            <h2 class="text-xs font-bold uppercase tracking-widest text-slate-400 mb-3 border-b border-slate-600 pb-1.5">
                                {dict.contact}
                            </h2>
                            <ContactBlock
                                personal_info=p.clone()
                                lang=lang.clone()
                                list_class_name="space-y-2 list-none"
                                row_class_name="flex items-start gap-2"
                                icon_class_name="text-slate-400 mt-0.5 text-xs"
                                value_class_name="text-sm text-slate-200"
                            />
                        </section>

                        {if !data.summary.is_empty() {
                            view! {
                                <section class="mb-6">
                                    <h2 class="text-xs font-bold uppercase tracking-widest text-slate-400 mb-3 border-b border-slate-600 pb-1.5">
                                        {dict.profile}
                                    </h2>
                                    <p class="text-slate-300 text-xs leading-relaxed whitespace-pre-wrap">{data.summary}</p>
                                </section>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}

                        {if !data.soft_skills.is_empty() {
                            view! {
                                <section class="mb-6">
                                    <h2 class="text-xs font-bold uppercase tracking-widest text-slate-400 mb-3 border-b border-slate-600 pb-1.5">
                                        {dict.soft_skills}
                                    </h2>
                                    <ul class="space-y-1.5 list-none">
                                        {data.soft_skills.into_iter().map(|skill| {
                                            view! {
                                                <li class="text-xs text-slate-300 flex items-center gap-2">
                                                    <span aria-hidden="true"><i class="bi bi-check2 text-xs" style="color: var(--accent, #6366f1)"></i></span>
                                                    {skill}
                                                </li>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </ul>
                                </section>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}

                        {if !data.languages.is_empty() {
                            view! {
                                <section class="mb-6">
                                    <h2 class="text-xs font-bold uppercase tracking-widest text-slate-400 mb-3 border-b border-slate-600 pb-1.5">
                                        {dict.languages}
                                    </h2>
                                    <ul class="flex flex-wrap gap-1.5 list-none">
                                        {data.languages.into_iter().map(|l| {
                                            view! {
                                                <li class="px-2 py-0.5 bg-slate-700 text-slate-200 text-xs rounded font-medium">
                                                    {l}
                                                </li>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </ul>
                                </section>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}

                        {if !data.volunteering.is_empty() {
                            view! {
                                <section class="mb-6">
                                    <h2 class="text-xs font-bold uppercase tracking-widest text-slate-400 mb-3 border-b border-slate-600 pb-1.5">
                                        {dict.volunteering}
                                    </h2>
                                    <div class="space-y-3">
                                        {data.volunteering.into_iter().map(|v| {
                                            let s_date = format_date_numeric(&v.start_date);
                                            let e_date = if v.current { dict.present.to_string() } else { format_date_numeric(&v.end_date) };
                                            let has_dates = !v.start_date.is_empty() || !v.end_date.is_empty() || v.current;

                                            view! {
                                                <div>
                                                    <h3 class="text-[12px] font-bold text-slate-100 leading-tight">{v.role}</h3>
                                                    <p class="text-[11px] text-slate-300 font-medium">{v.organization}</p>
                                                    <p class="text-[10px] text-slate-400 mt-0.5">
                                                        {s_date}{if has_dates { " — " } else { "" }}{e_date}
                                                    </p>
                                                    {if !v.description.is_empty() {
                                                        view! { <p class="text-[10.5px] text-slate-300 leading-snug mt-1 whitespace-pre-wrap">{v.description}</p> }.into_any()
                                                    } else { view! { <span></span> }.into_any() }}
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </section>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}

                        {if !data.interests.is_empty() {
                            view! {
                                <section class="mb-6">
                                    <h2 class="text-xs font-bold uppercase tracking-widest text-slate-400 mb-3 border-b border-slate-600 pb-1.5">
                                        {dict.interests}
                                    </h2>
                                    <ul class="space-y-2 list-none">
                                        {data.interests.into_iter().map(|it| {
                                            view! {
                                                <li>
                                                    <h3 class="text-[12px] font-bold text-slate-100 leading-tight">{it.name}</h3>
                                                    {if !it.description.is_empty() {
                                                        view! { <p class="text-[10.5px] text-slate-300 leading-snug mt-0.5 whitespace-pre-wrap">{it.description}</p> }.into_any()
                                                    } else { view! { <span></span> }.into_any() }}
                                                </li>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </ul>
                                </section>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </aside>

                    {/* Right Main Content */}
                    <main class="w-[65%] p-8">
                        {if !p.objective.is_empty() {
                            view! {
                                <section class="mb-6">
                                    <p class="text-slate-600 font-medium italic text-sm leading-relaxed pl-4 py-1" style="border-left: 4px solid var(--accent, #6366f1)">
                                        "\""{p.objective}"\""
                                    </p>
                                </section>
                            }.into_any()
                        } else { view! { <span></span> }.into_any() }}

                        {if !data.technical_skills.is_empty() {
                            view! {
                                <section class="mb-6">
                                    <h2 class="text-sm font-bold uppercase tracking-wider text-slate-800 pb-1.5 mb-3" style="border-bottom: 2px solid var(--accent, #1e293b)">
                                        {dict.technical_skills}
                                    </h2>
                                    <ul class="grid grid-cols-2 gap-x-6 gap-y-4 list-none">
                                        {data.technical_skills.into_iter().map(|skill| {
                                            view! {
                                                <li>
                                                    <h3 class="font-bold text-gray-900 text-[13px]">{skill.name}</h3>
                                                    {if !skill.description.is_empty() {
                                                        view! { <p class="mt-1 text-gray-700 leading-relaxed whitespace-pre-wrap text-[12px]">{skill.description}</p> }.into_any()
                                                    } else { view! { <span></span> }.into_any() }}
                                                </li>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </ul>
                                </section>
                            }.into_any()
                        } else { view! { <span></span> }.into_any() }}

                        {if !data.experience.is_empty() {
                            view! {
                                <section class="mb-6">
                                    <h2 class="text-sm font-bold uppercase tracking-wider text-slate-800 pb-1.5 mb-3" style="border-bottom: 2px solid var(--accent, #1e293b)">
                                        {dict.experience}
                                    </h2>
                                    <div class="space-y-5">
                                        {data.experience.into_iter().map(|exp| {
                                            let s_date = format_date_numeric(&exp.start_date);
                                            let e_date = if exp.current { dict.present.to_string() } else { format_date_numeric(&exp.end_date) };
                                            let has_dates = !exp.start_date.is_empty() || !exp.end_date.is_empty() || exp.current;

                                            view! {
                                                <div>
                                                    <div class="flex items-start justify-between">
                                                        <div>
                                                            <h3 class="font-bold text-gray-900">{exp.position}</h3>
                                                            <p class="text-gray-600 font-medium text-sm">{exp.company}</p>
                                                        </div>
                                                        <span class="text-xs text-gray-500 whitespace-nowrap ml-4 mt-0.5 bg-gray-100 px-2 py-0.5 rounded">
                                                            {s_date}{if has_dates { " — " } else { "" }}{e_date}
                                                        </span>
                                                    </div>
                                                    {if !exp.description.is_empty() {
                                                        view! { <p class="mt-2 text-gray-700 leading-relaxed whitespace-pre-wrap">{exp.description}</p> }.into_any()
                                                    } else { view! { <span></span> }.into_any() }}
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </section>
                            }.into_any()
                        } else { view! { <span></span> }.into_any() }}

                        {if !data.education.is_empty() {
                            view! {
                                <section class="mb-6">
                                    <h2 class="text-sm font-bold uppercase tracking-wider text-slate-800 pb-1.5 mb-3" style="border-bottom: 2px solid var(--accent, #1e293b)">
                                        {dict.education}
                                    </h2>
                                    <div class="space-y-5">
                                        {data.education.into_iter().map(|edu| {
                                            let s_date = format_date_numeric(&edu.start_date);
                                            let e_date = if edu.end_date.is_empty() { dict.present.to_string() } else { format_date_numeric(&edu.end_date) };
                                            let has_dates = !edu.start_date.is_empty() || !edu.end_date.is_empty();

                                            view! {
                                                <div>
                                                    <div class="flex items-start justify-between">
                                                        <div>
                                                            <h3 class="font-bold text-gray-900">{edu.degree}</h3>
                                                            {if !edu.field.is_empty() {
                                                                view! { <p class="text-gray-700 font-medium text-sm mt-0.5">{edu.field}</p> }.into_any()
                                                            } else { view! { <span></span> }.into_any() }}
                                                            <p class="text-gray-600 font-medium text-[12.5px]">{edu.institution}</p>
                                                        </div>
                                                        <span class="text-xs text-gray-500 whitespace-nowrap ml-4 mt-0.5 bg-gray-100 px-2 py-0.5 rounded">
                                                            {s_date}{if has_dates { " — " } else { "" }}{e_date}
                                                        </span>
                                                    </div>
                                                    {if !edu.description.is_empty() {
                                                        view! { <p class="mt-2 text-gray-700 leading-relaxed whitespace-pre-wrap">{edu.description}</p> }.into_any()
                                                    } else { view! { <span></span> }.into_any() }}
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </section>
                            }.into_any()
                        } else { view! { <span></span> }.into_any() }}

                        {if !data.certifications.is_empty() {
                            view! {
                                <section class="mb-6">
                                    <h2 class="text-sm font-bold uppercase tracking-wider text-slate-800 pb-1.5 mb-3" style="border-bottom: 2px solid var(--accent, #1e293b)">
                                        {dict.certifications}
                                    </h2>
                                    <ul class="space-y-3 list-none">
                                        {data.certifications.into_iter().map(|cert| {
                                            view! {
                                                <li>
                                                    <div class="flex items-start justify-between">
                                                        <div>
                                                            <h3 class="font-bold text-gray-900">
                                                                {cert.name}
                                                                {if !cert.url.is_empty() {
                                                                    view! {
                                                                        <a href=cert.url.clone() target="_blank" rel="noopener noreferrer" class="ml-2 text-cyan-600 hover:text-cyan-800 break-all text-xs print:text-gray-900 print:no-underline">
                                                                            <span aria-hidden="true"><i class="bi bi-box-arrow-up-right print:hidden"></i></span>
                                                                            <span class="hidden print:inline">{format!(" ({})", cert.url)}</span>
                                                                        </a>
                                                                    }.into_any()
                                                                } else { view! { <span></span> }.into_any() }}
                                                            </h3>
                                                            <p class="text-gray-600 font-medium text-sm">{cert.issuer}</p>
                                                        </div>
                                                        <span class="text-xs text-gray-500 whitespace-nowrap ml-4 mt-0.5 bg-gray-100 px-2 py-0.5 rounded">
                                                            {format_date_numeric(&cert.date)}
                                                        </span>
                                                    </div>
                                                </li>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </ul>
                                </section>
                            }.into_any()
                        } else { view! { <span></span> }.into_any() }}
                    </main>
                </article>
            }.into_any()
        }}
    }
}
