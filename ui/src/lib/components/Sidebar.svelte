<script lang="ts">
  import { t } from '$lib/i18n';
  import { activeRequest, createRequestFromEndpoint, selectRequest } from '$lib/stores/requests';
  import { activeTab, selectedFiles, setActiveSpecTab, specByFile } from '$lib/stores/project';
  import type { OpenApiSpec } from '$lib/types/project';

  interface EndpointItem {
    file: string;
    method: string;
    path: string;
    summary: string;
    operation: Record<string, unknown>;
  }

  const OPENAPI_METHODS: string[] = ['get', 'post', 'put', 'patch', 'delete', 'head', 'options', 'trace'];

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

  function endpointId(endpoint: EndpointItem): string {
    return `${endpoint.file}::${endpoint.method}::${endpoint.path}`;
  }

  function resolveParameterRef(spec: OpenApiSpec, parameterLike: unknown): Record<string, unknown> | null {
    if (!parameterLike || typeof parameterLike !== 'object') {
      return null;
    }

    const parameter = parameterLike as Record<string, unknown>;
    const refValue = parameter.$ref;
    if (typeof refValue !== 'string' || !refValue.startsWith('#/components/parameters/')) {
      return parameter;
    }

    const refName = refValue.slice('#/components/parameters/'.length);
    if (!refName) {
      return null;
    }

    const componentsValue = spec.components;
    if (!componentsValue || typeof componentsValue !== 'object') {
      return null;
    }

    const parametersValue = (componentsValue as Record<string, unknown>).parameters;
    if (!parametersValue || typeof parametersValue !== 'object') {
      return null;
    }

    const referencedParameter = (parametersValue as Record<string, unknown>)[refName];
    if (!referencedParameter || typeof referencedParameter !== 'object') {
      return null;
    }

    return referencedParameter as Record<string, unknown>;
  }

  function mergePathItemParameters(
    spec: OpenApiSpec,
    pathItem: Record<string, unknown>,
    operation: Record<string, unknown>
  ): Record<string, unknown> {
    const mergedOperation: Record<string, unknown> = { ...operation };

    const pathParameters = Array.isArray(pathItem.parameters)
      ? pathItem.parameters
          .map((value) => resolveParameterRef(spec, value))
          .filter((value): value is Record<string, unknown> => Boolean(value))
      : [];

    const operationParameters = Array.isArray(operation.parameters)
      ? operation.parameters
          .map((value) => resolveParameterRef(spec, value))
          .filter((value): value is Record<string, unknown> => Boolean(value))
      : [];

    if (pathParameters.length === 0 && operationParameters.length === 0) {
      return mergedOperation;
    }

    const operationParameterKeys = new Set(
      operationParameters.map((value) => {
        const parameter = value as Record<string, unknown>;
        return `${String(parameter.in ?? '')}::${String(parameter.name ?? '')}`;
      })
    );

    const mergedParameters = [
      ...pathParameters.filter((value) => {
        const parameter = value as Record<string, unknown>;
        const key = `${String(parameter.in ?? '')}::${String(parameter.name ?? '')}`;
        return !operationParameterKeys.has(key);
      }),
      ...operationParameters
    ];

    mergedOperation.parameters = mergedParameters;
    return mergedOperation;
  }

  function collectEndpoints(file: string, spec: OpenApiSpec | null): EndpointItem[] {
    if (!spec) {
      return [];
    }

    const paths = spec.paths;
    if (!paths || typeof paths !== 'object') {
      return [];
    }

    const endpoints: EndpointItem[] = [];

    for (const [path, pathItemValue] of Object.entries(paths as Record<string, unknown>)) {
      if (!pathItemValue || typeof pathItemValue !== 'object') {
        continue;
      }

      const pathItem = pathItemValue as Record<string, unknown>;

      for (const method of OPENAPI_METHODS) {
        const operationValue = pathItem[method];
        if (!operationValue || typeof operationValue !== 'object') {
          continue;
        }

        const operation = mergePathItemParameters(spec, pathItem, operationValue as Record<string, unknown>);
        const summaryValue = operation.summary;

        endpoints.push({
          file,
          method: method.toUpperCase(),
          path,
          summary: typeof summaryValue === 'string' ? summaryValue : '',
          operation
        });
      }
    }

    return endpoints;
  }

  const activeSpec = $derived.by(() => {
    if (!$activeTab) {
      return null;
    }

    const spec = $specByFile[$activeTab];
    return spec ?? null;
  });

  const endpointItems = $derived.by(() => {
    if (!$activeTab) {
      return [];
    }

    return collectEndpoints($activeTab, activeSpec as OpenApiSpec | null);
  });

  function selectEndpoint(endpoint: EndpointItem): void {
    const request = createRequestFromEndpoint(
      endpoint.file,
      endpoint.path,
      endpoint.method,
      endpoint.operation
    );
    selectRequest(request);
  }

  function isEndpointSelected(endpoint: EndpointItem): boolean {
    if (!$activeRequest) {
      return false;
    }

    return (
      $activeRequest.file === endpoint.file
      && $activeRequest.path === endpoint.path
      && $activeRequest.method.toUpperCase() === endpoint.method
    );
  }

  function addEndpoint(): void {
    if (!$activeTab) {
      return;
    }

    const newPath = `/new-path-${Date.now()}`;
    const newMethod = 'GET';
    const newOperation: Record<string, unknown> = {
      summary: 'New operation',
      responses: {
        default: {
          description: 'Default response'
        }
      }
    };

    specByFile.update((map) => {
      const currentSpec = map[$activeTab] ?? {};
      const nextSpec: Record<string, unknown> = { ...currentSpec };
      const pathsValue = nextSpec.paths;
      const paths = (pathsValue && typeof pathsValue === 'object'
        ? { ...(pathsValue as Record<string, unknown>) }
        : {}) as Record<string, unknown>;

      const pathItemValue = paths[newPath];
      const pathItem = (pathItemValue && typeof pathItemValue === 'object'
        ? { ...(pathItemValue as Record<string, unknown>) }
        : {}) as Record<string, unknown>;

      pathItem[newMethod.toLowerCase()] = newOperation;
      paths[newPath] = pathItem;
      nextSpec.paths = paths;

      return {
        ...map,
        [$activeTab]: nextSpec
      };
    });

    selectRequest(createRequestFromEndpoint($activeTab, newPath, newMethod, newOperation));
  }
