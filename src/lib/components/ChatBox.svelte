<script context="module" lang="ts">
	interface ContentPart {
		text: string;
		isMath: boolean;
		display: boolean;
	}

	/**
	 * Parses a string into text and math chunks.
	 *
	 * Detects LaTeX delimiters $$...$$ and $...$.
	 * Returns an array of ContentPart for rendering.
	 */
	function ParseContent(text: string): ContentPart[] {
		const Parts: ContentPart[] = [];
		// Regex to find $$...$$ (display), \[...\] (display), $...$ (inline), or \(...\) (inline)
		// qwen2-math often outputs \[ ... \] for display math.
		const Regex = /(\$\$[\s\S]*?\$\$)|(\\\[[\s\S]*?\\\])|(\$[\s\S]*?\$)|(\\\([\s\S]*?\\\))/g;

		let LastIndex = 0;
		let Match;

		while ((Match = Regex.exec(text)) !== null) {
			// Add text before match
			if (Match.index > LastIndex) {
				Parts.push({
					text: text.slice(LastIndex, Match.index),
					isMath: false,
					display: false,
				});
			}

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
			});

			LastIndex = Match.index + FullMatch.length;
		}

		// Add remaining text
		if (LastIndex < text.length) {
			Parts.push({
				text: text.slice(LastIndex),
				isMath: false,
				display: false,
			});
		}

		return Parts;
	}
</script>

<script lang="ts">
	import MathRenderer from "./MathRenderer.svelte";
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
		min-height: 0; /* Critical for Grid item scrolling */
		overflow-y: scroll; /* Force scrollbar */
		overflow-x: hidden;
		padding: 20px;
		display: flex;
		flex-direction: column;
		gap: 20px;
		scroll-behavior: smooth;
		min-height: 0; /* Important for flex items to scroll */
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
