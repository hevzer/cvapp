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
    pub dark_mode: Option<bool>,
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

pub fn detect_system_dark_mode() -> bool {
    web_sys::window()
        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .map(|m| m.matches())
        .unwrap_or(false)
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

use crate::services::file_sync::LinkedFile;

#[derive(Clone, PartialEq, Debug)]
pub enum SyncStatus {
    LocalCache,
    Saving,
    Synced(String),
    Error(String),
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
    pub linked_file: RwSignal<Option<LinkedFile>>,
    pub sync_status: RwSignal<SyncStatus>,
    pub save_timer: RwSignal<Option<i32>>,
}

impl AppState {
    pub fn new() -> Self {
        let (initial_data, template, dark, t_scale, s_scale, f_scale, accent, sidebar, font, photo) =
            if let Ok(wrapper) = LocalStorage::get::<ZustandWrapper>(STORAGE_KEY) {
                let s = wrapper.state;
                (
                    s.resume_data,
                    s.active_template,
                    s.dark_mode.unwrap_or_else(detect_system_dark_mode),
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
                    detect_system_dark_mode(),
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
            linked_file: RwSignal::new(None),
            sync_status: RwSignal::new(SyncStatus::LocalCache),
            save_timer: RwSignal::new(None),
        }
    }

    pub fn schedule_save(&self) {
        if let Some(w) = web_sys::window() {
            if let Some(handle) = self.save_timer.get_untracked() {
                w.clear_timeout_with_handle(handle);
            }
            use wasm_bindgen::JsCast;
            let this = *self;
            let cb = wasm_bindgen::closure::Closure::once(move || {
                this.save_timer.set(None);
                this.save_all();
            });
            if let Ok(handle) = w.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                300,
            ) {
                self.save_timer.set(Some(handle));
            }
            cb.forget();
        } else {
            self.save_all();
        }
    }

    pub fn save_all(&self) {
        self.save_to_storage();

        if let Some(linked) = self.linked_file.get_untracked() {
            let status = self.sync_status;
            status.set(SyncStatus::Saving);
            let data = self.resume_data.get_untracked();
            if let Ok(json_str) = serde_json::to_string_pretty(&data) {
                wasm_bindgen_futures::spawn_local(async move {
                    match crate::services::file_sync::write_to_linked_file(&linked, &json_str).await {
                        Ok(_) => {
                            status.set(SyncStatus::Synced(linked.name().to_string()));
                        }
                        Err(e) => {
                            status.set(SyncStatus::Error(e));
                        }
                    }
                });
            }
        } else {
            self.sync_status.set(SyncStatus::LocalCache);
        }
    }

    pub fn save_to_storage(&self) {
        let wrapper = ZustandWrapper {
            state: PersistedState {
                resume_data: self.resume_data.get_untracked(),
                active_template: self.active_template.get_untracked(),
                dark_mode: Some(self.dark_mode.get_untracked()),
                text_scale: self.text_scale.get_untracked(),
                spacing_scale: self.spacing_scale.get_untracked(),
                font_scale: self.font_scale.get_untracked(),
                accent_color: self.accent_color.get_untracked(),
                sidebar_color: self.sidebar_color.get_untracked(),
                font_family: self.font_family.get_untracked(),
                hide_photo: self.hide_photo.get_untracked(),
            },
            version: Some(0),
        };
        let _ = LocalStorage::set(STORAGE_KEY, wrapper);
    }

    pub fn load_example(&self) {
        self.resume_data.set(get_example_data());
        self.schedule_save();
    }

    pub fn clear_all(&self) {
        self.resume_data.set(ResumeData::default());
        self.schedule_save();
    }
}
