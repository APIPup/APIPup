# Data Storage Design

## Overview

APIPup stores all data within the user's project directory, maintained by git.
API specifications use the standard OpenAPI 3.x format. Runtime client data
(environments, history, UI state) lives in a `.pup/` subdirectory.

## Project Opening

Two entry points, both supported:

- **Menu**: Tauri file dialog to select a directory
- **CLI**: directory path passed as a command-line argument

Both paths trigger the same project loading sequence.

## File Structure

```
my-api-project/
├── openapi.json          # OpenAPI 3.x spec (git-tracked)
├── pets-api.yaml         # Another spec (git-tracked)
└── .pup/                 # Client project data (user decides whether to git-track)
    ├── config.json       # Selected files, active tab, active operation, active env
    ├── environments.json # Named environment variable sets (base_url, token, etc.)
    └── history.json      # Request execution history (recommended: .gitignore)
```

### openapi.json / openapi.yaml

Strict OpenAPI 3.x format. No `x-` extension fields for APIPup-specific data.
APIPup request fields map to standard OpenAPI fields:

| APIPup concept | OpenAPI field |
|---|---|
| HTTP method | `paths[path][method]` key |
| Path | `paths` key |
| Query/path params | `operation.parameters` |
| Request headers | `operation.parameters` (header type) |
| Request body | `operation.requestBody` |
| Description | `operation.summary` |
| Example request | `operation.requestBody.content[*].examples` |

### .pup/config.json

```json
{
  "selected_files": ["openapi.json", "pets-api.yaml"],
  "active_tab": "openapi.json",
  "active_operation_id": "createPet",
  "active_environment": "dev"
}
```

### .pup/environments.json

```json
{
  "dev": { "base_url": "http://localhost:3000", "token": "..." },
  "prod": { "base_url": "https://api.example.com", "token": "..." }
}
```

## Startup Sequence

1. Check for `.pup/config.json`
   - **Found**: load `selected_files` directly
   - **Not found**: scan directory for all `.json` / `.yaml` / `.yml` files,
     attempt to parse each as OpenAPI 3.x, present multi-select dialog to user,
     write selection to `.pup/config.json`
2. Parse each selected file with `openapiv3` crate
3. Build one Tab per file, populate sidebar with endpoints
4. Start `notify` watcher on the project directory

## Bidirectional Sync

### File → UI (external edits)

```
External editor saves openapi.json
  └── notify fires event
      └── Rust re-parses the file with openapiv3
          └── tauri emit("spec-changed", { file, spec })
              └── Frontend store updates reactively
```

### UI → File (auto-save)

```
User edits request (method / url / body / ...)
  └── Frontend debounce 300ms
      └── invoke("save_operation", { file, path, method, operation })
          └── Rust mutates spec object in memory
              └── Write back to disk (preserving original format: JSON or YAML)
```

When both change simultaneously (rare), the last write wins. No merge conflict
resolution is needed at this stage; git handles that at the version-control level.

## Architecture: Rust Layer

```
tauri/src/
├── commands/
│   ├── http.rs         # existing: send HTTP requests
│   ├── project.rs      # new: open/scan project directory
│   └── spec.rs         # new: read/write OpenAPI spec operations
├── services/
│   ├── watcher.rs      # new: file watching (notify crate)
│   └── project.rs      # new: in-memory project state
└── lib.rs
```

### Tauri Commands

| Command | Parameters | Returns |
|---|---|---|
| `open_project` | `path: String` | `ProjectInfo` |
| `scan_openapi_files` | `dir: String` | `Vec<OpenApiFile>` |
| `select_files` | `dir: String, files: Vec<String>` | `()` |
| `load_spec` | `file: String` | `OpenApiSpec` |
| `save_operation` | `file, path, method, operation` | `()` |
| `save_pup_config` | `config: PupConfig` | `()` |

### Tauri Events (backend → frontend)

| Event | Payload | When |
|---|---|---|
| `spec-changed` | `{ file: String, spec: OpenApiSpec }` | File modified externally |
| `project-loaded` | `ProjectInfo` | Project successfully opened |

## Key Dependencies

| Crate | Purpose |
|---|---|
| `openapiv3` | OpenAPI 3.x parsing and in-memory manipulation |
| `notify` | Cross-platform filesystem event watching |
| `serde_yaml` | YAML serialization/deserialization |
| `tauri-plugin-dialog` | Native folder picker dialog |

## Format Handling

- Each file retains its original format on write-back (JSON files stay JSON, YAML files stay YAML)
- When multiple OpenAPI files exist in the same directory, APIPup scans all of them and asks the user which to load; the choice is persisted in `.pup/config.json` and auto-applied on next open
- Both JSON and YAML are parsed through `openapiv3` (via `serde_json` / `serde_yaml`)

## Out of Scope

- OpenAPI 2.x (Swagger) support
- Merging concurrent edits from multiple sources
- Real-time collaboration

## Implementation Checklist

### Backend (Rust / Tauri)

- [x] ~~Add Rust dependencies: `openapiv3`, `serde_yaml`~~
- [x] ~~Add backend command modules: `commands/project.rs`, `commands/spec.rs`~~
- [x] ~~Add backend service modules: `services/project.rs`, `services/spec_io.rs`~~
- [x] ~~Register new Tauri commands in `tauri/src/lib.rs`~~
- [x] ~~Implement `open_project` (load `.pup/config.json` if present)~~
- [x] ~~Implement `scan_openapi_files` (`.json/.yaml/.yml` + OpenAPI 3.x validation)~~
- [x] ~~Implement `select_files` and persist selection to `.pup/config.json`~~
- [x] ~~Implement `save_pup_config`~~
- [x] ~~Implement `load_spec` (JSON/YAML parse + OpenAPI 3.x validation)~~
- [x] ~~Implement `save_operation` (`paths[path][method]` write-back)~~
- [x] ~~Preserve original spec format on write-back (JSON remains JSON, YAML remains YAML)~~
- [x] ~~Run backend compile validation (`cargo check`)~~

### Frontend (Svelte)

- [x] ~~Add project/spec API wrappers in `ui/src/lib/api` (`openProject`, `scanOpenApiFiles`, `selectFiles`, `loadSpec`, `saveOperation`, `savePupConfig`)~~
- [x] ~~Add frontend domain interfaces for `ProjectInfo`, `OpenApiFile`, `PupConfig`, and operation save payloads~~
- [x] ~~Add project-level store for selected files, active tab, active operation, active environment~~
- [x] ~~Implement project open flow from both entry points (menu + CLI path)~~
- [x] ~~Implement first-open file selection UI (multi-select + confirm write to `.pup/config.json`)~~
- [x] ~~Build tab model from loaded OpenAPI files and render endpoint sidebar per tab~~
- [x] ~~Implement UI auto-save flow for request edits (`debounce 300ms` -> `save_operation`)~~
- [x] ~~Add explicit loading/success/error handling for project loading and spec save flows~~

### Sync & Events

- [x] ~~Implement file watcher service (`notify`) for project directory~~
- [x] ~~Emit `spec-changed` event on external spec file modifications~~
- [x] ~~Emit `project-loaded` event after successful project open~~
- [x] ~~Wire frontend event listeners for `spec-changed` and apply reactive store updates~~
- [x] ~~Wire frontend event listeners for `project-loaded` and initialize project state~~

### Validation

- [x] ~~Add backend unit tests for OpenAPI format detection and 3.x validation~~
- [x] ~~Add backend unit tests for `save_operation` path/method write logic~~
- [x] ~~Add frontend integration checks for startup flow (with/without `.pup/config.json`)~~
