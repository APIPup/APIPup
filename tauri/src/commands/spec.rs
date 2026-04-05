use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_json::{Map, Value};
use tauri::State;

use crate::services::project::AppState;
use crate::services::spec_io::{read_openapi_value, write_openapi_value};

#[tauri::command]
pub fn load_spec(file: String, state: State<'_, AppState>) -> Result<Value, String> {
    let file_path = resolve_spec_path(&file, &state)?;
    let (spec, _) = read_openapi_value(&file_path)?;
    Ok(spec)
}

#[derive(Debug, Deserialize)]
pub struct SaveOperationRequest {
    pub file: String,
    pub path: String,
    pub method: String,
    pub operation: Value,
}

#[tauri::command]
pub fn save_operation(request: SaveOperationRequest, state: State<'_, AppState>) -> Result<(), String> {
    if request.path.trim().is_empty() {
        return Err("Path must not be empty".to_string());
    }

    if request.method.trim().is_empty() {
        return Err("Method must not be empty".to_string());
    }

    let file_path = resolve_spec_path(&request.file, &state)?;
    let (mut spec, format) = read_openapi_value(&file_path)?;

    let root = spec
        .as_object_mut()
        .ok_or_else(|| "Spec root must be a JSON object".to_string())?;

    apply_operation_to_paths(root, request.path, request.method, request.operation)?;

    write_openapi_value(&file_path, &spec, format)
}

fn apply_operation_to_paths(
    root: &mut Map<String, Value>,
    path: String,
    method: String,
    operation: Value,
) -> Result<(), String> {
    let paths = ensure_object(root, "paths")?;
    let path_item = paths.entry(path).or_insert_with(|| Value::Object(Map::new()));

    let path_item_obj = path_item
        .as_object_mut()
        .ok_or_else(|| "Spec path item must be a JSON object".to_string())?;

    path_item_obj.insert(method.to_ascii_lowercase(), operation);
    Ok(())
}

fn ensure_object<'a>(obj: &'a mut Map<String, Value>, key: &str) -> Result<&'a mut Map<String, Value>, String> {
    let value = obj
        .entry(key.to_string())
        .or_insert_with(|| Value::Object(Map::new()));

    value
        .as_object_mut()
        .ok_or_else(|| format!("Spec field '{key}' must be an object"))
}

fn resolve_spec_path(file: &str, state: &State<'_, AppState>) -> Result<PathBuf, String> {
    let requested_path = PathBuf::from(file);
    if requested_path.is_absolute() {
        return Ok(requested_path);
    }

    let project_dir = state
        .get_project_dir()?
        .ok_or_else(|| "No project is currently open".to_string())?;

    Ok(Path::new(&project_dir).join(requested_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_operation_to_paths_writes_method_under_path() {
        let mut root = Map::new();
        let operation = serde_json::json!({
            "summary": "List pets",
            "responses": { "200": { "description": "ok" } }
        });

        let result = apply_operation_to_paths(&mut root, "/pets".to_string(), "GET".to_string(), operation);
        assert!(result.is_ok());

        let paths = root
            .get("paths")
            .and_then(Value::as_object)
            .expect("paths should exist and be object");

        let pets = paths
            .get("/pets")
            .and_then(Value::as_object)
            .expect("/pets path item should exist and be object");

        assert!(pets.get("get").is_some());
    }

    #[test]
    fn apply_operation_to_paths_updates_existing_method_value() {
        let mut root = serde_json::json!({
            "paths": {
                "/pets": {
                    "get": {
                        "summary": "Old summary",
                        "responses": { "200": { "description": "ok" } }
                    }
                }
            }
        })
        .as_object()
        .cloned()
        .expect("root json object expected");

        let new_operation = serde_json::json!({
            "summary": "New summary",
            "responses": { "200": { "description": "ok" } }
        });

        let result = apply_operation_to_paths(&mut root, "/pets".to_string(), "GET".to_string(), new_operation);
        assert!(result.is_ok());

        let summary = root
            .get("paths")
            .and_then(Value::as_object)
            .and_then(|paths| paths.get("/pets"))
            .and_then(Value::as_object)
            .and_then(|item| item.get("get"))
            .and_then(Value::as_object)
            .and_then(|operation| operation.get("summary"))
            .and_then(Value::as_str)
            .expect("updated operation summary should exist");

        assert_eq!(summary, "New summary");
    }
}
