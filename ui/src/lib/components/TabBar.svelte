<script lang="ts">
  interface Tab {
    key: string;
    label: string;
  }

  let { tabs, active, onSelect }: {
    tabs: Tab[];
    active: string;
    onSelect: (key: string) => void;
  } = $props();
</script>

<div class="flex border-b border-border">
  {#each tabs as tab}
    <button
      class="px-4 py-2 text-sm transition-colors {active === tab.key
        ? 'border-b-2 border-primary text-primary font-medium'
        : 'text-gray-500 hover:text-gray-700'}"
      onclick={() => onSelect(tab.key)}
    >
      {tab.label}
    </button>
  {/each}
</div>
