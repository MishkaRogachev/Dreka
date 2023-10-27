<script lang="ts">
import { Chart } from 'chart.js/auto';
import { onMount } from "svelte";

import type { LinkStatus } from '$bindings/communication';

export let status: LinkStatus | undefined

let ctx: any
let chartCanvas: any
let chart: any

$: {
    if (status) {
        if (status.is_connected) {
            let time = new Date().getTime();
            addStatusLog(time, status.bytes_sent, status.bytes_received);
        } else {
            clearLogs();
        }
    }
}

function addStatusLog(time: number, tx: number, rx: number) {
    chart.data.datasets[0].data.push({ x: time, y: tx });
    chart.data.datasets[1].data.push({ x: time, y: rx });
    chart.update();
}

function clearLogs() {
    chart.data.datasets[0].data = []
    chart.data.datasets[1].data = []
    chart.update();
}

onMount(async () => {
    ctx = chartCanvas.getContext('2d');
    chart = new Chart(ctx, {
        type: 'scatter',
        data: {
            datasets: [{
                label: 'Tx',
                showLine: true,
                fill: false,
                pointStyle: false,
                //borderColor: getComputedStyle(document.documentElement).getPropertyValue('primary-focus'),
                data: [],
            }, {
                label: 'Rx',
                showLine: true,
                fill: false,
                pointStyle: false,
                //borderColor: getComputedStyle(document.documentElement).getPropertyValue('accent'),
                data: []
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            scales: {
                x: {
                    display: false,
                },
                y: {
                    display: false
                }
            },
            plugins: {
                legend: {
                    display: false
                },
            },
        }
    });
})
</script>

<div class="flex-grow max-h-8">
    <canvas bind:this={chartCanvas}></canvas>
</div>
