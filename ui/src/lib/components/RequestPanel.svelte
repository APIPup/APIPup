<script lang="ts">
  import { t } from '$lib/i18n';
  import TabBar from './TabBar.svelte';
  import KeyValueEditor from './KeyValueEditor.svelte';
  import {
    activeRequest,
    updateRequest,
    loading,
    response,
    responseError
  } from '$lib/stores/requests';
  import { sendRequest } from '$lib/api/http';

  const METHODS = ['GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'HEAD', 'OPTIONS'];

  let activeTab = $state('params');

  const tabs = $derived([
    { key: 'params', label: $t('request.tab.params') },
    { key: 'headers', label: $t('request.tab.headers') },
    { key: 'body', label: $t('request.tab.body') }
  ]);

  async function handleSend() {
    const req = $activeRequest;
    if (!req || !req.url) return;

    loading.set(true);
    response.set(null);
    responseError.set(null);

    try {
      const headers: Record<string, string> = {};
      for (const h of req.headers) {
        if (h.enabled && h.key) {
          headers[h.key] = h.value;
        }
      }

      const result = await sendRequest({
        method: req.method,
        url: req.url,
        headers,
        body: req.body || null
      });

      response.set(result);
    } catch (err) {
      responseError.set(String(err));
    } finally {
      loading.set(false);
    }
  }

  function handleMethodChange(method: string) {
    if ($activeRequest) updateRequest($activeRequest.id, { method });
  }

  function handleUrlChange(url: string) {
    if ($activeRequest) updateRequest($activeRequest.id, { url });
  }

  function handleUrlKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') handleSend();
  }

  function handleHeadersUpdate(pairs: Array<{ key: string; value: string; enabled: boolean }>) {
    if ($activeRequest) updateRequest($activeRequest.id, { headers: pairs });
  }

  function handleBodyChange(body: string) {
    if ($activeRequest) updateRequest($activeRequest.id, { body });
  }
</script>

{#if $activeRequest}
  <!-- URL bar -->
  <div class="flex items-center gap-2 p-3 border-b border-border">
    <select
      value={$activeRequest.method}
      onchange={(e) => handleMethodChange(e.currentTarget.value)}
      class="px-2 py-1.5 border border-border rounded bg-white text-sm font-mono font-bold"
    >
      {#each METHODS as m}
        <option value={m}>{m}</option>
      {/each}
    </select>

    <input
      type="text"
      value={$activeRequest.url}
      oninput={(e) => handleUrlChange(e.currentTarget.value)}
      onkeydown={handleUrlKeydown}
      placeholder={$t('request.url_placeholder')}
      class="flex-1 px-3 py-1.5 border border-border rounded bg-white text-sm outline-none focus:border-primary"
    />

    <button
      onclick={handleSend}
      disabled={$loading || !$activeRequest.url}
      class="px-5 py-1.5 bg-primary text-white rounded text-sm font-medium hover:bg-primary-light transition-colors disabled:opacity-50"
    >
      {$loading ? '...' : $t('request.send')}
    </button>
  </div>

  <!-- Tabs -->
  <TabBar {tabs} active={activeTab} onSelect={(key) => (activeTab = key)} />

  <!-- Tab content -->
  <div class="flex-1 overflow-auto p-3">
    {#if activeTab === 'params'}
      <KeyValueEditor
        pairs={$activeRequest.headers}
        onUpdate={handleHeadersUpdate}
      />
    {:else if activeTab === 'headers'}
      <KeyValueEditor
        pairs={$activeRequest.headers}
        onUpdate={handleHeadersUpdate}
      />
    {:else if activeTab === 'body'}
      <textarea
        value={$activeRequest.body}
        oninput={(e) => handleBodyChange(e.currentTarget.value)}
        placeholder={$t('request.body_placeholder')}
        class="w-full h-full min-h-32 p-2 border border-border rounded bg-white font-mono text-sm resize-none outline-none focus:border-primary"
      ></textarea>
    {/if}
  </div>
{/if}
