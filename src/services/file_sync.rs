use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[derive(Clone, PartialEq, Debug)]
pub enum LinkedFile {
    DesktopPath { path: String, name: String },
    WebHandle { handle: JsValue, name: String },
}

impl LinkedFile {
    pub fn name(&self) -> &str {
        match self {
            LinkedFile::DesktopPath { name, .. } => name,
            LinkedFile::WebHandle { name, .. } => name,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TauriOpenFileResult {
    pub path: String,
    pub name: String,
    pub content: String,
}

#[wasm_bindgen(inline_js = r#"
export function isTauriEnv() {
    return typeof window !== 'undefined' && (Boolean(window.__TAURI_INTERNALS__) || Boolean(window.__TAURI__));
}

export function isFileSystemAccessSupported() {
    return typeof window !== 'undefined' && 'showSaveFilePicker' in window;
}

export async function tauriInvoke(cmd, args) {
    if (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) {
        return await window.__TAURI__.core.invoke(cmd, args || {});
    }
    if (window.__TAURI_INTERNALS__ && window.__TAURI_INTERNALS__.invoke) {
        return await window.__TAURI_INTERNALS__.invoke(cmd, args || {});
    }
    throw new Error('Tauri IPC is not available');
}

export async function webPickSaveFile(suggestedName) {
    if (!('showSaveFilePicker' in window)) {
        throw new Error('File System Access API is not supported in this browser');
    }
    const handle = await window.showSaveFilePicker({
        suggestedName: suggestedName || 'cvapp-data.json',
        types: [{
            description: 'JSON Files',
            accept: { 'application/json': ['.json'] }
        }]
    });
    return handle;
}

export async function webPickOpenFile() {
    if (!('showOpenFilePicker' in window)) {
        throw new Error('File System Access API is not supported in this browser');
    }
    const [handle] = await window.showOpenFilePicker({
        types: [{
            description: 'JSON Files',
            accept: { 'application/json': ['.json'] }
        }]
    });
    const file = await handle.getFile();
    const content = await file.text();
    return { handle, name: handle.name, content };
}

export async function webWriteFile(handle, content) {
    const writable = await handle.createWritable();
    await writable.write(content);
    await writable.close();
}

export function getHandleName(handle) {
    return handle ? handle.name : 'resume.json';
}
"#)]
extern "C" {
    #[wasm_bindgen(js_name = isTauriEnv)]
    pub fn is_tauri_env() -> bool;

    #[wasm_bindgen(js_name = isFileSystemAccessSupported)]
    pub fn is_file_system_access_supported() -> bool;

    #[wasm_bindgen(js_name = tauriInvoke)]
    fn js_tauri_invoke(cmd: &str, args: &JsValue) -> js_sys::Promise;

    #[wasm_bindgen(js_name = webPickSaveFile)]
    fn js_web_pick_save_file(suggested_name: &str) -> js_sys::Promise;

    #[wasm_bindgen(js_name = webPickOpenFile)]
    fn js_web_pick_open_file() -> js_sys::Promise;

    #[wasm_bindgen(js_name = webWriteFile)]
    fn js_web_write_file(handle: &JsValue, content: &str) -> js_sys::Promise;

    #[wasm_bindgen(js_name = getHandleName)]
    fn js_get_handle_name(handle: &JsValue) -> String;
}

pub fn is_direct_file_sync_supported() -> bool {
    is_tauri_env() || is_file_system_access_supported()
}

pub async fn link_new_file(suggested_name: &str) -> Result<LinkedFile, String> {
    if is_tauri_env() {
        let args_obj = js_sys::Object::new();
        let _ = js_sys::Reflect::set(
            &args_obj,
            &JsValue::from_str("suggestedName"),
            &JsValue::from_str(suggested_name),
        );
        let promise = js_tauri_invoke("pick_save_file", &args_obj.into());
        let val = JsFuture::from(promise)
            .await
            .map_err(|e| format!("{:?}", e))?;

        if val.is_null() || val.is_undefined() {
            return Err("User cancelled file selection".to_string());
        }

        if let Some(path) = val.as_string() {
            let name = path
                .rsplit(['/', '\\'])
                .next()
                .unwrap_or("resume.json")
                .to_string();
            Ok(LinkedFile::DesktopPath { path, name })
        } else {
            Err("Failed to get chosen path from desktop dialog".to_string())
        }
    } else if is_file_system_access_supported() {
        let promise = js_web_pick_save_file(suggested_name);
        let handle = JsFuture::from(promise)
            .await
            .map_err(|e| format!("{:?}", e))?;
        let name = js_get_handle_name(&handle);
        Ok(LinkedFile::WebHandle { handle, name })
    } else {
        Err(
            "Live file linking is not supported in this browser. Use 'Save Data' to download."
                .to_string(),
        )
    }
}

pub async fn open_and_link_file() -> Result<(LinkedFile, String), String> {
    if is_tauri_env() {
        let args_obj = js_sys::Object::new();
        let promise = js_tauri_invoke("pick_open_file", &args_obj.into());
        let val = JsFuture::from(promise)
            .await
            .map_err(|e| format!("{:?}", e))?;

        if val.is_null() || val.is_undefined() {
            return Err("User cancelled file selection".to_string());
        }

        let json_str = js_sys::JSON::stringify(&val)
            .map_err(|e| format!("{:?}", e))?
            .as_string()
            .ok_or_else(|| "Failed to stringify response".to_string())?;

        let res: TauriOpenFileResult = serde_json::from_str(&json_str)
            .map_err(|e| format!("Failed to parse file response: {:?}", e))?;

        let linked = LinkedFile::DesktopPath {
            path: res.path,
            name: res.name,
        };
        Ok((linked, res.content))
    } else if is_file_system_access_supported() {
        let promise = js_web_pick_open_file();
        let val = JsFuture::from(promise)
            .await
            .map_err(|e| format!("{:?}", e))?;

        let handle = js_sys::Reflect::get(&val, &JsValue::from_str("handle"))
            .map_err(|e| format!("{:?}", e))?;
        let name = js_sys::Reflect::get(&val, &JsValue::from_str("name"))
            .map_err(|e| format!("{:?}", e))?
            .as_string()
            .unwrap_or_else(|| "resume.json".to_string());
        let content = js_sys::Reflect::get(&val, &JsValue::from_str("content"))
            .map_err(|e| format!("{:?}", e))?
            .as_string()
            .unwrap_or_default();

        let linked = LinkedFile::WebHandle { handle, name };
        Ok((linked, content))
    } else {
        Err(
            "Live file linking is not supported in this browser. Use 'Load Data' to open."
                .to_string(),
        )
    }
}

pub async fn write_to_linked_file(linked: &LinkedFile, content: &str) -> Result<(), String> {
    match linked {
        LinkedFile::DesktopPath { path, .. } => {
            let args_obj = js_sys::Object::new();
            let _ = js_sys::Reflect::set(
                &args_obj,
                &JsValue::from_str("path"),
                &JsValue::from_str(path),
            );
            let _ = js_sys::Reflect::set(
                &args_obj,
                &JsValue::from_str("content"),
                &JsValue::from_str(content),
            );
            let promise = js_tauri_invoke("write_file_content", &args_obj.into());
            JsFuture::from(promise)
                .await
                .map_err(|e| format!("{:?}", e))?;
            Ok(())
        }
        LinkedFile::WebHandle { handle, .. } => {
            let promise = js_web_write_file(handle, content);
            JsFuture::from(promise)
                .await
                .map_err(|e| format!("{:?}", e))?;
            Ok(())
        }
    }
}
