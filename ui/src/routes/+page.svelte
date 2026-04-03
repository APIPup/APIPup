<script lang="ts">
  import Sidebar from '$lib/components/Sidebar.svelte';
  import RequestPanel from '$lib/components/RequestPanel.svelte';
  import ResponsePanel from '$lib/components/ResponsePanel.svelte';
</script>

<div class="flex h-screen overflow-hidden">
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
