use std::path::{Path, PathBuf};
use std::sync::Mutex;

use notify::{recommended_watcher, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PupConfig {
    pub selected_files: Vec<String>,
    pub active_tab: Option<String>,
    pub active_operation_id: Option<String>,
    pub active_environment: Option<String>,
}

#[derive(Debug, Default)]
pub struct ProjectState {
    pub current_project_dir: Option<String>,
}

#[derive(Debug, Default)]
pub struct AppState {
    pub project_state: Mutex<ProjectState>,
    pub project_watcher: Mutex<Option<RecommendedWatcher>>,
}

impl AppState {
    pub fn set_project_dir(&self, project_dir: String) -> Result<(), String> {
        let mut state = self
            .project_state
            .lock()
            .map_err(|_| "Failed to acquire project state lock".to_string())?;

        state.current_project_dir = Some(project_dir);
        Ok(())
    }

    pub fn get_project_dir(&self) -> Result<Option<String>, String> {
        let state = self
            .project_state
            .lock()
            .map_err(|_| "Failed to acquire project state lock".to_string())?;

        Ok(state.current_project_dir.clone())
    }

    pub fn watch_project_files(&self, app_handle: &AppHandle, project_dir: &str) -> Result<(), String> {
        let project_root = PathBuf::from(project_dir);
        let watch_root = project_root.clone();
        let event_app_handle = app_handle.clone();

        let mut watcher = recommended_watcher(move |result: notify::Result<Event>| {
            let event = match result {
                Ok(value) => value,
                Err(_) => return,
            };

            if !is_spec_related_event_kind(&event.kind) {
                return;
            }

            for path in event.paths {
                if !is_spec_candidate(&path) {
                    continue;
                }

                let relative_file = match path.strip_prefix(&watch_root).ok().and_then(|value| value.to_str()) {
                    Some(value) => value.replace('\\', "/"),
                    None => continue,
                };

                let _ = event_app_handle.emit(
                    "spec-changed",
                    SpecChangedEvent {
                        file: relative_file,
                    },
                );
            }
        })
        .map_err(|e| format!("Failed to create project watcher: {e}"))?;

        watcher
            .watch(&project_root, RecursiveMode::Recursive)
            .map_err(|e| format!("Failed to watch project directory '{}': {e}", project_root.display()))?;

        let mut watcher_state = self
            .project_watcher
            .lock()
            .map_err(|_| "Failed to acquire project watcher lock".to_string())?;

        watcher_state.replace(watcher);
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SpecChangedEvent {
    pub file: String,
}

fn is_spec_related_event_kind(kind: &EventKind) -> bool {
    matches!(
        kind,
        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) | EventKind::Any
    )
}

fn is_spec_candidate(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase());

    matches!(ext.as_deref(), Some("json") | Some("yaml") | Some("yml"))
}
