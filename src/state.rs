use crate::data::example::get_example_data;
use crate::models::{
    current_timestamp_secs, CvProject, PersistedState, ProjectsStore, ResumeData, TemplateType,
};
use crate::services::file_sync::LinkedFile;
use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

pub const PROJECTS_STORAGE_KEY: &str = "cvapp-projects";
pub const LEGACY_STORAGE_KEY: &str = "cvapp-storage";

pub fn detect_system_dark_mode() -> bool {
    web_sys::window()
        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .map(|m| m.matches())
        .unwrap_or(false)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ZustandWrapper {
    pub state: PersistedState,
    #[serde(default)]
    pub version: Option<u32>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum SyncStatus {
    LocalCache,
    Saving,
    Synced(String),
    Error(String),
}

#[derive(Clone, Copy)]
pub struct AppState {
    pub projects: RwSignal<Vec<CvProject>>,
    pub active_project_id: RwSignal<Option<String>>,
    pub is_home_view: RwSignal<bool>,
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
        let (initial_projects, initial_active_id, initial_state) = Self::load_or_migrate_projects();
        let sys_dark = detect_system_dark_mode();

        Self {
            projects: RwSignal::new(initial_projects),
            active_project_id: RwSignal::new(initial_active_id),
            is_home_view: RwSignal::new(true),
            resume_data: RwSignal::new(initial_state.resume_data),
            active_template: RwSignal::new(initial_state.active_template),
            dark_mode: RwSignal::new(initial_state.dark_mode.unwrap_or(sys_dark)),
            text_scale: RwSignal::new(initial_state.text_scale),
            spacing_scale: RwSignal::new(initial_state.spacing_scale),
            font_scale: RwSignal::new(initial_state.font_scale),
            accent_color: RwSignal::new(initial_state.accent_color),
            sidebar_color: RwSignal::new(initial_state.sidebar_color),
            font_family: RwSignal::new(initial_state.font_family),
            hide_photo: RwSignal::new(initial_state.hide_photo),
            linked_file: RwSignal::new(None),
            sync_status: RwSignal::new(SyncStatus::LocalCache),
            save_timer: RwSignal::new(None),
        }
    }

    fn load_or_migrate_projects() -> (Vec<CvProject>, Option<String>, PersistedState) {
        // 1. Try modern multi-project storage
        if let Ok(store) = LocalStorage::get::<ProjectsStore>(PROJECTS_STORAGE_KEY) {
            if !store.projects.is_empty() {
                let active_id = store
                    .active_project_id
                    .clone()
                    .filter(|id| store.projects.iter().any(|p| &p.id == id))
                    .or_else(|| store.projects.first().map(|p| p.id.clone()));

                let active_state = active_id
                    .as_ref()
                    .and_then(|id| store.projects.iter().find(|p| &p.id == id))
                    .map(|p| p.data.clone())
                    .unwrap_or_default();

                return (store.projects, active_id, active_state);
            }
        }

        // 2. Try migrating legacy single-resume storage
        if let Ok(wrapper) = LocalStorage::get::<ZustandWrapper>(LEGACY_STORAGE_KEY) {
            let legacy_state = wrapper.state;
            let project_name = if !legacy_state.resume_data.personal_info.full_name.trim().is_empty() {
                format!(
                    "{}'s Resume",
                    legacy_state.resume_data.personal_info.full_name.trim()
                )
            } else {
                "My Resume".to_string()
            };

            let project = CvProject::new(project_name, legacy_state.clone());
            let active_id = project.id.clone();
            let projects = vec![project];

            let store = ProjectsStore {
                active_project_id: Some(active_id.clone()),
                projects: projects.clone(),
            };
            let _ = LocalStorage::set(PROJECTS_STORAGE_KEY, store);

            return (projects, Some(active_id), legacy_state);
        }

        // 3. Brand new user: Seed with sample example project
        let example_data = get_example_data();
        let default_state = PersistedState {
            resume_data: example_data,
            active_template: TemplateType::Modern,
            dark_mode: None,
            ..Default::default()
        };

        let sample_project = CvProject::new("Sample Resume", default_state.clone());
        let active_id = sample_project.id.clone();
        let projects = vec![sample_project];

        let store = ProjectsStore {
            active_project_id: Some(active_id.clone()),
            projects: projects.clone(),
        };
        let _ = LocalStorage::set(PROJECTS_STORAGE_KEY, store);

        (projects, Some(active_id), default_state)
    }

    pub fn get_persisted_state(&self) -> PersistedState {
        PersistedState {
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
        }
    }

    pub fn get_active_project(&self) -> Option<CvProject> {
        let active_id = self.active_project_id.get()?;
        self.projects.with(|projs| {
            projs.iter().find(|p| p.id == active_id).cloned()
        })
    }

    pub fn get_active_project_name(&self) -> String {
        self.get_active_project()
            .map(|p| p.name)
            .unwrap_or_else(|| "Untitled Resume".to_string())
    }

    pub fn go_to_home(&self) {
        self.save_all();
        self.is_home_view.set(true);
    }

    pub fn open_project(&self, id: &str) {
        // Save current working state first
        self.save_all();

        let target_project = self.projects.with(|projs| {
            projs.iter().find(|p| p.id == id).cloned()
        });

        if let Some(project) = target_project {
            self.active_project_id.set(Some(id.to_string()));
            let s = project.data;
            self.resume_data.set(s.resume_data);
            self.active_template.set(s.active_template);
            if let Some(dm) = s.dark_mode {
                self.dark_mode.set(dm);
            }
            self.text_scale.set(s.text_scale);
            self.spacing_scale.set(s.spacing_scale);
            self.font_scale.set(s.font_scale);
            self.accent_color.set(s.accent_color);
            self.sidebar_color.set(s.sidebar_color);
            self.font_family.set(s.font_family);
            self.hide_photo.set(s.hide_photo);
            self.is_home_view.set(false);
            self.save_projects_to_storage();
        }
    }

    pub fn create_new_project(&self, name: &str, data: Option<PersistedState>) -> String {
        self.save_all();

        let state_data = data.unwrap_or_default();
        let project = CvProject::new(name, state_data);
        let new_id = project.id.clone();

        self.projects.update(|projs| {
            projs.insert(0, project);
        });

        self.open_project(&new_id);
        new_id
    }

    pub fn create_example_project(&self) -> String {
        let example_data = get_example_data();
        let state = PersistedState {
            resume_data: example_data,
            active_template: TemplateType::Modern,
            dark_mode: Some(self.dark_mode.get_untracked()),
            ..Default::default()
        };
        self.create_new_project("Example Resume", Some(state))
    }

    pub fn duplicate_project(&self, id: &str) -> Option<String> {
        self.save_all();

        let dup = self.projects.with(|projs| {
            projs.iter().find(|p| p.id == id).map(|p| p.duplicate())
        })?;

        let new_id = dup.id.clone();
        self.projects.update(|projs| {
            projs.insert(0, dup);
        });

        self.save_projects_to_storage();
        Some(new_id)
    }

    pub fn rename_project(&self, id: &str, new_name: &str) {
        let trimmed = new_name.trim();
        if trimmed.is_empty() {
            return;
        }

        self.projects.update(|projs| {
            if let Some(p) = projs.iter_mut().find(|p| p.id == id) {
                p.name = trimmed.to_string();
                p.updated_at = current_timestamp_secs();
            }
        });

        self.save_projects_to_storage();
    }

    pub fn delete_project(&self, id: &str) {
        let mut switched_id: Option<String> = None;

        self.projects.update(|projs| {
            projs.retain(|p| p.id != id);
            if projs.is_empty() {
                // If all projects deleted, create a fresh empty one
                let clean = CvProject::new("My Resume", PersistedState::default());
                switched_id = Some(clean.id.clone());
                projs.push(clean);
            }
        });

        let current_active = self.active_project_id.get_untracked();
        if current_active.as_deref() == Some(id) {
            if let Some(new_id) = switched_id {
                self.open_project(&new_id);
            } else if let Some(first) = self.projects.with(|p| p.first().map(|x| x.id.clone())) {
                self.open_project(&first);
            }
        } else {
            self.save_projects_to_storage();
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
        // 1. Sync active project data in memory
        let current_state = self.get_persisted_state();
        let now = current_timestamp_secs();

        if let Some(active_id) = self.active_project_id.get_untracked() {
            self.projects.update(|projs| {
                if let Some(p) = projs.iter_mut().find(|p| p.id == active_id) {
                    p.data = current_state.clone();
                    p.updated_at = now;
                }
            });
        }

        // 2. Persist projects store
        self.save_projects_to_storage();

        // 3. Persist legacy backup key
        let wrapper = ZustandWrapper {
            state: current_state,
            version: Some(0),
        };
        let _ = LocalStorage::set(LEGACY_STORAGE_KEY, wrapper);

        // 4. If a live local file is linked, stream update to disk
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
        self.schedule_save();
    }

    pub fn save_projects_to_storage(&self) {
        let store = ProjectsStore {
            active_project_id: self.active_project_id.get_untracked(),
            projects: self.projects.get_untracked(),
        };
        let _ = LocalStorage::set(PROJECTS_STORAGE_KEY, store);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zustand_wrapper_serialization() {
        let wrapper = ZustandWrapper {
            state: PersistedState::default(),
            version: Some(0),
        };
        let json = serde_json::to_string(&wrapper).unwrap();
        assert!(json.contains("\"activeTemplate\":\"ats\""));
    }

    #[test]
    fn test_sync_status_equality() {
        assert_eq!(SyncStatus::LocalCache, SyncStatus::LocalCache);
        assert_eq!(
            SyncStatus::Synced("test.json".to_string()),
            SyncStatus::Synced("test.json".to_string())
        );
        assert_ne!(
            SyncStatus::Synced("a.json".to_string()),
            SyncStatus::Synced("b.json".to_string())
        );
    }
}

