<script lang="ts">
	/**
	 * Main Chat Interface Page.
	 *
	 * Orchestrates the user interaction, state management for processing,
	 * and communication with the Rust backend command `QueryOllama`.
	 *
	 * Author: KleaSCM
	 * Email: KleaSCM@gmail.com
	 */
	import { invoke } from "@tauri-apps/api/core";
	import ChatBox from "../lib/components/ChatBox.svelte";

	interface Message {
		role: "user" | "assistant";
		content: string;
	}

	let InputValue = "";
	let Messages: Message[] = [];
	let IsLoading = false;

	let InputElement: HTMLTextAreaElement;

	/**
	 * Submits the user's question to the backend.
	 *
	 * Prevents empty submissions and sets loading state to disable UI during processing.
	 */
	async function HandleSubmit() {
		// Prevent empty or duplicate simultaneous requests.
		if (!InputValue.trim() || IsLoading) {
			return;
		}

		const UserMsg = InputValue;
		InputValue = "";

		// Reset textarea height immediately
		if (InputElement) {
			InputElement.style.height = "auto";
		}

		Messages = [...Messages, { role: "user", content: UserMsg }];
		IsLoading = true;

		try {
			const Response = await invoke("QueryOllama", { prompt: UserMsg });
			Messages = [...Messages, { role: "assistant", content: Response as string }];
		} catch (Error) {
			console.error("Error:", Error);
			Messages = [...Messages, { role: "assistant", content: `Error: ${Error}` }];
		} finally {
			IsLoading = false;
		}
	}

	/**
	 * Handles keyboard shortcuts for submission.
	 *
	 * Allows Enter to submit, but Shift+Enter for new lines (if multiline were enabled,
	 * currently rows=1 but good practice).
	 */
	function HandleKeydown(e: KeyboardEvent) {
		if (e.key === "Enter" && !e.shiftKey) {
			e.preventDefault();
			HandleSubmit();
		}
	}
</script>

```
<div class="Container">
	<header>
		<h1>Crunchypi 🥧</h1>
		<p class="Subtitle"></p>
	</header>

	<ChatBox messages={Messages} />

	<div class="InputArea">
		<div class="InputWrapper">
			<!-- rows="1" to keep it compact initially -->
			<!-- Auto-expanding textarea -->
			<textarea
				bind:this={InputElement}
				bind:value={InputValue}
				on:keydown={HandleKeydown}
				on:input={(e) => {
					// Auto-expand logic
					const Target = e.target as HTMLTextAreaElement;
					Target.style.height = "auto";
					Target.style.height = Target.scrollHeight + "px";
				}}
				placeholder="Ask a math question... (Shift+Enter for new line)"
				rows="1"
				style="max-height: 200px; overflow-y: auto;"
			></textarea>
			<button disabled={IsLoading} on:click={HandleSubmit}>
				{#if IsLoading}
					<span class="Spinner"></span>
				{:else}
					➤
				{/if}
			</button>
		</div>
	</div>
</div>
```

<style>
	.Container {
		display: grid;
		grid-template-rows: auto 1fr auto; /* Header, Chat (expands), Input */
		height: 100%;
		max-width: 900px;
		margin: 0 auto;
		width: 100%;
		position: relative;
		overflow: hidden;
	}

	header {
		padding: 20px;
		text-align: center;
		flex-shrink: 0;
	}

	h1 {
		margin: 0;
		font-size: 1.8rem;
		background: linear-gradient(to right, #fbc2eb, #a18cd1);
		background-clip: text;
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
	}

	.Subtitle {
		margin: 5px 0 0;
		font-size: 0.9rem;
		color: var(--text-secondary);
	}

	.InputArea {
		padding: 20px;
		background: linear-gradient(to top, rgba(15, 12, 41, 0.9) 0%, transparent 100%);
		flex-shrink: 0;
	}

	.InputWrapper {
		background: var(--glass-bg);
		border: 1px solid var(--glass-border);
		border-radius: 24px;
		padding: 2px 4px; /* Reduced vertical padding */
		display: flex;
		align-items: center;
		backdrop-filter: blur(10px);
		box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
		transition: border-color 0.2s;
	}

	.InputWrapper:focus-within {
		border-color: var(--secondary-accent);
	}

	textarea {
		flex: 1;
		background: transparent;
		border: none;
		color: #fff;
		font-family: inherit;
		font-size: 1rem;
		padding: 2px 4px; /* Reduced vertical padding */
		resize: none;
		outline: none;
		max-height: 100px;
	}

	button {
		background: var(--secondary-accent);
		border: none;
		border-radius: 50%;
		width: 36px;
		height: 36px;
		color: #fff;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		transition:
			transform 0.2s,
			background 0.2s;
		margin-left: 8px;
	}

	button:hover:not(:disabled) {
		transform: scale(1.1);
		background: var(--primary-accent);
	}

	button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.Spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: #fff;
		border-radius: 50%;
		animation: Spin 1s linear infinite;
	}

	@keyframes Spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
