use crate::date::format_date_numeric;
use crate::i18n::get_dictionary;
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn ATSMinimalist() -> impl IntoView {
    let state = expect_context::<AppState>();
    let resume = state.resume_data;
    let hide_photo = state.hide_photo;

    view! {
        {move || {
            let data = resume.get();
            let p = data.personal_info;
            let lang = data.cv_language;
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

            let accent_header = "text-sm font-bold uppercase tracking-wider pb-1 mb-2";
            let accent_header_style = "color: var(--accent, #1e293b); border-bottom: 1px solid var(--accent, #d1d5db);";

            view! {
                <article class="bg-white text-gray-900 p-8 max-w-[210mm] mx-auto text-[13px] leading-relaxed min-h-[297mm] max-h-[297mm] overflow-hidden">
                    {/* Header */}
                    <header class="text-center pb-4 mb-5" style="border-bottom: 2px solid var(--accent, #1e293b);">
                        {if !hide_photo.get() && !p.photo_url.is_empty() {
                            view! {
                                <div class="flex justify-center mb-3">
                                    <img src=p.photo_url alt=p.full_name.clone() class="w-16 h-16 rounded-full object-cover" />
                                </div>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}

                        {if !p.full_name.is_empty() {
                            view! {
                                <h1 class="text-2xl font-bold tracking-wide uppercase text-gray-900">
                                    {p.full_name.clone()}
                                </h1>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}

                        {if !p.title.is_empty() {
                            view! {
                                <p class="text-sm font-medium text-gray-600 mt-1 tracking-wider uppercase">
                                    {p.title.clone()}
                                </p>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}

                        <ul class="flex flex-wrap items-center justify-center gap-x-4 gap-y-1 mt-3 text-xs text-gray-600 list-none">
                            {if !p.email.is_empty() {
                                view! { <li><span class="sr-only">"Email: "</span>{p.email}</li> }.into_any()
                            } else { view! { <span></span> }.into_any() }}
                            {if !p.phone.is_empty() {
                                view! { <li><span class="sr-only">"Phone: "</span>{p.phone}</li> }.into_any()
                            } else { view! { <span></span> }.into_any() }}
                            {if !p.location.is_empty() {
                                view! { <li><span class="sr-only">"Location: "</span>{p.location}</li> }.into_any()
                            } else { view! { <span></span> }.into_any() }}
                            {if !p.linkedin.is_empty() {
                                view! { <li><span class="sr-only">"LinkedIn: "</span>{p.linkedin}</li> }.into_any()
                            } else { view! { <span></span> }.into_any() }}
                            {if !p.github.is_empty() {
                                view! { <li><span class="sr-only">"GitHub: "</span>{p.github}</li> }.into_any()
                            } else { view! { <span></span> }.into_any() }}
                            {if !p.website.is_empty() {
                                view! { <li><span class="sr-only">"Website: "</span>{p.website}</li> }.into_any()
                            } else { view! { <span></span> }.into_any() }}
                            {if !p.driving_license.is_empty() {
                                view! { <li>{dict.driving_license}": "{p.driving_license}</li> }.into_any()
                            } else { view! { <span></span> }.into_any() }}
                        </ul>
                    </header>

                    {/* Objective */}
                    {if !p.objective.is_empty() {
                        view! {
                            <section class="mb-5 text-center">
                                <p class="text-gray-800 font-medium italic">"\""{p.objective}"\""</p>
                            </section>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}

                    {/* Summary */}
                    {if !data.summary.is_empty() {
                        view! {
                            <section class="mb-5">
                                <h2 class=accent_header style=accent_header_style>
                                    {dict.profile}
                                </h2>
                                <p class="text-gray-700 leading-relaxed whitespace-pre-wrap">{data.summary}</p>
                            </section>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}

                    {/* Experience */}
                    {if !data.experience.is_empty() {
                        view! {
                            <section class="mb-5">
                                <h2 class="text-sm font-bold uppercase tracking-wider pb-1 mb-3" style=accent_header_style>
                                    {dict.experience}
                                </h2>
                                <div class="space-y-4">
                                    {data.experience.into_iter().map(|exp| {
                                        let s_date = format_date_numeric(&exp.start_date);
                                        let e_date = if exp.current {
                                            dict.present.to_string()
                                        } else {
                                            format_date_numeric(&exp.end_date)
                                        };
                                        let has_dates = !exp.start_date.is_empty() || !exp.end_date.is_empty() || exp.current;

                                        view! {
                                            <div>
                                                <div class="flex items-start justify-between">
                                                    <div>
                                                        <h3 class="font-bold text-gray-900">{exp.position}</h3>
                                                        <p class="text-gray-600 font-medium">{exp.company}</p>
                                                    </div>
                                                    <span class="text-gray-600 min-w-[130px] text-right">
                                                        {s_date}
                                                        {if has_dates { " — " } else { "" }}
                                                        {e_date}
                                                    </span>
                                                </div>
                                                {if !exp.description.is_empty() {
                                                    view! {
                                                        <p class="mt-1.5 text-gray-700 leading-relaxed whitespace-pre-wrap">{exp.description}</p>
                                                    }.into_any()
                                                } else {
                                                    view! { <span></span> }.into_any()
                                                }}
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </section>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}

                    {/* Education */}
                    {if !data.education.is_empty() {
                        view! {
                            <section class="mb-5">
                                <h2 class="text-sm font-bold uppercase tracking-wider text-gray-800 border-b border-gray-300 pb-1 mb-3">
                                    {dict.education}
                                </h2>
                                <div class="space-y-3">
                                    {data.education.into_iter().map(|edu| {
                                        let s_date = format_date_numeric(&edu.start_date);
                                        let e_date = if edu.end_date.is_empty() {
                                            dict.present.to_string()
                                        } else {
                                            format_date_numeric(&edu.end_date)
                                        };
                                        let has_dates = !edu.start_date.is_empty() || !edu.end_date.is_empty();

                                        view! {
                                            <div>
                                                <div class="flex items-start justify-between">
                                                    <div>
                                                        <h3 class="font-bold text-gray-900">{edu.degree}</h3>
                                                        {if !edu.field.is_empty() {
                                                            view! { <p class="text-gray-700 text-sm font-medium">{edu.field}</p> }.into_any()
                                                        } else {
                                                            view! { <span></span> }.into_any()
                                                        }}
                                                        <p class="text-gray-600 font-medium">{edu.institution}</p>
                                                    </div>
                                                    <span class="text-gray-600 min-w-[130px] text-right">
                                                        {s_date}
                                                        {if has_dates { " — " } else { "" }}
                                                        {e_date}
                                                    </span>
                                                </div>
                                                {if !edu.description.is_empty() {
                                                    view! {
                                                        <p class="mt-1 text-gray-700 leading-relaxed whitespace-pre-wrap">{edu.description}</p>
                                                    }.into_any()
                                                } else {
                                                    view! { <span></span> }.into_any()
                                                }}
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </section>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}

                    {/* Certifications */}
                    {if !data.certifications.is_empty() {
                        view! {
                            <section class="mb-5">
                                <h2 class="text-sm font-bold uppercase tracking-wider text-gray-800 border-b border-gray-300 pb-1 mb-3">
                                    {dict.certifications}
                                </h2>
                                <div class="space-y-3">
                                    {data.certifications.into_iter().map(|cert| {
                                        view! {
                                            <div>
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
                                                            } else {
                                                                view! { <span></span> }.into_any()
                                                            }}
                                                        </h3>
                                                        <p class="text-gray-600 font-medium">{cert.issuer}</p>
                                                    </div>
                                                    <span class="text-gray-600 min-w-[130px] text-right">
                                                        {format_date_numeric(&cert.date)}
                                                    </span>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </section>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}

                    {/* Volunteering */}
                    {if !data.volunteering.is_empty() {
                        view! {
                            <section class="mb-5">
                                <h2 class="text-sm font-bold uppercase tracking-wider pb-1 mb-3" style=accent_header_style>
                                    {dict.volunteering}
                                </h2>
                                <div class="space-y-4">
                                    {data.volunteering.into_iter().map(|v| {
                                        let s_date = format_date_numeric(&v.start_date);
                                        let e_date = if v.current {
                                            dict.present.to_string()
                                        } else {
                                            format_date_numeric(&v.end_date)
                                        };
                                        let has_dates = !v.start_date.is_empty() || !v.end_date.is_empty() || v.current;

                                        view! {
                                            <div>
                                                <div class="flex items-start justify-between">
                                                    <div>
                                                        <h3 class="font-bold text-gray-900">{v.role}</h3>
                                                        <p class="text-gray-600 font-medium">{v.organization}</p>
                                                    </div>
                                                    <span class="text-gray-600 min-w-[130px] text-right">
                                                        {s_date}
                                                        {if has_dates { " — " } else { "" }}
                                                        {e_date}
                                                    </span>
                                                </div>
                                                {if !v.description.is_empty() {
                                                    view! {
                                                        <p class="mt-1.5 text-gray-700 leading-relaxed whitespace-pre-wrap">{v.description}</p>
                                                    }.into_any()
                                                } else {
                                                    view! { <span></span> }.into_any()
                                                }}
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </section>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}

                    {/* Technical Skills */}
                    {if !data.technical_skills.is_empty() {
                        view! {
                            <section class="mb-4">
                                <h2 class=accent_header style=accent_header_style>
                                    {dict.technical_skills}
                                </h2>
                                <ul class="flex flex-wrap gap-x-3 gap-y-1 text-gray-700 list-none">
                                    {data.technical_skills.into_iter().map(|s| {
                                        view! { <li>{s.name}</li> }
                                    }).collect::<Vec<_>>()}
                                </ul>
                            </section>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}

                    {/* Soft Skills */}
                    {if !data.soft_skills.is_empty() {
                        view! {
                            <section class="mb-5">
                                <h2 class=accent_header style=accent_header_style>
                                    {dict.soft_skills}
                                </h2>
                                <ul class="flex flex-wrap gap-x-3 gap-y-1 text-gray-700 list-none">
                                    {data.soft_skills.into_iter().map(|s| {
                                        view! { <li>{s}</li> }
                                    }).collect::<Vec<_>>()}
                                </ul>
                            </section>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}

                    {/* Languages */}
                    {if !data.languages.is_empty() {
                        view! {
                            <section class="mb-5">
                                <h2 class=accent_header style=accent_header_style>
                                    {dict.languages}
                                </h2>
                                <ul class="flex flex-wrap gap-x-3 gap-y-1 text-gray-700 list-none">
                                    {data.languages.into_iter().map(|l| {
                                        view! { <li>{l}</li> }
                                    }).collect::<Vec<_>>()}
                                </ul>
                            </section>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}

                    {/* Interests */}
                    {if !data.interests.is_empty() {
                        view! {
                            <section class="mb-5">
                                <h2 class="text-sm font-bold uppercase tracking-wider pb-1 mb-3" style=accent_header_style>
                                    {dict.interests}
                                </h2>
                                <div class="space-y-2">
                                    {data.interests.into_iter().map(|it| {
                                        view! {
                                            <div>
                                                <h3 class="font-bold text-gray-900">{it.name}</h3>
                                                {if !it.description.is_empty() {
                                                    view! {
                                                        <p class="text-gray-700 leading-relaxed whitespace-pre-wrap">{it.description}</p>
                                                    }.into_any()
                                                } else {
                                                    view! { <span></span> }.into_any()
                                                }}
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </section>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </article>
            }.into_any()
        }}
    }
}
