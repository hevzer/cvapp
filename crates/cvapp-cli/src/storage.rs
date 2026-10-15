use cvapp::data::example::get_example_data;
use cvapp::models::{CvProject, PersistedState, ProjectsStore, TemplateType};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::time::Duration;

const APP_IPC_ADDR: &str = "127.0.0.1:38761";

#[derive(Serialize, Deserialize)]
struct LegacyZustandWrapper {
    state: PersistedState,
    version: Option<u32>,
}

/// Resolves the file path for the CVapp projects database.
pub fn resolve_store_path(explicit_path: Option<&Path>) -> PathBuf {
    if let Some(p) = explicit_path {
        return p.to_path_buf();
    }

    if let Ok(env_path) = env::var("CVAPP_FILE")
        && !env_path.trim().is_empty()
    {
        return PathBuf::from(env_path.trim());
    }

    // Check current directory and parents for .cvapp/projects.json or cvapp-projects.json
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut check_dir = current_dir.as_path();

    loop {
        let cvapp_dir_file = check_dir.join(".cvapp").join("projects.json");
        if cvapp_dir_file.is_file() {
            return cvapp_dir_file;
        }

        let flat_file = check_dir.join("cvapp-projects.json");
        if flat_file.is_file() {
            return flat_file;
        }

        // Also check for resume.json in project root
        let resume_json = check_dir.join("resume.json");
        if resume_json.is_file() {
            return resume_json;
        }

        if let Some(parent) = check_dir.parent() {
            check_dir = parent;
        } else {
            break;
        }
    }

    // Default to .cvapp/projects.json in current directory, or home config
    let default_local = current_dir.join(".cvapp").join("projects.json");
    if default_local.parent().map(|p| p.exists()).unwrap_or(false) {
        return default_local;
    }

    if let Ok(home) = env::var("HOME") {
        PathBuf::from(home)
            .join(".config")
            .join("cvapp")
            .join("projects.json")
    } else {
        default_local
    }
}

/// Loads the ProjectsStore from disk, automatically migrating legacy formats
/// or initializing default seed data if the file does not exist.
pub fn load_store(explicit_path: Option<&Path>) -> Result<(PathBuf, ProjectsStore), String> {
    let path = resolve_store_path(explicit_path);

    if !path.exists() {
        // Initialize a new default store
        let example_data = get_example_data();
        let default_state = PersistedState {
            resume_data: example_data,
            active_template: TemplateType::Modern,
            ..Default::default()
        };
        let initial_project = CvProject::new("Example Resume", default_state);
        let store = ProjectsStore {
            active_project_id: Some(initial_project.id.clone()),
            projects: vec![initial_project],
        };

        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let json = serde_json::to_string_pretty(&store).map_err(|e| e.to_string())?;
        fs::write(&path, json).map_err(|e| format!("Failed to create initial database at {}: {}", path.display(), e))?;

        return Ok((path, store));
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read database at {}: {}", path.display(), e))?;

    // Try parsing as ProjectsStore
    if let Ok(store) = serde_json::from_str::<ProjectsStore>(&content) {
        return Ok((path, store));
    }

    // Try parsing as single CvProject
    if let Ok(single_project) = serde_json::from_str::<CvProject>(&content) {
        let store = ProjectsStore {
            active_project_id: Some(single_project.id.clone()),
            projects: vec![single_project],
        };
        return Ok((path, store));
    }

    // Try parsing as legacy ZustandWrapper
    if let Ok(wrapper) = serde_json::from_str::<LegacyZustandWrapper>(&content) {
        let project = CvProject::new("My Resume", wrapper.state);
        let store = ProjectsStore {
            active_project_id: Some(project.id.clone()),
            projects: vec![project],
        };
        return Ok((path, store));
    }

    // Try parsing as raw PersistedState
    if let Ok(state) = serde_json::from_str::<PersistedState>(&content) {
        let project = CvProject::new("My Resume", state);
        let store = ProjectsStore {
            active_project_id: Some(project.id.clone()),
            projects: vec![project],
        };
        return Ok((path, store));
    }

    Err(format!("Unrecognized or invalid resume JSON format in {}", path.display()))
}

/// Persists the store to disk formatted with indentation.
pub fn save_store(path: &Path, store: &ProjectsStore) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let json = serde_json::to_string_pretty(store).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| format!("Failed to write store to {}: {}", path.display(), e))?;
    if notify_running_app(store) {
        eprintln!("Sent changes to the running CVapp desktop app.");
    }
    Ok(())
}

