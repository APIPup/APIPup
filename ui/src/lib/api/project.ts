import { invoke } from '@tauri-apps/api/core';

import type { OpenApiFile, ProjectInfo, PupConfig } from '$lib/types/project';

export async function openProject(path: string): Promise<ProjectInfo> {
  return invoke<ProjectInfo>('open_project', { path });
}

export async function getCliProjectPath(): Promise<string | null> {
  return invoke<string | null>('get_cli_project_path');
}

export async function scanOpenApiFiles(dir: string): Promise<OpenApiFile[]> {
  return invoke<OpenApiFile[]>('scan_openapi_files', { dir });
}

export async function selectFiles(dir: string, files: string[]): Promise<void> {
  return invoke<void>('select_files', {
    request: {
      dir,
      files
    }
  });
}

export async function savePupConfig(config: PupConfig): Promise<void> {
  return invoke<void>('save_pup_config', { config });
}
