<script lang="ts">
	import { goto } from '$app/navigation';
	import NavButton from '$lib/components/NavButton.svelte';
	import { authStore } from '$lib/stores/user';
	import TruNorth from '$lib/assets/TruNorth.jpg';
	import Toast from '$lib/components/Toast.svelte';
</script>

<div>
	<h1 class="text-center text-3xl font-bold underline font-arial">TruNorth Automation</h1>
	<img class="absolute top-5 right-5" src={TruNorth} alt="TruNorth Decal" />
	<div class="break-after-column m-2 w-fit">
		<div class="pl-5 grid grid-flow-col auto-cols-fr gap-1">
			<div>
				<NavButton onClick={() => goto('/TimeEntry')} label="Time Entry" />
			</div>
			{#if $authStore.accessLevel > 1}
				<div>
					<NavButton onClick={() => goto('/ManagerReview')} label="Manager Review" />
				</div>
				<div>
					<NavButton onClick={() => goto('/ProjectOverview')} label="Project Overview" />
				</div>
				<div>
					<NavButton onClick={() => goto('/Metrics')} label="Metrics" />
				</div>
				<div>
					<NavButton onClick={() => goto('/UserAdministration')} label="User Administration" />
				</div>
			{/if}
			<div>
				<NavButton onClick={() => goto('/Settings')} label="Settings" />
			</div>
		</div>
	</div>
	<div class="min-h-8">
		<Toast />
	</div>
</div>
