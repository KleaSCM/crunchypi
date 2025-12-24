<script lang="ts">
	/**
	 * Graph Renderer Component.
	 *
	 * Wraps the function-plot library to render 2D mathematical graphs.
	 *
	 * Author: KleaSCM
	 * Email: KleaSCM@gmail.com
	 */
	import { onMount, afterUpdate } from "svelte";
	import functionPlot from "function-plot";

	export let Expression: string;

	let GraphContainer: HTMLElement;
	let Width = 400;
	let Height = 250;

	function DrawGraph() {
		if (!GraphContainer || !Expression) return;

		try {
			// Clear previous graph
			GraphContainer.innerHTML = "";

			// Aggressively strip everything before and including the first '=' if present
			// This handles 'y = ', 'f(x) = ', 'plot = ', etc.
			const CleanFn = Expression.includes("=") ? Expression.split("=")[1].trim() : Expression.trim();

			functionPlot({
				target: GraphContainer,
				width: Width,
				height: Height,
				grid: true,
				data: [
					{
						fn: CleanFn,
						color: "#fbc2eb",
						graphType: "polyline",
					},
				],
				xAxis: {
					domain: [-10, 10],
					label: "x",
				},
				yAxis: {
					domain: [-10, 10],
					label: "y",
				},
				disableZoom: false,
			});
		} catch (e: any) {
			console.error("Failed to plot graph:", e);
			// Show the exact string we tried to plot and the error message
			const CleanFnForDisplay = Expression.includes("=") ? Expression.split("=")[1].trim() : Expression.trim();
			GraphContainer.innerHTML = `
				<div style="padding: 10px; border: 1px solid #ff6b6b; border-radius: 8px; background: rgba(255,0,0,0.1);">
					<div style="color: #ff6b6b; font-size: 0.8rem; font-weight: bold; margin-bottom: 4px;">Plot Error</div>
					<div style="color: #ccc; font-size: 0.75rem; font-family: monospace;">Fn: "${CleanFnForDisplay}"</div>
					<div style="color: #ff6b6b; font-size: 0.75rem; margin-top: 4px;">${e.message}</div>
				</div>`;
		}
	}

	onMount(() => {
		DrawGraph();
	});

	afterUpdate(() => {
		DrawGraph();
	});
</script>

<div class="GraphWrapper">
	<div class="Graph" bind:this={GraphContainer}></div>
	<div class="Label">{Expression}</div>
</div>

<style>
	.GraphWrapper {
		background: rgba(0, 0, 0, 0.2);
		border-radius: 12px;
		padding: 10px;
		display: inline-block;
		margin: 5px 0;
		border: 1px solid rgba(255, 255, 255, 0.1);
	}

	.Graph {
		border-radius: 8px;
		overflow: hidden;
	}

	/* Force function-plot text to be white/legible */
	:global(.function-plot text) {
		fill: #ccc !important;
	}

	:global(.function-plot .domain) {
		stroke: #555 !important;
	}

	:global(.function-plot .tick line) {
		stroke: #333 !important;
	}

	.Label {
		text-align: center;
		color: #fbc2eb;
		font-family: monospace;
		font-size: 0.9rem;
		margin-top: 5px;
	}
</style>
