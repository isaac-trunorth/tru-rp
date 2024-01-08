<script lang="ts">
	import Navbar from '$lib/components/navbar.svelte';
	import { goto } from '$app/navigation';

	const routeToTime = () => routeToPage('timecard/entry', false);
	const routeToReview = () => routeToPage('timecard/review', false);
	function routeToPage(route: string, replaceState: boolean): null {
		goto(`/${route}`, { replaceState });
		return null;
	}
</script>

<div>
	<h1 class="text-center text-3xl font-bold underline">TruNorth Automation</h1>
	<div class="columns-2">
		<div class="break-after-column">
			<div class="pl-5">
				<button on:click={routeToTime}>Time Entry</button>
				<button on:click={routeToReview}>Manager Review</button>
			</div>
		</div>
		<div>
			<p>Page content</p>
		</div>
	</div>
</div>
