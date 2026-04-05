import { get } from 'svelte/store';
import { beforeEach, describe, expect, it, vi } from 'vitest';

const projectApiMock = vi.hoisted(() => ({
  getCliProjectPath: vi.fn<() => Promise<string | null>>(),
  openProject: vi.fn(),
  savePupConfig: vi.fn(),
  scanOpenApiFiles: vi.fn(),
  selectFiles: vi.fn()
}));

const specApiMock = vi.hoisted(() => ({
  loadSpec: vi.fn()
}));

vi.mock('$lib/api/project', () => ({
  getCliProjectPath: projectApiMock.getCliProjectPath,
  openProject: projectApiMock.openProject,
  savePupConfig: projectApiMock.savePupConfig,
  scanOpenApiFiles: projectApiMock.scanOpenApiFiles,
  selectFiles: projectApiMock.selectFiles
}));

vi.mock('$lib/api/spec', () => ({
  loadSpec: specApiMock.loadSpec
}));

import {
  activeTab,
  clearProjectState,
  initializeProjectFromCli,
  needsFileSelection,
  projectInfo,
  selectedFiles,
  specByFile
} from '$lib/stores/project';
import type { ProjectInfo } from '$lib/types/project';

function buildProjectInfo(overrides: Partial<ProjectInfo>): ProjectInfo {
  return {
    project_dir: '/tmp/demo',
    selected_files: [],
    active_tab: null,
    active_operation_id: null,
    active_environment: null,
    needs_file_selection: true,
    ...overrides
  };
}

describe('project startup flow', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    clearProjectState();
  });

  it('does nothing when CLI project path is missing', async () => {
    projectApiMock.getCliProjectPath.mockResolvedValueOnce(null);

    await initializeProjectFromCli();

    expect(projectApiMock.openProject).not.toHaveBeenCalled();
    expect(get(projectInfo)).toBeNull();
  });

  it('loads startup state and keeps file selection modal required when config has no files', async () => {
    projectApiMock.getCliProjectPath.mockResolvedValueOnce('/tmp/demo');
    projectApiMock.openProject.mockResolvedValueOnce(
      buildProjectInfo({
        selected_files: [],
        needs_file_selection: true
      })
    );
    projectApiMock.scanOpenApiFiles.mockResolvedValueOnce([
      { file: 'openapi.yaml', title: null, version: null }
    ]);

    await initializeProjectFromCli();

    expect(projectApiMock.openProject).toHaveBeenCalledWith('/tmp/demo');
    expect(specApiMock.loadSpec).not.toHaveBeenCalled();
    expect(get(selectedFiles)).toEqual([]);
    expect(get(needsFileSelection)).toBe(true);
  });

  it('loads selected specs when existing config contains selected files', async () => {
    projectApiMock.getCliProjectPath.mockResolvedValueOnce('/tmp/demo');
    projectApiMock.openProject.mockResolvedValueOnce(
      buildProjectInfo({
        selected_files: ['petstore.yaml'],
        active_tab: 'petstore.yaml',
        needs_file_selection: false
      })
    );
    projectApiMock.scanOpenApiFiles.mockResolvedValueOnce([
      { file: 'petstore.yaml', title: 'Petstore', version: '1.0.0' }
    ]);

    const mockSpec = {
      openapi: '3.0.3',
      info: { title: 'Petstore', version: '1.0.0' },
      paths: {}
    };
    specApiMock.loadSpec.mockResolvedValueOnce(mockSpec);

    await initializeProjectFromCli();

    expect(specApiMock.loadSpec).toHaveBeenCalledWith('petstore.yaml');
    expect(get(activeTab)).toBe('petstore.yaml');
    expect(get(specByFile)).toEqual({
      'petstore.yaml': mockSpec
    });
  });
});
