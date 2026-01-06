<script lang="ts">
	import '../app.css';

	let { children } = $props();
</script>

<svelte:head>
	<title>TEI Scribe</title>
</svelte:head>

{@render children()}
