<script lang="ts">
	/**
	 * MathRenderer Component.
	 *
	 * Wraps KaTeX to render LaTeX strings into HTML.
	 * Handles error catching for invalid LaTeX to prevent app crashes.
	 *
	 * Author: KleaSCM
	 * Email: KleaSCM@gmail.com
	 */
	import katex from "katex";
	import "katex/dist/katex.min.css";

	export let content: string = "";
	export let displayMode: boolean = false;

	let Html = "";

	// Reactive block to re-render when content changes.
	$: {
		try {
			Html = katex.renderToString(content, {
				displayMode,
				throwOnError: false,
			});
		} catch (e) {
			Html = `<span class="Error">Error rendering math</span>`;
			console.error(e);
		}
	}
</script>

<span class="MathContent">
	{@html Html}
</span>

<style>
	.MathContent {
		font-size: 1.1em;
	}

	:global(.Error) {
		color: #ff6b6b;
	}
</style>
