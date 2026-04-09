<script lang="ts">
  import { onDestroy } from 'svelte';

  import { t } from '$lib/i18n';
  import { saveOperation } from '$lib/api/spec';
  import TabBar from './TabBar.svelte';
  import KeyValueEditor from './KeyValueEditor.svelte';
  import {
    activeRequest,
    updateActiveRequest,
    loading,
    response,
    responseError,
    type EditablePair
  } from '$lib/stores/requests';
  import { updateSpecEndpoint } from '$lib/stores/project';
  import { sendRequest } from '$lib/api/http';

  const METHODS = ['GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'HEAD', 'OPTIONS'];
  const AUTO_SAVE_DEBOUNCE_MS = 300;

  let activeTab = $state('params');
  let autoSaving = $state(false);
  let autoSaveError = $state<string | null>(null);
  let saveTimer: ReturnType<typeof setTimeout> | null = null;
  let saveVersion = 0;

  const tabs = $derived([
    { key: 'params', label: $t('request.tab.params') },
    { key: 'headers', label: $t('request.tab.headers') },
    { key: 'body', label: $t('request.tab.body') }
  ]);

  const responseItems = $derived.by(() => {
    const operation = $activeRequest?.operation;
    if (!operation) {
      return [] as Array<{ status: string; description: string }>;
    }

    const responsesValue = operation.responses;
    if (!responsesValue || typeof responsesValue !== 'object') {
      return [] as Array<{ status: string; description: string }>;
    }

    return Object.entries(responsesValue as Record<string, unknown>).map(([status, responseValue]) => {
      const description =
        responseValue && typeof responseValue === 'object' && typeof (responseValue as Record<string, unknown>).description === 'string'
          ? ((responseValue as Record<string, unknown>).description as string)
          : '';

      return {
        status,
        description
      };
    });
  });

  function buildParameters(pairs: EditablePair[], location: string): Record<string, unknown>[] {
    return pairs
      .filter((item) => item.enabled && item.key.trim().length > 0)
      .map((item) => ({
        name: item.key,
        in: location,
        required: location === 'path',
        schema: {
          type: 'string'
        },
        example: item.value
      }));
  }

  function buildOperationFromRequest(request: {
    operation: Record<string, unknown>;
    queryParams: EditablePair[];
    pathParams: EditablePair[];
    cookieParams: EditablePair[];
    headers: EditablePair[];
    body: string;
  }): Record<string, unknown> {
    const operation: Record<string, unknown> = { ...request.operation };

    const originalParameters = Array.isArray(operation.parameters)
      ? operation.parameters.filter((value) => {
          if (!value || typeof value !== 'object') {
            return false;
          }

          const parameter = value as Record<string, unknown>;
          return !['query', 'path', 'cookie', 'header'].includes(String(parameter.in ?? ''));
        })
      : [];

    const allParameters = [
      ...buildParameters(request.queryParams, 'query'),
      ...buildParameters(request.pathParams, 'path'),
      ...buildParameters(request.cookieParams, 'cookie'),
      ...buildParameters(request.headers, 'header')
    ];

    const mergedParameters = [...originalParameters, ...allParameters];
    if (mergedParameters.length > 0) {
      operation.parameters = mergedParameters;
    } else {
      delete operation.parameters;
    }

    if (request.body.trim().length === 0) {
      delete operation.requestBody;
      return operation;
    }

    let bodyExample: unknown = request.body;
    try {
      bodyExample = JSON.parse(request.body);
    } catch {
      bodyExample = request.body;
    }

    operation.requestBody = {
      content: {
        'application/json': {
          example: bodyExample
        }
      }
    };

    return operation;
  }

  function applyRequestUpdate(updates: {
    method?: string;
    url?: string;
    queryParams?: EditablePair[];
    pathParams?: EditablePair[];
    cookieParams?: EditablePair[];
    headers?: EditablePair[];
    body?: string;
  }): void {
    const current = $activeRequest;
    if (!current) {
      return;
    }

    const nextMethod = updates.method ?? current.method;
    const nextUrl = updates.url ?? current.url;
    const nextQueryParams = updates.queryParams ?? current.queryParams;
    const nextPathParams = updates.pathParams ?? current.pathParams;
    const nextCookieParams = updates.cookieParams ?? current.cookieParams;
    const nextHeaders = updates.headers ?? current.headers;
    const nextBody = updates.body ?? current.body;

    const nextOperation = buildOperationFromRequest({
      operation: current.operation,
      queryParams: nextQueryParams,
      pathParams: nextPathParams,
      cookieParams: nextCookieParams,
      headers: nextHeaders,
      body: nextBody
    });

    updateSpecEndpoint(
      current.file,
      current.path,
      current.method,
      nextUrl,
      nextMethod,
      nextOperation
    );

    updateActiveRequest({
      method: nextMethod,
      url: nextUrl,
      path: nextUrl,
      queryParams: nextQueryParams,
      pathParams: nextPathParams,
      cookieParams: nextCookieParams,
      headers: nextHeaders,
      body: nextBody,
      operation: nextOperation
    });

    scheduleAutoSave(current.file, nextUrl, nextMethod, nextOperation);
  }

  function scheduleAutoSave(
    file: string,
    path: string,
    method: string,
    operation: Record<string, unknown>
  ): void {
    if (saveTimer) {
      clearTimeout(saveTimer);
    }

    autoSaveError = null;
    const nextVersion = saveVersion + 1;
    saveVersion = nextVersion;

    saveTimer = setTimeout(async () => {
      autoSaving = true;

      try {
        await saveOperation({
          file,
          path,
          method,
          operation
        });

        if (saveVersion !== nextVersion) {
          return;
        }

        autoSaveError = null;
      } catch (err) {
        if (saveVersion !== nextVersion) {
          return;
        }

        autoSaveError = String(err);
      } finally {
        if (saveVersion === nextVersion) {
          autoSaving = false;
        }
      }
    }, AUTO_SAVE_DEBOUNCE_MS);
  }

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
    applyRequestUpdate({ method });
  }

  function handleUrlChange(url: string) {
    applyRequestUpdate({ url });
  }

  function handleUrlKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') handleSend();
  }

  function handleQueryParamsUpdate(pairs: EditablePair[]) {
    applyRequestUpdate({ queryParams: pairs });
  }

  function handlePathParamsUpdate(pairs: EditablePair[]) {
    applyRequestUpdate({ pathParams: pairs });
  }

  function handleCookieParamsUpdate(pairs: EditablePair[]) {
    applyRequestUpdate({ cookieParams: pairs });
  }

  function handleHeadersUpdate(pairs: EditablePair[]) {
    applyRequestUpdate({ headers: pairs });
  }

  function handleBodyChange(body: string) {
    applyRequestUpdate({ body });
  }

  onDestroy(() => {
    if (saveTimer) {
      clearTimeout(saveTimer);
    }
  });
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

    {#if autoSaving}
      <span class="text-[11px] text-gray-500">{$t('request.autosave_saving')}</span>
    {:else if autoSaveError}
      <span class="text-[11px] text-red-600 truncate" title={autoSaveError}>
        {$t('request.autosave_error')}
      </span>
    {/if}
  </div>

  <div class="px-3 py-2 border-b border-border bg-surface-dark/40 space-y-1">
    {#if typeof $activeRequest.operation.summary === 'string' && $activeRequest.operation.summary.trim().length > 0}
      <div class="text-xs text-gray-700">
        <span class="font-semibold">{$t('request.detail.summary')}: </span>
        <span>{$activeRequest.operation.summary}</span>
      </div>
    {/if}

    {#if typeof $activeRequest.operation.description === 'string' && $activeRequest.operation.description.trim().length > 0}
      <div class="text-xs text-gray-600 whitespace-pre-wrap">
        <span class="font-semibold">{$t('request.detail.description')}: </span>
        <span>{$activeRequest.operation.description}</span>
      </div>
    {/if}

    {#if responseItems.length > 0}
      <div class="pt-1">
        <div class="text-[11px] font-semibold uppercase tracking-wide text-gray-500">{$t('request.detail.responses')}</div>
        <div class="mt-1 space-y-1">
          {#each responseItems as item}
            <div class="text-xs text-gray-600 flex items-start gap-2">
              <span class="font-mono text-gray-700 min-w-9">{item.status}</span>
              <span class="truncate">{item.description || '-'}</span>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>

  <!-- Tabs -->
  <TabBar {tabs} active={activeTab} onSelect={(key) => (activeTab = key)} />

  <!-- Tab content -->
  <div class="flex-1 overflow-auto p-3">
    {#if activeTab === 'params'}
      <div class="space-y-4">
        <div>
          <div class="text-[11px] font-semibold uppercase tracking-wide text-gray-500 mb-1">{$t('request.params.query')}</div>
          <KeyValueEditor pairs={$activeRequest.queryParams} onUpdate={handleQueryParamsUpdate} />
        </div>

        <div>
          <div class="text-[11px] font-semibold uppercase tracking-wide text-gray-500 mb-1">{$t('request.params.path')}</div>
          <KeyValueEditor pairs={$activeRequest.pathParams} onUpdate={handlePathParamsUpdate} />
        </div>

        <div>
          <div class="text-[11px] font-semibold uppercase tracking-wide text-gray-500 mb-1">{$t('request.params.cookie')}</div>
          <KeyValueEditor pairs={$activeRequest.cookieParams} onUpdate={handleCookieParamsUpdate} />
        </div>
      </div>
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
