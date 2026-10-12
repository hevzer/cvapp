use crate::data::example::get_example_data;
use crate::models::{ResumeData, TemplateType};
use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

const STORAGE_KEY: &str = "cvapp-storage";

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistedState {
    #[serde(default)]
    pub resume_data: ResumeData,
    #[serde(default)]
    pub active_template: TemplateType,
    #[serde(default)]
    pub dark_mode: bool,
    #[serde(default = "default_scale")]
    pub text_scale: f64,
    #[serde(default = "default_scale")]
    pub spacing_scale: f64,
    #[serde(default = "default_scale")]
    pub font_scale: f64,
    #[serde(default = "default_accent_color")]
    pub accent_color: String,
    #[serde(default = "default_sidebar_color")]
    pub sidebar_color: String,
    #[serde(default = "default_font_family")]
    pub font_family: String,
    #[serde(default)]
    pub hide_photo: bool,
}

fn default_scale() -> f64 {
    1.0
}
fn default_accent_color() -> String {
    "#6366f1".to_string()
}
fn default_sidebar_color() -> String {
    "#1e293b".to_string()
}
fn default_font_family() -> String {
    "Inter".to_string()
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ZustandWrapper {
    pub state: PersistedState,
    #[serde(default)]
    pub version: Option<u32>,
}

#[derive(Clone, Copy)]
pub struct AppState {
    pub resume_data: RwSignal<ResumeData>,
    pub active_template: RwSignal<TemplateType>,
    pub dark_mode: RwSignal<bool>,
    pub text_scale: RwSignal<f64>,
    pub spacing_scale: RwSignal<f64>,
    pub font_scale: RwSignal<f64>,
    pub accent_color: RwSignal<String>,
    pub sidebar_color: RwSignal<String>,
    pub font_family: RwSignal<String>,
    pub hide_photo: RwSignal<bool>,
}

impl AppState {
    pub fn new() -> Self {
        let (initial_data, template, dark, t_scale, s_scale, f_scale, accent, sidebar, font, photo) =
            if let Ok(wrapper) = LocalStorage::get::<ZustandWrapper>(STORAGE_KEY) {
                let s = wrapper.state;
                (
                    s.resume_data,
                    s.active_template,
                    s.dark_mode,
                    s.text_scale,
                    s.spacing_scale,
                    s.font_scale,
                    s.accent_color,
                    s.sidebar_color,
                    s.font_family,
                    s.hide_photo,
                )
            } else {
                (
                    ResumeData::default(),
                    TemplateType::Ats,
                    false,
                    1.0,
                    1.0,
                    1.0,
                    "#6366f1".to_string(),
                    "#1e293b".to_string(),
                    "Inter".to_string(),
                    false,
                )
            };

        Self {
            resume_data: RwSignal::new(initial_data),
            active_template: RwSignal::new(template),
            dark_mode: RwSignal::new(dark),
            text_scale: RwSignal::new(t_scale),
            spacing_scale: RwSignal::new(s_scale),
            font_scale: RwSignal::new(f_scale),
            accent_color: RwSignal::new(accent),
            sidebar_color: RwSignal::new(sidebar),
            font_family: RwSignal::new(font),
            hide_photo: RwSignal::new(photo),
        }
    }

    pub fn save_to_storage(&self) {
        let wrapper = ZustandWrapper {
            state: PersistedState {
                resume_data: self.resume_data.get(),
                active_template: self.active_template.get(),
                dark_mode: self.dark_mode.get(),
                text_scale: self.text_scale.get(),
                spacing_scale: self.spacing_scale.get(),
                font_scale: self.font_scale.get(),
                accent_color: self.accent_color.get(),
                sidebar_color: self.sidebar_color.get(),
                font_family: self.font_family.get(),
                hide_photo: self.hide_photo.get(),
            },
            version: Some(0),
        };
        let _ = LocalStorage::set(STORAGE_KEY, wrapper);
    }

    pub fn load_example(&self) {
        self.resume_data.set(get_example_data());
        self.save_to_storage();
    }

    pub fn clear_all(&self) {
        self.resume_data.set(ResumeData::default());
        self.save_to_storage();
    }
}
