use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const CLI_IPC_ADDR: &str = "127.0.0.1:38761";
const MAX_IPC_MESSAGE_BYTES: u64 = 8 * 1024 * 1024;

use tauri::Manager;

#[derive(Clone)]
struct PendingProjectUpdate(Arc<Mutex<Option<String>>>);

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OpenFileResponse {
  pub path: String,
  pub name: String,
  pub content: String,
}

#[tauri::command]
fn pick_save_file(suggested_name: Option<String>) -> Option<String> {
  let mut dialog = rfd::FileDialog::new().add_filter("JSON", &["json"]);
  if let Some(ref name) = suggested_name {
    dialog = dialog.set_file_name(name);
  }
  dialog.save_file().map(|p| p.to_string_lossy().to_string())
}

#[tauri::command]
fn pick_open_file() -> Result<Option<OpenFileResponse>, String> {
  if let Some(path_buf) = rfd::FileDialog::new().add_filter("JSON", &["json"]).pick_file() {
    let path = path_buf.to_string_lossy().to_string();
    let name = path_buf
      .file_name()
      .map(|n| n.to_string_lossy().to_string())
      .unwrap_or_else(|| "resume.json".to_string());
    let content = std::fs::read_to_string(&path_buf).map_err(|e| e.to_string())?;
    Ok(Some(OpenFileResponse { path, name, content }))
  } else {
    Ok(None)
  }
}

#[tauri::command]
fn write_file_content(path: String, content: String) -> Result<(), String> {
  std::fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
fn take_cli_project_update(state: tauri::State<'_, PendingProjectUpdate>) -> Option<String> {
  state.0.lock().ok()?.take()
}

fn handle_cli_connection(mut stream: TcpStream, pending: &Arc<Mutex<Option<String>>>) {
  let _ = stream.set_read_timeout(Some(Duration::from_secs(2)));
  let _ = stream.set_write_timeout(Some(Duration::from_secs(1)));

  let mut content = String::new();
  let read_result = {
    let reader = BufReader::new(&mut stream);
    reader.take(MAX_IPC_MESSAGE_BYTES + 1).read_line(&mut content)
  };
  let valid_store = read_result.is_ok()
    && content.len() as u64 <= MAX_IPC_MESSAGE_BYTES
    && serde_json::from_str::<serde_json::Value>(&content)
      .ok()
      .and_then(|value| value.get("projects").and_then(serde_json::Value::as_array).cloned())
      .is_some_and(|projects| !projects.is_empty());

  if valid_store
    && let Ok(mut queued) = pending.lock()
  {
    *queued = Some(content);
    let _ = stream.write_all(b"OK\n");
    let _ = stream.flush();
    return;
  }
  let _ = stream.write_all(b"ERR\n");
  let _ = stream.flush();
}

fn start_cli_ipc_listener(pending: Arc<Mutex<Option<String>>>) {
  std::thread::Builder::new()
    .name("cvapp-cli-ipc".to_string())
    .spawn(move || {
      let listener = match TcpListener::bind(CLI_IPC_ADDR) {
        Ok(listener) => listener,
        Err(error) => {
          log::warn!("Could not start CLI localhost connection on {CLI_IPC_ADDR}: {error}");
          return;
        }
      };
      log::info!("CLI localhost connection listening on {CLI_IPC_ADDR}");
      for connection in listener.incoming() {
        match connection {
          Ok(stream) => handle_cli_connection(stream, &pending),
          Err(error) => log::warn!("CLI localhost connection failed: {error}"),
        }
      }
    })
    .expect("failed to start CLI localhost connection thread");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  #[cfg(target_os = "linux")]
  {
    // Prevent WebKitGTK Wayland display protocol error (Error 71) and GPU compositing crashes
    if std::env::var("WEBKIT_DISABLE_DMABUF_RENDERER").is_err() {
      // SAFETY: Called on process initialization before spawning threads
      unsafe {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
      }
    }
    if std::env::var("WEBKIT_DISABLE_COMPOSITING_MODE").is_err() {
      // SAFETY: Called on process initialization before spawning threads
      unsafe {
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
      }
    }
  }

  tauri::Builder::default()
    .plugin(tauri_plugin_log::Builder::default().build())
    .invoke_handler(tauri::generate_handler![
      pick_save_file,
      pick_open_file,
      write_file_content,
      take_cli_project_update
    ])
    .setup(|app| {
      let pending = Arc::new(Mutex::new(None));
      app.manage(PendingProjectUpdate(pending.clone()));
      start_cli_ipc_listener(pending);
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while building tauri application");
}