</script>

<aside class="w-60 h-full bg-surface-dark border-r border-border flex flex-col">
  <div class="flex items-center justify-between p-3 border-b border-border">
    <span class="font-semibold text-sm">{$t('sidebar.title')}</span>
    <button
      onclick={addEndpoint}
      class="text-xs px-2 py-1 bg-primary text-white rounded hover:bg-primary-light transition-colors"
    >
      + {$t('sidebar.new_request')}
    </button>
  </div>

  <div class="flex-1 overflow-y-auto">
    {#if $selectedFiles.length > 0}
      <div class="p-2 space-y-1">
        {#if $selectedFiles.length > 1}
          <div class="text-[11px] font-semibold uppercase tracking-wide text-gray-500 px-1 pt-1">Specs</div>
          {#each $selectedFiles as file}
            <button
              class="w-full text-left px-2 py-1 rounded text-xs font-mono transition-colors
                {$activeTab === file ? 'bg-white text-gray-800' : 'text-gray-600 hover:bg-white/50'}"
              onclick={() => setActiveSpecTab(file)}
            >
              {file}
            </button>
          {/each}
        {/if}

        {#if endpointItems.length > 0}
          <div class="text-[11px] font-semibold uppercase tracking-wide text-gray-500 px-1 pt-2">Endpoints</div>
          <div class="space-y-1 pr-1">
            {#each endpointItems as endpoint (endpointId(endpoint))}
              <button
                class="w-full text-left px-2 py-1 rounded border transition-colors
                  {isEndpointSelected(endpoint)
                    ? 'bg-white border-primary/40 ring-1 ring-primary/20'
                    : 'bg-white/60 border-border/40 hover:bg-white'}"
                onclick={() => selectEndpoint(endpoint)}
              >
                <div class="flex items-center gap-2 min-w-0">
                  <span class="text-[10px] font-mono font-bold {methodColor(endpoint.method)}">{endpoint.method}</span>
                  <span class="text-[11px] text-gray-700 truncate">{endpoint.path}</span>
                </div>
                {#if endpoint.summary}
                  <div class="text-[11px] text-gray-500 truncate mt-0.5">{endpoint.summary}</div>
                {/if}
              </button>
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  </div>
</aside>
