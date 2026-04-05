use std::fs;
use std::path::{Path, PathBuf};
use std::{env, fs as std_fs};

use ignore::WalkBuilder;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};

use crate::services::project::{AppState, PupConfig};
use crate::services::spec_io::read_openapi_value;

const PUP_DIR_NAME: &str = ".pup";
const PUP_CONFIG_NAME: &str = "config.json";
const SKIP_DIRECTORY_NAMES: [&str; 8] = [
    "node_modules",
    ".git",
    "target",
    "dist",
    "build",
    ".next",
    ".svelte-kit",
    ".turbo",
];

#[derive(Debug, Serialize)]
pub struct OpenApiFile {
    pub file: String,
    pub title: Option<String>,
    pub version: Option<String>,
}

fn should_include_entry(path: &Path, is_dir: Option<bool>) -> bool {
    if !is_dir.unwrap_or(false) {
        return true;
    }

    let dir_name = match path.file_name().and_then(|value| value.to_str()) {
        Some(value) => value,
        None => return true,
    };

    !SKIP_DIRECTORY_NAMES.contains(&dir_name)
}

#[derive(Debug, Serialize)]
pub struct ProjectInfo {
    pub project_dir: String,
    pub selected_files: Vec<String>,
    pub active_tab: Option<String>,
    pub active_operation_id: Option<String>,
    pub active_environment: Option<String>,
    pub needs_file_selection: bool,
}

#[tauri::command]
pub fn open_project(path: String, state: State<'_, AppState>, app_handle: AppHandle) -> Result<ProjectInfo, String> {
    let project_dir = PathBuf::from(&path);

    if !project_dir.exists() {
        return Err(format!("Project directory '{}' does not exist", project_dir.display()));
    }

    if !project_dir.is_dir() {
        return Err(format!("Path '{}' is not a directory", project_dir.display()));
    }

    state.set_project_dir(path.clone())?;
    state.watch_project_files(&app_handle, &path)?;

    let config = read_pup_config(&project_dir)?.unwrap_or_default();

    let project_info = ProjectInfo {
        project_dir: path,
        needs_file_selection: config.selected_files.is_empty(),
        selected_files: config.selected_files,
        active_tab: config.active_tab,
        active_operation_id: config.active_operation_id,
        active_environment: config.active_environment,
    };

    app_handle
        .emit("project-loaded", &project_info)
        .map_err(|e| format!("Failed to emit project-loaded event: {e}"))?;

    Ok(project_info)
}

#[tauri::command]
pub fn scan_openapi_files(dir: String) -> Result<Vec<OpenApiFile>, String> {
    let project_dir = PathBuf::from(&dir);

    if !project_dir.exists() {
        return Err(format!("Directory '{}' does not exist", project_dir.display()));
    }

    if !project_dir.is_dir() {
        return Err(format!("Path '{}' is not a directory", project_dir.display()));
    }

    let mut files: Vec<OpenApiFile> = Vec::new();

    let walker = WalkBuilder::new(&project_dir)
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .filter_entry(|entry| should_include_entry(entry.path(), entry.file_type().map(|value| value.is_dir())))
        .build();

    for entry in walker {
        let entry = match entry {
            Ok(value) => value,
            Err(_) => continue,
        };

        let path = entry.path();

        if !path.is_file() || !is_spec_candidate(path) {
            continue;
        }

        let relative_file = match path.strip_prefix(&project_dir).ok().and_then(|value| value.to_str()) {
            Some(value) => value.replace('\\', "/"),
            None => continue,
        };

        let (spec, _) = match read_openapi_value(path) {
            Ok(value) => value,
            Err(_) => continue,
        };

        let title = spec
            .get("info")
            .and_then(|value| value.get("title"))
            .and_then(|value| value.as_str())
            .map(str::to_string);

        let version = spec
            .get("info")
            .and_then(|value| value.get("version"))
            .and_then(|value| value.as_str())
            .map(str::to_string);

        files.push(OpenApiFile {
            file: relative_file,
            title,
            version,
        });
    }

    files.sort_by(|a, b| a.file.cmp(&b.file));
    Ok(files)
}

#[derive(Debug, Deserialize)]
pub struct SelectFilesRequest {
    pub dir: String,
    pub files: Vec<String>,
}

#[tauri::command]
pub fn select_files(request: SelectFilesRequest) -> Result<(), String> {
    let project_dir = PathBuf::from(&request.dir);

    for file in &request.files {
        let full_path = project_dir.join(file);
        if !full_path.exists() {
            return Err(format!("Selected file '{}' does not exist", full_path.display()));
        }

        read_openapi_value(&full_path).map_err(|e| {
            format!(
                "Selected file '{}' is not a valid OpenAPI 3.x spec: {e}",
                full_path.display()
            )
        })?;
    }

    let mut config = read_pup_config(&project_dir)?.unwrap_or_default();
    config.selected_files = request.files;

    if config.active_tab.is_none() {
        config.active_tab = config.selected_files.first().cloned();
    }

    write_pup_config(&project_dir, &config)
}

#[tauri::command]
pub fn save_pup_config(config: PupConfig, state: State<'_, AppState>) -> Result<(), String> {
    let project_dir = state
        .get_project_dir()?
        .ok_or_else(|| "No project is currently open".to_string())?;

    write_pup_config(Path::new(&project_dir), &config)
}

#[tauri::command]
pub fn get_cli_project_path() -> Option<String> {
    let candidate = env::args().nth(1)?;
    let path = PathBuf::from(candidate);

    if !path.is_dir() {
        return None;
    }

    std_fs::canonicalize(path)
        .ok()
        .and_then(|value| value.to_str().map(str::to_string))
}

fn is_spec_candidate(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase());

    matches!(ext.as_deref(), Some("json") | Some("yaml") | Some("yml"))
}

fn read_pup_config(project_dir: &Path) -> Result<Option<PupConfig>, String> {
    let config_path = pup_config_path(project_dir);

    if !config_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read '{}': {e}", config_path.display()))?;

    let config: PupConfig = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse '{}': {e}", config_path.display()))?;

    Ok(Some(config))
}

fn write_pup_config(project_dir: &Path, config: &PupConfig) -> Result<(), String> {
    let pup_dir = project_dir.join(PUP_DIR_NAME);
    fs::create_dir_all(&pup_dir)
        .map_err(|e| format!("Failed to create '{}': {e}", pup_dir.display()))?;

    let config_path = pup_dir.join(PUP_CONFIG_NAME);
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config for '{}': {e}", config_path.display()))?;

    fs::write(&config_path, format!("{content}\n"))
        .map_err(|e| format!("Failed to write '{}': {e}", config_path.display()))
}

fn pup_config_path(project_dir: &Path) -> PathBuf {
    project_dir.join(PUP_DIR_NAME).join(PUP_CONFIG_NAME)
}
