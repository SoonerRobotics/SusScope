<script lang="ts">
  export let maxTime: number = 300; // Total duration in seconds
  export let currentTime: number = 0;
  export let markers: { time: number; label?: string }[] = [];
  export let interval: number = 15; // Interval for time markers in seconds
  export let onScrub: (time: number) => void = () => {};

  let timelineEl: HTMLButtonElement;

  function handleClick(event: MouseEvent) {
    const rect = timelineEl.getBoundingClientRect();
    const clickX = event.clientX - rect.left;
    const percent = clickX / rect.width;
    const newTime = Math.max(0, Math.min(maxTime, percent * maxTime));
    onScrub(newTime);
  }

  function handleMove(event: MouseEvent) {
    if (event.buttons !== 1) return;

    const rect = timelineEl.getBoundingClientRect();
    const moveX = event.clientX - rect.left;
    const percent = moveX / rect.width;
    const newTime = Math.max(0, Math.min(maxTime, percent * maxTime));
    currentTime = newTime;
  }

  function formatTime(seconds: number) {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  }

  $: intervalMarkers = Array.from(
    { length: Math.floor(maxTime / interval) + 1 },
    (_, i) => i * interval,
  );
</script>

<div class="mx-8 w-full">
  <button
  bind:this={timelineEl}
  onclick={handleClick}
  onmousemove={handleMove}
  class="w-full h-12 relative mt-8 bg-gray-300 rounded-md cursor-pointer"
>
  <!-- Interval Time Markers -->
  {#each intervalMarkers as time}
    <div
      class="absolute top-0 h-full w-0.5 bg-gray-500"
      style="left: {(time / maxTime) * 100}%"
    >
      <div
        class="absolute -top-5 left-1/2 -translate-x-1/2 text-xs text-gray-700"
      >
        {formatTime(time)}
      </div>
    </div>
  {/each}

  <!-- Custom Event Markers -->
  {#each markers as marker}
    {#if marker.time <= maxTime}
      <div
        class="absolute top-0 h-full w-0.5 bg-red-500"
        style="left: {(marker.time / maxTime) * 100}%"
      >
        {#if marker.label}
          <div
            class="absolute -top-7 left-1/2 -translate-x-1/2 px-2 py-0.5 text-xs bg-black text-white rounded shadow"
          >
            {marker.label}
          </div>
        {/if}
      </div>
    {/if}
  {/each}

  <!-- Current Time Indicator -->
  <div
    class="absolute top-0 h-full w-0.5 bg-blue-600"
    style="left: {(currentTime / maxTime) * 100}%"
  >
    <div
      class="absolute -top-7 left-1/2 -translate-x-1/2 px-2 py-0.5 text-xs bg-blue-600 text-white rounded shadow"
    >
      {formatTime(currentTime)}
    </div>
  </div>
</button>

</div>