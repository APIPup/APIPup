<script lang="ts">
  import { t } from '$lib/i18n';

  interface KeyValuePair {
    key: string;
    value: string;
    enabled: boolean;
  }

  let { pairs = $bindable(), onUpdate }: {
    pairs: KeyValuePair[];
    onUpdate: (pairs: KeyValuePair[]) => void;
  } = $props();

  function updatePair(index: number, field: keyof KeyValuePair, value: string | boolean) {
    const updated = pairs.map((p, i) =>
      i === index ? { ...p, [field]: value } : p
    );
    // Auto-add empty row at the end
    const last = updated[updated.length - 1];
    if (last && (last.key !== '' || last.value !== '')) {
      updated.push({ key: '', value: '', enabled: true });
    }
    onUpdate(updated);
  }

  function removePair(index: number) {
    const updated = pairs.filter((_, i) => i !== index);
    if (updated.length === 0) {
      updated.push({ key: '', value: '', enabled: true });
    }
    onUpdate(updated);
  }
</script>

<div class="overflow-auto">
  <table class="w-full text-sm">
    <thead>
      <tr class="text-left text-gray-500">
        <th class="w-8 p-1"></th>
        <th class="p-1 font-normal">{$t('request.key')}</th>
        <th class="p-1 font-normal">{$t('request.value')}</th>
        <th class="w-8 p-1"></th>
      </tr>
    </thead>
    <tbody>
      {#each pairs as pair, i}
        <tr class="border-t border-border/50">
          <td class="p-1 text-center">
            <input
              type="checkbox"
              checked={pair.enabled}
              onchange={(e) => updatePair(i, 'enabled', e.currentTarget.checked)}
              class="accent-primary"
            />
          </td>
          <td class="p-1">
            <input
              type="text"
              value={pair.key}
              oninput={(e) => updatePair(i, 'key', e.currentTarget.value)}
              placeholder="key"
              class="w-full bg-transparent outline-none px-1 py-0.5 border border-transparent focus:border-border rounded"
            />
          </td>
          <td class="p-1">
            <input
              type="text"
              value={pair.value}
              oninput={(e) => updatePair(i, 'value', e.currentTarget.value)}
              placeholder="value"
              class="w-full bg-transparent outline-none px-1 py-0.5 border border-transparent focus:border-border rounded"
            />
          </td>
          <td class="p-1 text-center">
            {#if pair.key || pair.value}
              <button
                onclick={() => removePair(i)}
                class="text-gray-400 hover:text-red-500 text-xs"
              >✕</button>
            {/if}
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