/// Best-effort push to a locally running Tauri desktop app. The CLI remains
/// usable when the app is closed; when it is open, the app applies this store
/// immediately through its localhost IPC listener.
fn notify_running_app(store: &ProjectsStore) -> bool {
    let Ok(addr) = APP_IPC_ADDR.parse::<SocketAddr>() else {
        return false;
    };
    let Ok(mut stream) = TcpStream::connect_timeout(&addr, Duration::from_millis(150)) else {
        return false;
    };
    let _ = stream.set_read_timeout(Some(Duration::from_millis(250)));
    let _ = stream.set_write_timeout(Some(Duration::from_millis(250)));

    if serde_json::to_writer(&mut stream, store).is_err()
        || stream.write_all(b"\n").is_err()
        || stream.flush().is_err()
    {
        return false;
    }

    // The acknowledgement is informational; a missing response must not make
    // an otherwise successful CLI database update fail.
    let mut ack = [0_u8; 3];
    matches!(stream.read_exact(&mut ack), Ok(()) if &ack == b"OK\n")
}

/// Finds a project by ID or case-insensitive name match.
/// If `query` is None, returns the active project or the first project.
pub fn find_project<'a>(store: &'a ProjectsStore, query: Option<&str>) -> Result<&'a CvProject, String> {
    if store.projects.is_empty() {
        return Err("No projects found in the database. Use 'cvapp project create' first.".to_string());
    }

    match query {
        Some(q) => {
            let q_trimmed = q.trim();
            // Try exact ID match
            if let Some(p) = store.projects.iter().find(|p| p.id == q_trimmed) {
                return Ok(p);
            }
            // Try prefix ID match
            if let Some(p) = store.projects.iter().find(|p| p.id.starts_with(q_trimmed)) {
                return Ok(p);
            }
            // Try case-insensitive name match
            let q_lower = q_trimmed.to_lowercase();
            if let Some(p) = store.projects.iter().find(|p| p.name.to_lowercase() == q_lower) {
                return Ok(p);
            }
            // Try case-insensitive candidate name match
            if let Some(p) = store.projects.iter().find(|p| p.candidate_name().to_lowercase().contains(&q_lower)) {
                return Ok(p);
            }

            Err(format!("Project '{}' not found. Available projects: {}", q, store.projects.iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", ")))
        }
        None => {
            if let Some(ref active_id) = store.active_project_id
                && let Some(p) = store.projects.iter().find(|p| p.id == *active_id)
            {
                return Ok(p);
            }
            store.projects.first().ok_or_else(|| "No projects found.".to_string())
        }
    }
}

/// Finds a mutable project by ID or case-insensitive name match.
pub fn find_project_mut<'a>(store: &'a mut ProjectsStore, query: Option<&str>) -> Result<&'a mut CvProject, String> {
    if store.projects.is_empty() {
        return Err("No projects found in the database. Use 'cvapp project create' first.".to_string());
    }

    let target_id = match query {
        Some(q) => {
            let q_trimmed = q.trim();
            if let Some(p) = store.projects.iter().find(|p| p.id == q_trimmed) {
                p.id.clone()
            } else if let Some(p) = store.projects.iter().find(|p| p.id.starts_with(q_trimmed)) {
                p.id.clone()
            } else {
                let q_lower = q_trimmed.to_lowercase();
                if let Some(p) = store.projects.iter().find(|p| p.name.to_lowercase() == q_lower) {
                    p.id.clone()
                } else if let Some(p) = store.projects.iter().find(|p| p.name.to_lowercase().contains(&q_lower)) {
                    p.id.clone()
                } else if let Some(p) = store.projects.iter().find(|p| p.candidate_name().to_lowercase().contains(&q_lower)) {
                    p.id.clone()
                } else {
                    return Err(format!("Project '{}' not found.", q));
                }
            }
        }
        None => {
            if let Some(ref active_id) = store.active_project_id {
                if store.projects.iter().any(|p| p.id == *active_id) {
                    active_id.clone()
                } else {
                    store.projects[0].id.clone()
                }
            } else {
                store.projects[0].id.clone()
            }
        }
    };

    store.projects.iter_mut().find(|p| p.id == target_id).ok_or_else(|| "Project not found.".to_string())
}
