export interface OpenApiFile {
  file: string;
  title: string | null;
  version: string | null;
}

export interface ProjectInfo {
  project_dir: string;
  selected_files: string[];
  active_tab: string | null;
  active_operation_id: string | null;
  active_environment: string | null;
  needs_file_selection: boolean;
}

export interface PupConfig {
  selected_files: string[];
  active_tab: string | null;
  active_operation_id: string | null;
  active_environment: string | null;
}

export type OpenApiSpec = Record<string, unknown>;

export interface SaveOperationPayload {
  file: string;
  path: string;
  method: string;
  operation: Record<string, unknown>;
}

export interface SpecChangedEvent {
  file: string;
}
