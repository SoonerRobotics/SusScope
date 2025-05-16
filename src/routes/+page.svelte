<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let directory = $state("");
  async function select_directory(event: Event) {
    event.preventDefault();
    const selected_path = await invoke<string>("select_log_file");
    if (selected_path != "None")
    {
      directory = selected_path;
    }
  }
</script>

<main class="min-h-screen w-full flex flex-col items-center justify-center text-center gap-4">
  <div class="flex flex-row gap-4 items-center justify-center">
    <img src="/SusScopeIcon.png" alt="logo" class="h-16 w-16" />
    <h1 class="text-3xl">SusScope</h1>
  </div>

  <button
    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded hover:cursor-pointer"
    onclick={select_directory}
  >
    Open Log File
  </button>

  {#if directory}
    <p class="mt-4">Selected Log File: {directory}</p>
  {/if}
</main>