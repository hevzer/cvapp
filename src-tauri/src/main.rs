// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
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

  app_lib::run();
}
