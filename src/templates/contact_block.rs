use crate::i18n::get_dictionary;
use crate::models::PersonalInfo;
use leptos::prelude::*;

#[component]
pub fn ContactBlock(
    personal_info: PersonalInfo,
    #[prop(into)] lang: String,
    #[prop(optional, into)] list_class_name: Option<&'static str>,
    #[prop(optional, into)] row_class_name: Option<&'static str>,
    #[prop(optional, into)] icon_class_name: Option<&'static str>,
    #[prop(optional, into)] value_class_name: Option<&'static str>,
) -> impl IntoView {
    let dict = get_dictionary(&lang);
    let list_cls = list_class_name.unwrap_or("");
    let row_cls = row_class_name.unwrap_or("");
    let icon_cls = icon_class_name.unwrap_or("");
    let val_cls = value_class_name.unwrap_or("");

    struct Row {
        label: &'static str,
        icon: &'static str,
        value: String,
        break_all: bool,
    }

    let rows: Vec<Row> = vec![
        Row {
            label: "Email",
            icon: "bi-envelope",
            value: personal_info.email,
            break_all: true,
        },
        Row {
            label: "Phone",
            icon: "bi-telephone",
            value: personal_info.phone,
            break_all: false,
        },
        Row {
            label: "Location",
            icon: "bi-geo-alt",
            value: personal_info.location,
            break_all: false,
        },
        Row {
            label: "LinkedIn",
            icon: "bi-linkedin",
            value: personal_info.linkedin,
            break_all: true,
        },
        Row {
            label: "GitHub",
            icon: "bi-github",
            value: personal_info.github,
            break_all: true,
        },
        Row {
            label: "Website",
            icon: "bi-globe",
            value: personal_info.website,
            break_all: true,
        },
        Row {
            label: dict.driving_license,
            icon: "bi-car-front",
            value: personal_info.driving_license,
            break_all: false,
        },
    ];

    let visible: Vec<Row> = rows.into_iter().filter(|r| !r.value.is_empty()).collect();
    if visible.is_empty() {
        return view! { <span></span> }.into_any();
    }

    view! {
        <ul class=list_cls>
            {visible.into_iter().map(|r| {
                let val_classes = if r.break_all {
                    format!("{} break-all", val_cls)
                } else {
                    val_cls.to_string()
                };

                view! {
                    <li class=row_cls>
                        <span aria-hidden="true" class=icon_cls>
                            <i class=format!("bi {}", r.icon)></i>
                        </span>
                        <span class="sr-only">{format!("{}: ", r.label)}</span>
                        <span class=val_classes>{r.value}</span>
                    </li>
                }
            }).collect::<Vec<_>>()}
        </ul>
    }.into_any()
}
