<script lang="ts">
  import { t } from '$lib/i18n';
  import {
    requestList,
    activeRequestId,
    addRequest,
    removeRequest,
    selectRequest
  } from '$lib/stores/requests';

  const METHOD_COLORS: Record<string, string> = {
    GET: 'text-green-600',
    POST: 'text-yellow-600',
    PUT: 'text-blue-600',
    PATCH: 'text-purple-600',
    DELETE: 'text-red-600',
    HEAD: 'text-gray-500',
    OPTIONS: 'text-gray-500'
  };

  function methodColor(method: string): string {
    return METHOD_COLORS[method] ?? 'text-gray-500';
  }

  function displayUrl(url: string): string {
    if (!url) return 'Untitled';
    try {
      const u = new URL(url);
      return u.pathname + u.search;
    } catch {
      return url;
    }
  }
</script>

<aside class="w-60 h-full bg-surface-dark border-r border-border flex flex-col">
  <div class="flex items-center justify-between p-3 border-b border-border">
    <span class="font-semibold text-sm">{$t('sidebar.title')}</span>
    <button
      onclick={addRequest}
      class="text-xs px-2 py-1 bg-primary text-white rounded hover:bg-primary-light transition-colors"
    >
      + {$t('sidebar.new_request')}
    </button>
  </div>

  <div class="flex-1 overflow-y-auto">
    {#each $requestList as req}
      <button
        class="w-full text-left px-3 py-2 flex items-center gap-2 text-sm border-b border-border/30 transition-colors
          {req.id === $activeRequestId ? 'bg-white' : 'hover:bg-white/50'}"
        onclick={() => selectRequest(req.id)}
      >
        <span class="font-mono text-xs font-bold {methodColor(req.method)} shrink-0 w-12">
          {req.method}
        </span>
        <span class="truncate text-gray-600">{displayUrl(req.url)}</span>
        {#if $requestList.length > 1}
          <button
            onclick={(e) => { e.stopPropagation(); removeRequest(req.id); }}
            class="ml-auto text-gray-400 hover:text-red-500 text-xs shrink-0"
            title={$t('sidebar.delete')}
          >✕</button>
        {/if}
      </button>
    {/each}
  </div>
</aside>
