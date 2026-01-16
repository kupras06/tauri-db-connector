<script lang="ts">
	import './layout.css';
	const { children } = $props();
</script>

<main class="min-h-screen">
	{@render children()}
</main>
