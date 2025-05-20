<script lang="ts">
    import { Chart, type ChartConfiguration, registerables } from "chart.js";
    import { onDestroy, onMount } from "svelte";

    Chart.register(...registerables);

    const { data }: {
        data: { x: number; y: number }[];
    } = $props();

    let canvas: HTMLCanvasElement;
    let chart: Chart;

    onMount(() => {
        const config: ChartConfiguration = {
            type: 'scatter',
            data: {
                datasets: [{
                    label: 'Position',
                    data: data,
                    backgroundColor: 'rgba(75, 192, 192, 0.6)',
                    borderColor: 'rgba(75, 192, 192, 1)',
                    borderWidth: 1,
                    showLine: true,
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: true,
                animation: false,
                scales: {
                    x: {
                        type: 'linear',
                        position: 'bottom',
                        min: -50,
                        max: 50,
                        title: {
                            display: true,
                            text: 'X (meters)'
                        }
                    },
                    y: {
                        type: 'linear',
                        position: 'left',
                        min: -50,
                        max: 50,
                        title: {
                            display: true,
                            text: 'Y (meters)'
                        }
                    }
                }
            }
        };

        chart = new Chart(canvas, config);
    });

    onDestroy(() => {
        chart?.destroy();
    });

    $effect(() => {
        if (chart) {
            chart.data.datasets[0].data = data;
            chart.update();
        }
    })
</script>

<canvas bind:this={canvas} class="w-full h-full"></canvas>