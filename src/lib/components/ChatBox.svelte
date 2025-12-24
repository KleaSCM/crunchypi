<script context="module" lang="ts">
	interface ContentPart {
		text: string;
		isMath: boolean;
		display: boolean;
		isGraph: boolean;
	}

	/**
	 * Parses a string into text, math, and graph chunks.
	 */
	function SanitizeLatex(latex: string): string | null {
		// 1. Remove \boxed{...} preserving content
		let Clean = latex.replace(/\\boxed\{([^{}]*)\}/g, "$1");
		// 2. Replace \frac{a}{b} with ((a)/(b))
		// Simple iterative approach for non-nested fractions
		while (Clean.includes("\\frac{")) {
			Clean = Clean.replace(/\\frac\{([^{}]+)\}\{([^{}]+)\}/g, "(($1)/($2))");
		}
		// 3. Remove \left, \right
		Clean = Clean.replace(/\\left|\\right/g, "");
		// 4. \cdot -> *
		Clean = Clean.replace(/\\cdot/g, "*");
		// 5. Remove backslashes generally if not caught above (e.g. \, \!)
		Clean = Clean.replace(/\\/g, "");

		// 6. Check if it looks like y = ... or f(x) = ...
		// Allow spaces, simple math chars.
		const Match = Clean.match(/^\s*(y|f\(x\))\s*=\s*([x0-9+\-*/^(). ]+)\s*$/i);
		if (Match) {
			return Match[0];
		}
		return null;
	}

	function ParseContent(text: string): ContentPart[] {
		const Parts: ContentPart[] = [];
		// 1. Math Regex: $$...$$, \[...\], $...$, \(...\)
		const MathRegex = /(\$\$[\s\S]*?\$\$)|(\\\[[\s\S]*?\\\])|(\$[\s\S]*?\$)|(\\\([\s\S]*?\\\))/g;

		// 2. Graph Regex: Heuristic for "y = ..." or "f(x) = ..." on its own line or clearly separated.
		// Capturing group 1 is the full match.
		const GraphRegex = /(?:^|\n)\s*(y\s*=\s*[x0-9+\-*/^(). ]+|f\(x\)\s*=\s*[x0-9+\-*/^(). ]+)\s*(?:\n|$)/gi;

		let LastIndex = 0;
		let Match;

		while ((Match = MathRegex.exec(text)) !== null) {
			const PreMatchText = text.slice(LastIndex, Match.index);

			// Process PreMatchText for Graphs
			let GraphLastIndex = 0;
			let GraphMatch;
			while ((GraphMatch = GraphRegex.exec(PreMatchText)) !== null) {
				if (GraphMatch.index > GraphLastIndex) {
					Parts.push({
						text: PreMatchText.slice(GraphLastIndex, GraphMatch.index),
						isMath: false,
						display: false,
						isGraph: false,
					});
				}

				Parts.push({
					text: GraphMatch[1], // The function content
					isMath: false,
					display: false,
					isGraph: true,
				});

				GraphLastIndex = GraphMatch.index + GraphMatch[0].length;
			}
			if (GraphLastIndex < PreMatchText.length) {
				Parts.push({
					text: PreMatchText.slice(GraphLastIndex),
					isMath: false,
					display: false,
					isGraph: false,
				});
			}

			// Add Math Chunk
			const FullMatch = Match[0];
			let IsDisplay = false;
			let CleanMath = FullMatch;

			if (FullMatch.startsWith("$$")) {
				IsDisplay = true;
				CleanMath = FullMatch.slice(2, -2);
			} else if (FullMatch.startsWith("\\[")) {
				IsDisplay = true;
				CleanMath = FullMatch.slice(2, -2);
			} else if (FullMatch.startsWith("$")) {
				IsDisplay = false;
				CleanMath = FullMatch.slice(1, -1);
			} else if (FullMatch.startsWith("\\(")) {
				IsDisplay = false;
				CleanMath = FullMatch.slice(2, -2);
			}

			Parts.push({
				text: CleanMath,
				isMath: true,
				display: IsDisplay,
				isGraph: false,
			});

			// NEW: Check if this Math chunk is also a graphable function
			// We try to sanitize and see if it yields a valid function string.
			const GraphCandidate = SanitizeLatex(CleanMath);
			if (GraphCandidate) {
				Parts.push({
					text: GraphCandidate,
					isMath: false,
					display: false,
					isGraph: true,
				});
			}

			LastIndex = Match.index + FullMatch.length;
		}

		// Process remaining text for Graphs
		const Remaining = text.slice(LastIndex);
		let GraphLastIndex = 0;
		let GraphMatch;

		while ((GraphMatch = GraphRegex.exec(Remaining)) !== null) {
			if (GraphMatch.index > GraphLastIndex) {
				Parts.push({
					text: Remaining.slice(GraphLastIndex, GraphMatch.index),
					isMath: false,
					display: false,
					isGraph: false,
				});
			}
			Parts.push({
				text: GraphMatch[1],
				isMath: false,
				display: false,
				isGraph: true,
			});
			GraphLastIndex = GraphMatch.index + GraphMatch[0].length;
		}
		if (GraphLastIndex < Remaining.length) {
			Parts.push({
				text: Remaining.slice(GraphLastIndex),
				isMath: false,
				display: false,
				isGraph: false,
			});
		}

		return Parts;
	}
