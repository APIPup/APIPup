use std::fs;
use std::path::Path;

use serde_json::Value;

#[derive(Debug, Clone, Copy)]
pub enum SpecFormat {
    Json,
    Yaml,
}

pub fn read_openapi_value(file_path: &Path) -> Result<(Value, SpecFormat), String> {
    let format = detect_spec_format(file_path)?;
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read spec file '{}': {e}", file_path.display()))?;

    let value: Value = match format {
        SpecFormat::Json => serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON spec '{}': {e}", file_path.display()))?,
        SpecFormat::Yaml => {
            let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)
                .map_err(|e| format!("Failed to parse YAML spec '{}': {e}", file_path.display()))?;
            serde_json::to_value(yaml_value)
                .map_err(|e| format!("Failed to convert YAML spec '{}': {e}", file_path.display()))?
        }
    };

    validate_openapi_3(&value)?;

    Ok((value, format))
}

pub fn write_openapi_value(file_path: &Path, spec: &Value, format: SpecFormat) -> Result<(), String> {
    validate_openapi_3(spec)?;

    let output = match format {
        SpecFormat::Json => {
            let mut data = serde_json::to_string_pretty(spec)
                .map_err(|e| format!("Failed to serialize JSON spec '{}': {e}", file_path.display()))?;
            data.push('\n');
            data
        }
        SpecFormat::Yaml => serde_yaml::to_string(spec)
            .map_err(|e| format!("Failed to serialize YAML spec '{}': {e}", file_path.display()))?,
    };

    fs::write(file_path, output)
        .map_err(|e| format!("Failed to write spec file '{}': {e}", file_path.display()))
}

pub fn validate_openapi_3(spec: &Value) -> Result<(), String> {
    let parsed: openapiv3::OpenAPI = serde_json::from_value(spec.clone())
        .map_err(|e| format!("Invalid OpenAPI structure: {e}"))?;

    if !parsed.openapi.starts_with("3.") {
        return Err(format!(
            "Unsupported OpenAPI version '{}'. Only 3.x is supported",
            parsed.openapi
        ));
    }

    Ok(())
}

fn detect_spec_format(file_path: &Path) -> Result<SpecFormat, String> {
    let ext = file_path
        .extension()
        .and_then(|value| value.to_str())
        .ok_or_else(|| format!("Spec file '{}' has no extension", file_path.display()))?
        .to_ascii_lowercase();

    match ext.as_str() {
        "json" => Ok(SpecFormat::Json),
        "yaml" | "yml" => Ok(SpecFormat::Yaml),
        _ => Err(format!(
            "Unsupported spec extension '.{}'. Expected .json, .yaml, or .yml",
            ext
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_file_path(extension: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("apipup-spec-io-{nanos}.{extension}"))
    }

    fn write_temp_spec(extension: &str, content: &str) -> std::path::PathBuf {
        let path = unique_temp_file_path(extension);
        fs::write(&path, content).expect("failed to write temporary spec file");
        path
    }

    #[test]
    fn read_openapi_value_detects_json_format() {
        let file_path = write_temp_spec(
            "json",
            r#"{
  "openapi": "3.0.3",
  "info": {"title": "T", "version": "1.0.0"},
  "paths": {}
}"#,
        );

        let result = read_openapi_value(&file_path);
        let _ = fs::remove_file(&file_path);

        let (_, format) = result.expect("json spec should be read successfully");
        assert!(matches!(format, SpecFormat::Json));
    }

    #[test]
    fn read_openapi_value_detects_yaml_format() {
        let file_path = write_temp_spec(
            "yaml",
            r#"openapi: 3.0.3
info:
  title: T
  version: 1.0.0
paths: {}
"#,
        );

        let result = read_openapi_value(&file_path);
        let _ = fs::remove_file(&file_path);

        let (_, format) = result.expect("yaml spec should be read successfully");
        assert!(matches!(format, SpecFormat::Yaml));
    }

    #[test]
    fn validate_openapi_3_rejects_non_3x_version() {
        let value = serde_json::json!({
            "openapi": "2.0",
            "info": {"title": "Legacy", "version": "1.0.0"},
            "paths": {}
        });

        let result = validate_openapi_3(&value);
        assert!(result.is_err());
        let error = result.err().expect("error must exist for non-3x spec");
        assert!(error.contains("Only 3.x is supported"));
    }
}
