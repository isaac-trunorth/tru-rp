<script lang="ts">
	export let onClick: Function;
	export let label: string;
</script>

<button
	on:click={() => onClick()}
	class="bg-blue-400 hover:bg-transparent hover:text-blue-700 font-semibold text-white py-1 px-1 border hover:border-blue-400 border-transparent rounded"
>
	{label}</button
>
