<script lang="ts">
	export let header = 'Header';
	let showContents = false;
</script>

<div>
	<button on:click={() => (showContents = !showContents)}>
		{#if !showContents}
			<h1 class="font-bold hover:bg-slate-100">{header} &#x2193;</h1>
		{:else}
			<h1 class="font-bold underline hover:bg-slate-100">{header}</h1>
		{/if}
	</button>
	{#if showContents}
		<div>
			<slot />
		</div>
	{/if}
</div>
