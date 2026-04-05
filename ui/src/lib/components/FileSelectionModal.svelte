<script lang="ts">
  import type { OpenApiFile } from '$lib/types/project';

  let {
    open,
    files,
    selectedFiles,
    saving,
    onToggle,
    onConfirm,
    onClose
  }: {
    open: boolean;
    files: OpenApiFile[];
    selectedFiles: string[];
    saving: boolean;
    onToggle: (file: string, checked: boolean) => void;
    onConfirm: () => void | Promise<void>;
    onClose: () => void;
  } = $props();
</script>

{#if open}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/35 p-4">
    <div class="w-full max-w-2xl rounded-lg border border-border bg-white shadow-xl">
      <div class="flex items-center justify-between border-b border-border px-4 py-3">
        <h2 class="text-sm font-semibold">Select OpenAPI Files</h2>
        <button
          onclick={onClose}
          class="rounded px-2 py-1 text-xs text-gray-500 hover:bg-surface-dark"
        >
          Close
        </button>
      </div>

      <div class="max-h-80 overflow-auto p-4 space-y-2">
        {#if files.length === 0}
          <div class="text-sm text-gray-500">No OpenAPI files found in this project.</div>
        {:else}
          {#each files as file}
            <label class="flex items-center gap-2 text-sm text-gray-700">
              <input
                type="checkbox"
                checked={selectedFiles.includes(file.file)}
                onchange={(e) => onToggle(file.file, e.currentTarget.checked)}
              />
              <span class="font-mono break-all">{file.file}</span>
              {#if file.title}
                <span class="text-gray-500">— {file.title}</span>
              {/if}
            </label>
          {/each}
        {/if}
      </div>

      <div class="flex items-center justify-between border-t border-border px-4 py-3">
        <span class="text-xs text-gray-500">{selectedFiles.length} file(s) selected</span>
        <button
          onclick={onConfirm}
          disabled={saving || selectedFiles.length === 0}
          class="px-3 py-1.5 bg-primary text-white rounded text-sm font-medium hover:bg-primary-light transition-colors disabled:opacity-50"
        >
          {saving ? 'Saving...' : 'Apply Selection'}
        </button>
      </div>
    </div>
  </div>
{/if}
