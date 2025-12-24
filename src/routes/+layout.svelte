<script>
	/**
	 * Root Layout Component.
	 *
	 * Applies global styles and defines the main application structure.
	 *
	 * Author: KleaSCM
	 * Email: KleaSCM@gmail.com
	 */
	import "../lib/styles.css";
</script>

<div class="AppLayout">
	<slot />
</div>

<style>
	.AppLayout {
		width: 100vw;
		height: 100vh;
		display: flex;
		flex-direction: column;
		overflow: hidden; /* Strict viewport clipping */
		background: radial-gradient(circle at 50% 10%, rgba(161, 140, 209, 0.15) 0%, transparent 60%);
	}
</style>
