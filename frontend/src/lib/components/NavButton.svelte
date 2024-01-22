<script lang="ts">
	export let onClick: Function;
	export let label: string;
</script>

<button
	on:click={() => onClick()}
	class="bg-slate-400 hover:bg-transparent hover:text-slate-800 font-semibold text-white py-1 px-1 border hover:border-slate-800 border-transparent rounded"
>
	{label}</button
>
