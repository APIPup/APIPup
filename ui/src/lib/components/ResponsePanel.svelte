<script lang="ts">
  import { t } from '$lib/i18n';
  import TabBar from './TabBar.svelte';
  import { response, responseError, loading } from '$lib/stores/requests';

  let activeTab = $state('body');

  const tabs = $derived([
    { key: 'body', label: $t('response.tab.body') },
    { key: 'headers', label: $t('response.tab.headers') }
  ]);

  function statusColor(status: number): string {
    if (status >= 200 && status < 300) return 'text-green-600';
    if (status >= 300 && status < 400) return 'text-yellow-600';
    if (status >= 400 && status < 500) return 'text-orange-600';
    return 'text-red-600';
  }

  function formatBody(body: string): string {
    try {
      return JSON.stringify(JSON.parse(body), null, 2);
    } catch {
      return body;
    }
  }
</script>

<div class="flex flex-col h-full border-t border-border">
  <!-- Status bar -->
  <div class="flex items-center gap-3 px-3 py-2 border-b border-border bg-surface-dark">
    <span class="text-sm font-semibold text-gray-600">{$t('response.title')}</span>
    {#if $loading}
      <span class="text-sm text-gray-400">Sending...</span>
    {:else if $response}
      <span class="text-sm font-mono font-bold {statusColor($response.status)}">
        {$response.status} {$response.status_text}
      </span>
      <span class="text-xs text-gray-400">{$response.elapsed_ms}ms</span>
    {/if}
  </div>

  {#if $responseError}
    <div class="p-3 text-sm text-red-600">
      <span class="font-bold">{$t('response.error')}:</span> {$responseError}
    </div>
  {:else if $response}
    <TabBar {tabs} active={activeTab} onSelect={(key) => (activeTab = key)} />

    <div class="flex-1 overflow-auto p-3">
      {#if activeTab === 'body'}
        <pre class="text-sm font-mono whitespace-pre-wrap break-all">{formatBody($response.body)}</pre>
      {:else if activeTab === 'headers'}
        <table class="w-full text-sm">
          <tbody>
            {#each Object.entries($response.headers) as [key, value]}
              <tr class="border-b border-border/30">
                <td class="py-1 pr-3 font-mono font-bold text-gray-600">{key}</td>
                <td class="py-1 font-mono text-gray-500">{value}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </div>
  {:else}
    <div class="flex-1 flex items-center justify-center text-gray-400 text-sm">
      {$t('response.empty')}
    </div>
  {/if}
</div>
