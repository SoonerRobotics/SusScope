<script lang="ts">
    // Components
    import LatLonChart from "$components/charts/lat-long-map.svelte";
    import Chart from "$components/charts/xy-chart.svelte";
    import Timeline from "$components/timeline.svelte";
    import { Pane, Splitpanes } from "svelte-splitpanes";
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
    let sorted_event_map = $state<any>({});
    let show_gps_position_view = $state(true);
    let position_view = $state(1);
    let current_page = $state("dashboard");
    let topic_search = $state("");

    // dashboard stuff
    let current_time = $state(0);
    let current_state = $state(0);

    async function read_file() {
        const contents = await invoke<any>("read_log_file");
        if (contents != "") {
            events = contents;
        }

        // Sort events by timestamp
        events.sort((a, b) => a.timestamp - b.timestamp);

        // Create a map of events by type
        events.forEach((event) => {
            if (!sorted_event_map[event.type]) {
                sorted_event_map[event.type] = [];
            }
            sorted_event_map[event.type].push(event);
        });
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
        return sorted_event_map.position.map((event: any) => ({
            x: event.event.x,
            y: event.event.y,
        }));
    }

    function truncate_decimals(num: number, decimalPlaces: number): number {
        const factor = Math.pow(10, decimalPlaces);
        return Math.floor(num * factor) / factor;
    }

    function get_position_map_data() {
        return sorted_event_map.position
            .filter((item: any, index: number) => index % 100 == 0)
            .map((event: any) => ({
                latitude: event.event.latitude,
                longitude: event.event.longitude,
                theta: event.event.theta,
            }));
    }

    function get_gps_data() {
        return sorted_event_map.position.map((event: any) => ({
            latitude: event.event.latitude,
            longitude: event.event.longitude,
        }));
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
                        <button
                            class={`w-full text-white p-2 mb-2 rounded-md text-left text-lg cursor-pointer hover:bg-gray-800 ${
                                current_page === "dashboard"
                                    ? "bg-gray-800"
                                    : "bg-gray-700"
                            }`}
                            onclick={() => set_current_page("dashboard")}
                        >
                            Dashboard
                        </button>
                        {#each get_unique_types() as type}
                            <button
                                class={`w-full text-white p-2 mb-2 rounded-md text-left text-lg cursor-pointer hover:bg-gray-800 ${
                                    current_page === type
                                        ? "bg-gray-800"
                                        : "bg-gray-700"
                                }`}
                                onclick={() => set_current_page(type as string)}
                            >
                                {type}
                            </button>
                        {/each}
                    </div>
                </div>
            </Pane>
            <Pane minSize={80}>
                <div class="flex-1 flex flex-col h-full">
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
                        <a
                            href="/"
                            class="flex flex-row items-center gap-2 ml-auto"
                        >
                            <img
                                src="/SusScopeIcon.png"
                                alt="logo"
                                class="h-10 w-10"
                            />
                            <h1 class="text-2xl font-bold">SusScope</h1>
                        </a>
                    </nav>

                    <div class="p-2 w-full h-full">
                        {#if current_page === "dashboard"}
                            <div class="flex flex-col">
                                <div class="flex flex-row">
                                    <div class="flex flex-col">
                                        <!-- play, stop, restart -->
                                        <button
                                            class="bg-gray-700 text-white p-2 mb-2 rounded-md text-sm cursor-pointer hover:opacity-90"
                                            onclick={() => {
                                                current_time = 0;
                                                current_state = 0;
                                            }}
                                        >
                                            Restart
                                        </button>
                                        <button
                                            class="bg-gray-700 text-white p-2 mb-2 rounded-md text-sm cursor-pointer hover:opacity-90"
                                            onclick={() => {
                                                current_state = 1;
                                            }}
                                        >
                                            Play
                                        </button>
                                        <button
                                            class="bg-gray-700 text-white p-2 mb-2 rounded-md text-sm cursor-pointer hover:opacity-90"
                                            onclick={() => {
                                                current_state = 0;
                                            }}
                                        >
                                            Pause
                                        </button>
                                    </div>

                                    <!-- <p>{`${events[events.length - 1]}`}</p> -->
                                    <Timeline
                                        maxTime={300}
                                        currentTime={current_time}
                                        onScrub={(time: number) => {
                                            current_time = time;
                                        }}
                                    />
                                </div>
                            </div>
                        {:else if current_page === "position"}
                            <div class="w-full h-full flex flex-col">
                                <div
                                    class="w-full flex flex-row justify-between"
                                >
                                    <div>
                                        <button
                                            class={`
                                            bg-gray-700 text-white p-2 mb-2 rounded-md cursor-pointer hover:opacity-90 ${
                                                position_view === 0
                                                    ? "bg-red-400"
                                                    : ""
                                            }`}
                                            onclick={() => (position_view = 0)}
                                        >
                                            XY View
                                        </button>
                                        <button
                                            class={`
                                            bg-gray-700 text-white p-2 mb-2 rounded-md cursor-pointer hover:opacity-90 ${
                                                position_view === 1
                                                    ? "bg-red-400"
                                                    : ""
                                            }`}
                                            onclick={() => (position_view = 1)}
                                        >
                                            Map View
                                        </button>
                                    </div>
                                    {#if position_view === 1}
                                        <div class="flex items-center">
                                            <label
                                                class="inline-flex items-center cursor-pointer"
                                            >
                                                <input
                                                    type="checkbox"
                                                    bind:checked={
                                                        show_gps_position_view
                                                    }
                                                    class="sr-only peer"
                                                />
                                                <div
                                                    class="relative w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600 dark:peer-checked:bg-blue-600"
                                                ></div>
                                                <span
                                                    class="ms-3 text-sm font-medium text-black"
                                                >
                                                    Show GPS
                                                </span>
                                            </label>
                                        </div>
                                    {/if}
                                </div>
                                {#if position_view === 0}
                                    <Chart data={get_position_data()} />
                                {:else if position_view === 1}
                                    <LatLonChart
                                        data={get_position_map_data()}
                                        gps_data={get_gps_data()}
                                        show_gps={show_gps_position_view}
                                    />
                                {/if}
                            </div>
                        {:else if current_page === "log"}
                            <div
                                class="w-full h-full max-h-full overflow-y-auto"
                            >
                                <table class="w-full table-auto">
                                    <thead>
                                        <tr>
                                            <th class="px-4 py-2">Timestamp</th>
                                            <th class="px-4 py-2">Level</th>
                                            <th class="px-4 py-2">Node</th>
                                            <th class="px-4 py-2">Function</th>
                                            <th class="px-4 py-2">Message</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {#each sorted_event_map[current_page] as event}
                                            <tr>
                                                <td class="border px-4 py-2">
                                                    {truncate_decimals(
                                                        event.timestamp,
                                                        4,
                                                    )}
                                                </td>
                                                <td class="border px-4 py-2">
                                                    {event.event.level}
                                                </td>
                                                <td class="border px-4 py-2">
                                                    {event.event.node}
                                                </td>
                                                <td class="border px-4 py-2">
                                                    {event.event
                                                        .function_caller}
                                                </td>
                                                <td class="border px-4 py-2">
                                                    {event.event.message}
                                                </td>
                                            </tr>
                                        {/each}
                                    </tbody>
                                </table>
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
