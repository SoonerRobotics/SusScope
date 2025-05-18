<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { Chart, type ChartConfiguration, registerables } from "chart.js";

    Chart.register(...registerables);

    export let data: { x: number; y: number }[];

    let canvas: HTMLCanvasElement;
    let chart: Chart;

    console.log(data);

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
                maintainAspectRatio: false,
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