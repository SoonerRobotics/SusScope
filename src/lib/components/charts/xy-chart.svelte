<script lang="ts">
    import { Chart, type ChartConfiguration, registerables } from "chart.js";
    import { onDestroy, onMount } from "svelte";

    Chart.register(...registerables);

    export let data: { x: number; y: number }[];

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
                        title: {
                            display: true,
                            text: 'X'
                        }
                    },
                    y: {
                        title: {
                            display: true,
                            text: 'Y'
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
</script>

<canvas bind:this={canvas} class="w-full h-full"></canvas>