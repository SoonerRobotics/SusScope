<script lang="ts">
    // Components
    import { Pane, Splitpanes } from "svelte-splitpanes";
    import Chart from "$components/charts/xy-chart.svelte";

    // State
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    type EventType = {
        timestamp: number;
    } & (
        | {
              type: "metadata";
              event: {
                  system_state: number;
                  mobility: number;
              };
          }
        | {
              type: "position";
              event: {
                  x: number;
                  y: number;
                  theta: number;
                  latitude: number;
                  longitude: number;
              };
          }
    );

    let events: EventType[] = $state([]);
    let current_page = $state("none");
    let topic_search = $state("");

    async function read_file() {
        const contents = await invoke<any>("read_log_file");
        if (contents != "") {
            events = contents;
        }
    }

    function get_metadata_event() {
        return events.find((event) => event.type === "metadata") ?? null;
    }

    function get_unique_types() {
        const unique_types = new Set();
        events.forEach((event) => {
            if (event.type) unique_types.add(event.type);
        });
        return Array.from(unique_types);
    }

    async function set_current_page(type: string) {
        current_page = type;
    }

    function get_position_data() {
        const positions = events
            .filter((event) => event.type === "position")
            .map((event) => ({
                x: event.event.x,
                y: event.event.y,
            }));

        const filtered_positions = positions.filter((_, index) => index % 10 === 0);
        return filtered_positions;
    }

    onMount(() => {
        read_file();
    });
</script>

<main class="w-full h-screen">
    {#if events}
        <Splitpanes class="flex flex-row w-full h-full">
            <Pane minSize={2} size={15}>
                <div class="flex flex-col items-center p-2 bg-gray-600 h-full">
                    <input
                        type="text"
                        bind:value={topic_search}
                        placeholder="Search"
                        class="border border-white rounded-md p-2 mb-4 text-white outline-none"
                    />

                    <div class="h-2 bg-white w-full mb-4"></div>

                    <div class="flex flex-col items-start w-full">
                        {#each get_unique_types() as type}
                            <button
                                class="w-full bg-gray-700 text-white p-2 mb-2 rounded-md cursor-pointer hover:bg-gray-800"
                                onclick={() => set_current_page(type as string)}
                            >
                                {type}
                            </button>
                        {/each}
                    </div>
                </div>
            </Pane>
            <Pane minSize={80}>
                <div class="flex-1 flex flex-col">
                    <nav
                        class="w-full bg-gray-600 text-white px-2 py-2 flex items-center"
                    >
                        <div>
                            <h1 class="text-2xl font-bold">
                                {get_metadata_event()?.event.system_state == 2
                                    ? "Autonomous"
                                    : "Manual"}
                            </h1>
                        </div>
                        <a href="/" class="flex flex-row items-center gap-2 ml-auto">
                            <img
                                src="/SusScopeIcon.png"
                                alt="logo"
                                class="h-10 w-10"
                            />
                            <h1 class="text-2xl font-bold">SusScope</h1>
                        </a>
                    </nav>

                    <div class="p-2 w-full h-full">
                        {#if current_page === "none"}
                            <p class="text-black text-center">
                                Select a topic to view its data
                            </p>
                        {:else if current_page === "position"}
                            <div class="w-full h-[90%]">
                                <Chart data={get_position_data()} />
                            </div>
                        {/if}
                    </div>
                </div>
            </Pane>
        </Splitpanes>
    {:else}
        <div class="min-h-screen w-full flex items-center justify-center">
            <p>Loading...</p>
        </div>
    {/if}
</main>