</script>

<script lang="ts">
	import MathRenderer from "./MathRenderer.svelte";
	import GraphRenderer from "./GraphRenderer.svelte";
	import { afterUpdate } from "svelte";

	interface Message {
		role: "user" | "assistant";
		content: string;
	}

	export let messages: Message[] = [];

	let ChatContainer: HTMLElement;

	/**
	 * Auto-scroll to the bottom whenever the DOM updates (e.g. new message).
	 */
	afterUpdate(() => {
		if (ChatContainer) {
			ChatContainer.scrollTop = ChatContainer.scrollHeight;
		}
	});

	/**
	 * Copies the provided text to the clipboard.
	 * Useful for copying raw LaTeX or code blocks.
	 */
	function CopyToClipboard(text: string) {
		navigator.clipboard.writeText(text);
	}
</script>

<div class="ChatContainer" bind:this={ChatContainer}>
	{#each messages as Msg}
		<div class="Message {Msg.role}">
			<div class="Avatar">
				{#if Msg.role === "user"}
					<img src="/Yuriko.jpg" alt="User" />
				{:else}
					<img src="/crunchypi.jpg" alt="AI" />
				{/if}
			</div>
			<div class="ContentWrapper">
				<div class="Bubble">
					{#if Msg.role === "assistant"}
						<!-- Simple heuristic: text and $$math$$ or $math$ -->
						<div class="MarkdownBody">
							{#each ParseContent(Msg.content) as Part}
								{#if Part.isMath}
									<MathRenderer content={Part.text} displayMode={Part.display} />
								{:else if Part.isGraph}
									<GraphRenderer Expression={Part.text} />
								{:else}
									<span>{Part.text}</span>
								{/if}
							{/each}
						</div>
						<button class="CopyBtn" on:click={() => CopyToClipboard(Msg.content)} title="Copy Raw LaTeX">
							📄
						</button>
					{:else}
						<p>{Msg.content}</p>
					{/if}
				</div>
			</div>
		</div>
	{/each}
</div>

<style>
	.ChatContainer {
		height: 100%; /* Fill the grid cell */
		width: 100%;
		min-height: 0px; /* Explicit pixel unit */
		overflow-y: scroll !important; /* Force */
		overflow-x: hidden;
		padding: 20px;
		display: flex;
		flex-direction: column;
		gap: 20px;
		scroll-behavior: smooth;
	}

	.Message {
		display: flex;
		gap: 12px;
		max-width: 80%;
	}

	.Message.user {
		align-self: flex-end;
		flex-direction: row-reverse;
	}

	.Message.assistant {
		align-self: flex-start;
	}

	.Avatar {
		width: 42px;
		height: 42px;
		border-radius: 50%;
		overflow: hidden;
		flex-shrink: 0;
		border: 2px solid rgba(255, 255, 255, 0.2);
	}

	.Avatar img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.ContentWrapper {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
	}

	.Message.user .ContentWrapper {
		align-items: flex-end;
	}

	.Bubble {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		padding: 12px 16px;
		border-radius: 18px;
		border: 1px solid rgba(255, 255, 255, 0.1);
		color: #fff;
		position: relative;
		font-size: 0.95rem;
		line-height: 1.5;
	}

	.Message.user .Bubble {
		background: linear-gradient(135deg, #a18cd1 0%, #fbc2eb 100%);
		color: #2c1a4d;
		border: none;
		border-bottom-right-radius: 4px;
	}

	.Message.assistant .Bubble {
		border-bottom-left-radius: 4px;
	}

	.CopyBtn {
		margin-top: 8px;
		background: transparent;
		border: none;
		cursor: pointer;
		opacity: 0.5;
		font-size: 0.8rem;
		padding: 4px;
		border-radius: 4px;
		transition: opacity 0.2s;
	}

	.CopyBtn:hover {
		opacity: 1;
		background: rgba(255, 255, 255, 0.1);
	}

	.MarkdownBody {
		white-space: pre-wrap;
	}
</style>
