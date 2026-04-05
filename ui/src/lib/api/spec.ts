import { invoke } from '@tauri-apps/api/core';

import type { OpenApiSpec, SaveOperationPayload } from '$lib/types/project';

export async function loadSpec(file: string): Promise<OpenApiSpec> {
  return invoke<OpenApiSpec>('load_spec', { file });
}

export async function saveOperation(payload: SaveOperationPayload): Promise<void> {
  return invoke<void>('save_operation', { request: payload });
}
