<script lang="ts">
	import { flip } from 'svelte/animate';
	import { fly } from 'svelte/transition';
	import { notificationStore } from '$lib/stores/notifications';
</script>

<div class="z-50">
	{#each $notificationStore as notification ($notificationStore.indexOf(notification))}
		<div
			class="bg-yellow-200 rounded flex justify-center font-bold"
			animate:flip
			transition:fly={{ y: 30 }}
		>
			{notification}
		</div>
	{/each}
</div>
