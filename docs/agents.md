# Agents Code Standards

## Language Requirements

All code MUST be written in English, except:
- Language files (i18n translations)
- User-facing strings for non-English users

## Required English Content
- Variable/function/class names
- File/directory names
- Comments and documentation
- Error messages and logs
- Configuration files

## TypeScript (Svelte) Standards

- Use explicit type annotations for public APIs, props, store values, and function return types when not trivially inferred.
- Prefer `interface` for structured domain models (e.g., request/response payloads) and `type` for unions or utility aliases.
- Naming:
  - `camelCase` for variables and functions.
  - `PascalCase` for components and interfaces.
  - `UPPER_SNAKE_CASE` for constants maps and immutable lookup tables.
- Keep business logic in small pure functions where possible; event handlers should stay focused and readable.
- Use early returns to reduce nesting in handlers and async flows.
- For async calls, always handle loading, success, and error states explicitly.
- Use `Record<string, string>` for header-like key-value maps and typed arrays for editable list items.
- Prefer immutable updates (`map`, `filter`, spread syntax) for store/state updates.
- Keep imports grouped by source (`$lib` aliases together, then local files), and avoid unused imports.
- Limit comments to clarifying intent for non-obvious logic; do not add redundant comments.

## Rust (Tauri) Standards

- Use `snake_case` for modules, functions, fields, and local variables; use `PascalCase` for structs and enums.
- Keep command contracts explicit with dedicated `serde` structs for request and response payloads.
- Derive traits minimally and intentionally (commonly `Debug`, `Serialize`, `Deserialize`).
- Prefer `Result<T, String>` in Tauri commands when error messages are surfaced directly to the frontend.
- Use `map_err` with clear, actionable error text at each fallible boundary.
- Favor early returns via `?` and avoid deep nesting.
- Keep functions single-purpose: parse/validate input, build request, execute, transform response.
- Use standard library types (`HashMap`, `Option`) directly unless a custom type improves clarity.
- Avoid `unwrap`; use safe fallbacks (`unwrap_or`) or proper error propagation.
- Preserve formatting and style compatible with `rustfmt` defaults.

## Cross-Layer Contract Rules

- Keep TS and Rust payload fields aligned by name and semantics (e.g., `status_text`, `elapsed_ms`).
- When changing command payloads, update both frontend types and backend structs in the same change.
- Ensure error messages crossing the Tauri boundary remain concise and user-actionable.