<script lang="ts">
	import TimeEntry from '$lib/components/TimeEntry.svelte';
	import ManagerReview from '$lib/components/ManagerReview.svelte';
	import NavButton from '$lib/components/NavButton.svelte';

	let selected = TimeEntry;

	const routeToTime = () => (selected = TimeEntry);
	const routeToReview = () => (selected = ManagerReview);
</script>

<div>
	<h1 class="text-center text-3xl font-bold underline">TruNorth Automation</h1>
	<div class="columns-2">
		<div class="break-after-column">
			<div class="pl-5">
				<ul class="list-none space-y-2">
					<li>
						<NavButton onClick={() => routeToTime()} label="Time Entry" />
					</li>
					<li>
						<NavButton onClick={() => routeToReview()} label="Manager Review" />
					</li>
				</ul>
			</div>
		</div>
		<div>
			<svelte:component this={selected} />
		</div>
	</div>
</div>
