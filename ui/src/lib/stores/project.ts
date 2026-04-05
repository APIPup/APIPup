import { derived, get, writable } from 'svelte/store';

import {
  getCliProjectPath,
  openProject,
  savePupConfig,
  scanOpenApiFiles,
  selectFiles
} from '$lib/api/project';
import { loadSpec } from '$lib/api/spec';
import type { OpenApiFile, OpenApiSpec, ProjectInfo, PupConfig } from '$lib/types/project';

export const projectInfo = writable<ProjectInfo | null>(null);
export const projectFiles = writable<OpenApiFile[]>([]);
export const specByFile = writable<Record<string, OpenApiSpec>>({});

export const selectedFiles = writable<string[]>([]);
export const activeTab = writable<string | null>(null);
export const activeOperationId = writable<string | null>(null);
export const activeEnvironment = writable<string | null>(null);

export const projectLoading = writable<boolean>(false);
export const projectError = writable<string | null>(null);
export const pendingSelectionFiles = writable<string[]>([]);

export const hasOpenProject = derived(projectInfo, ($projectInfo) => $projectInfo !== null);
export const needsFileSelection = derived(
  [projectInfo, selectedFiles],
  ([$projectInfo, $selectedFiles]) => {
    if (!$projectInfo) {
      return false;
    }
    return $projectInfo.needs_file_selection || $selectedFiles.length === 0;
  }
);

export function setActiveSpecTab(file: string | null): void {
  activeTab.set(file);
}

export function applyProjectInfo(info: ProjectInfo): void {
  projectInfo.set(info);
  selectedFiles.set(info.selected_files);
  pendingSelectionFiles.set(info.selected_files);
  activeTab.set(info.active_tab ?? info.selected_files[0] ?? null);
  activeOperationId.set(info.active_operation_id);
  activeEnvironment.set(info.active_environment);
}

export async function initializeProjectFromCli(): Promise<void> {
  const cliPath = await getCliProjectPath();
  if (!cliPath) {
    return;
  }

  await openProjectByPath(cliPath);
}

export async function openProjectByPath(path: string): Promise<ProjectInfo> {
  projectLoading.set(true);
  projectError.set(null);

  try {
    const info = await openProject(path);
    applyProjectInfo(info);

    await refreshOpenApiFiles();

    if (!info.needs_file_selection && info.selected_files.length > 0) {
      await loadSpecs(info.selected_files);
    }

    return info;
  } catch (err) {
    const message = String(err);
    projectError.set(message);
    throw new Error(message);
  } finally {
    projectLoading.set(false);
  }
}

export async function refreshOpenApiFiles(): Promise<OpenApiFile[]> {
  const info = get(projectInfo);
  if (!info) {
    projectFiles.set([]);
    return [];
  }

  const files = await scanOpenApiFiles(info.project_dir);
  projectFiles.set(files);

  if (get(pendingSelectionFiles).length === 0) {
    pendingSelectionFiles.set(files.map((file) => file.file));
  }

  return files;
}

export async function selectProjectFiles(files: string[]): Promise<void> {
  const info = get(projectInfo);
  if (!info) {
    throw new Error('No project is currently open');
  }

  await selectFiles(info.project_dir, files);

  selectedFiles.set(files);
  pendingSelectionFiles.set(files);

  activeTab.set(files[0] ?? null);

  await loadSpecs(files);

  projectInfo.update((current) => {
    if (!current) {
      return current;
    }

    return {
      ...current,
      selected_files: files,
      needs_file_selection: false
    };
  });
}

export function togglePendingFile(file: string, checked: boolean): void {
  pendingSelectionFiles.update((files) => {
    if (checked) {
      if (files.includes(file)) {
        return files;
      }

      return [...files, file];
    }

    return files.filter((value) => value !== file);
  });
}

export async function loadSpecs(files: string[]): Promise<void> {
  const info = get(projectInfo);
  if (!info) {
    throw new Error('No project is currently open');
  }

  const loadedSpecs: Record<string, OpenApiSpec> = {};

  for (const file of files) {
    loadedSpecs[file] = await loadSpec(file);
  }

  specByFile.set(loadedSpecs);
}

export async function reloadSpecFile(file: string): Promise<void> {
  const selected = get(selectedFiles);
  if (!selected.includes(file)) {
    return;
  }

  const nextSpec = await loadSpec(file);
  specByFile.update((map) => ({
    ...map,
    [file]: nextSpec
  }));
}

export function updateSpecEndpoint(
  file: string,
  oldPath: string,
  oldMethod: string,
  newPath: string,
  newMethod: string,
  operation: Record<string, unknown>
): void {
  specByFile.update((map) => {
    const currentSpec = map[file];
    if (!currentSpec) {
      return map;
    }

    const nextSpec: Record<string, unknown> = { ...currentSpec };
    const pathsValue = nextSpec.paths;
    const paths = (pathsValue && typeof pathsValue === 'object'
      ? { ...(pathsValue as Record<string, unknown>) }
      : {}) as Record<string, unknown>;

    const oldPathItemValue = paths[oldPath];
    const oldPathItem = (oldPathItemValue && typeof oldPathItemValue === 'object'
      ? { ...(oldPathItemValue as Record<string, unknown>) }
      : {}) as Record<string, unknown>;
    delete oldPathItem[oldMethod.toLowerCase()];

    if (Object.keys(oldPathItem).length === 0) {
      delete paths[oldPath];
    } else {
      paths[oldPath] = oldPathItem;
    }

    const targetPathItemValue = paths[newPath];
    const targetPathItem = (targetPathItemValue && typeof targetPathItemValue === 'object'
      ? { ...(targetPathItemValue as Record<string, unknown>) }
      : {}) as Record<string, unknown>;

    targetPathItem[newMethod.toLowerCase()] = operation;
    paths[newPath] = targetPathItem;
    nextSpec.paths = paths;

    return {
      ...map,
      [file]: nextSpec
    };
  });
}

export async function persistPupConfig(): Promise<void> {
  const info = get(projectInfo);
  if (!info) {
    throw new Error('No project is currently open');
  }

  const config: PupConfig = {
    selected_files: get(selectedFiles),
    active_tab: get(activeTab),
    active_operation_id: get(activeOperationId),
    active_environment: get(activeEnvironment)
  };

  await savePupConfig(config);
}

export function clearProjectState(): void {
  projectInfo.set(null);
  projectFiles.set([]);
  specByFile.set({});
  selectedFiles.set([]);
  activeTab.set(null);
  activeOperationId.set(null);
  activeEnvironment.set(null);
  pendingSelectionFiles.set([]);
  projectError.set(null);
  projectLoading.set(false);
}
