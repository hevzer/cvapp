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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  #[cfg(target_os = "linux")]
  {
    // Prevent WebKitGTK Wayland display protocol error (Error 71)
    if std::env::var("WEBKIT_DISABLE_DMABUF_RENDERER").is_err() {
      // SAFETY: Called on process initialization before spawning threads
      unsafe {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
      }
    }
  }

  tauri::Builder::default()
    .plugin(tauri_plugin_log::Builder::default().build())
    .invoke_handler(tauri::generate_handler![
      pick_save_file,
      pick_open_file,
      write_file_content
    ])
    .setup(|_app| Ok(()))
    .run(tauri::generate_context!())
    .expect("error while building tauri application");
}
