<script lang="ts">
    // Components
    import { Pane, Splitpanes } from "svelte-splitpanes";
    // State
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    // State
    let directory = $state("");
    let topic_search = $state("");

    // Functions
    async function get_directory() {
        const selected_path = await invoke<string>("get_log_file");
        if (selected_path != "None") {
            directory = selected_path;
        }
    }

    onMount(() => {
        get_directory();
    });
</script>

<main class="w-full flex-grow flex">
    {#if directory}
        <Splitpanes class="flex flex-row w-full">
            <Pane minSize={2} size={15}>
                <div class="flex flex-col items-center p-2 bg-gray-600 h-full">
                    <input
                        type="text"
                        bind:value={topic_search}
                        placeholder="Search"
                        class="border border-white rounded-md p-2 mb-4 text-white outline-none"
                    />
                </div>
            </Pane>
            <Pane minSize={80}>
                <div class="flex-1 flex flex-col">
                    <nav
                        class="w-full bg-gray-600 text-white px-2 py-2 flex items-center"
                    >
                        <div></div>
                        <div class="flex flex-row items-center gap-2 ml-auto">
                            <img
                                src="/SusScopeIcon.png"
                                alt="logo"
                                class="h-10 w-10"
                            />
                            <h1 class="text-2xl font-bold">SusScope</h1>
                        </div>
                    </nav>
                    
                    <div class="p-2">
                        <p>Selected Log File: {directory}</p>
                    </div>
                </div>
            </Pane>
        </Splitpanes>
    {/if}

    {#if !directory}
        <div class="min-h-screen w-full flex items-center justify-center">
            <p>Loading...</p>
        </div>
    {/if}
</main>
