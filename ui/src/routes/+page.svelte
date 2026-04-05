<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { open } from '@tauri-apps/plugin-dialog';

  import Sidebar from '$lib/components/Sidebar.svelte';
  import RequestPanel from '$lib/components/RequestPanel.svelte';
  import ResponsePanel from '$lib/components/ResponsePanel.svelte';
  import FileSelectionModal from '$lib/components/FileSelectionModal.svelte';
  import {
    applyProjectInfo,
    hasOpenProject,
    initializeProjectFromCli,
    loadSpecs,
    needsFileSelection,
    pendingSelectionFiles,
    projectError,
    projectFiles,
    projectInfo,
    projectLoading,
    refreshOpenApiFiles,
    reloadSpecFile,
    selectProjectFiles,
    togglePendingFile,
    openProjectByPath
  } from '$lib/stores/project';
  import type { ProjectInfo, SpecChangedEvent } from '$lib/types/project';

  let projectPath = $state('');
  let selectionSaving = $state(false);
  let showSelectionModal = $state(false);

  onMount(() => {
    let unlistenProjectLoaded: (() => void) | null = null;
    let unlistenSpecChanged: (() => void) | null = null;

    const setup = async () => {
      unlistenProjectLoaded = await listen<ProjectInfo>('project-loaded', async (event) => {
        const info = event.payload;
        applyProjectInfo(info);
        await refreshOpenApiFiles();

        if (!info.needs_file_selection && info.selected_files.length > 0) {
          await loadSpecs(info.selected_files);
        }

        showSelectionModal = info.needs_file_selection;
      });

      unlistenSpecChanged = await listen<SpecChangedEvent>('spec-changed', async (event) => {
        await reloadSpecFile(event.payload.file);
      });

      await initializeProjectFromCli();
      showSelectionModal = $needsFileSelection;
    };

    void setup();

    return () => {
      unlistenProjectLoaded?.();
      unlistenSpecChanged?.();
    };
  });

  async function handleOpenProject() {
    const trimmedPath = projectPath.trim();
    if (!trimmedPath) {
      return;
    }

    await openProjectByPath(trimmedPath);
    showSelectionModal = $needsFileSelection;
  }

  async function handleBrowseProject() {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Project Directory'
    });

    if (typeof selected !== 'string' || !selected) {
      return;
    }

    projectPath = selected;
    await openProjectByPath(selected);
    showSelectionModal = $needsFileSelection;
  }

  async function handleApplySelection() {
    selectionSaving = true;

    try {
      await selectProjectFiles($pendingSelectionFiles);
      showSelectionModal = false;
    } finally {
      selectionSaving = false;
    }
  }

  function handleCloseSelectionModal() {
    if (selectionSaving) {
      return;
    }

    showSelectionModal = false;
  }
</script>

<div class="h-screen overflow-hidden flex flex-col">
  <div class="px-4 py-3 border-b border-border bg-surface-dark flex items-center gap-3">
    <input
      type="text"
      value={projectPath}
      oninput={(e) => (projectPath = e.currentTarget.value)}
      placeholder="Open project path..."
      class="flex-1 px-3 py-1.5 border border-border rounded bg-white text-sm outline-none focus:border-primary"
    />
    <button
      onclick={handleBrowseProject}
      disabled={$projectLoading}
      class="px-3 py-1.5 border border-border rounded bg-white text-sm hover:bg-gray-50 transition-colors disabled:opacity-50"
    >
      Browse
    </button>
    <button
      onclick={handleOpenProject}
      disabled={$projectLoading || !projectPath.trim()}
      class="px-4 py-1.5 bg-primary text-white rounded text-sm font-medium hover:bg-primary-light transition-colors disabled:opacity-50"
    >
      {$projectLoading ? 'Opening...' : 'Open Project'}
    </button>
    {#if $projectInfo}
      <span class="text-xs text-gray-500 truncate max-w-80">{$projectInfo.project_dir}</span>
    {/if}
  </div>

  {#if $projectError}
    <div class="px-4 py-2 text-sm text-red-600 border-b border-border">{$projectError}</div>
  {/if}

  <div class="flex flex-1 overflow-hidden">
    <!-- Left: Sidebar -->
    <Sidebar />

    <!-- Right: Request + Response -->
    <main class="flex-1 flex flex-col overflow-hidden">
      <!-- Top: Request panel -->
      <div class="flex flex-col flex-1 min-h-0">
        <RequestPanel />
      </div>

      <!-- Bottom: Response panel -->
      <div class="flex flex-col flex-1 min-h-0">
        <ResponsePanel />
      </div>
    </main>
  </div>

  <FileSelectionModal
    open={$hasOpenProject && $needsFileSelection && showSelectionModal}
    files={$projectFiles}
    selectedFiles={$pendingSelectionFiles}
    saving={selectionSaving}
    onToggle={togglePendingFile}
    onConfirm={handleApplySelection}
    onClose={handleCloseSelectionModal}
  />
</div>
