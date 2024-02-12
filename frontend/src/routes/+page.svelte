<script lang="ts">
	import TimeEntry from '$lib/components/TimeEntry.svelte';
	import ManagerReview from '$lib/components/ManagerReview.svelte';
	import NavButton from '$lib/components/NavButton.svelte';
	import ProjectOverview from '$lib/components/ProjectOverview.svelte';
	import Toast from '$lib/components/Toast.svelte';
	import TruNorth from '$lib/assets/TruNorth.jpg';
	import { authStore } from '$lib/stores/user';
	import Modal from '$lib/components/modal.svelte';
	import type { User } from '$lib/model/users';
	import UserAdministration from '$lib/components/UserAdministration.svelte';
	import Settings from '$lib/components/Settings.svelte';
	import Metrics from '$lib/components/Metrics.svelte';

	let selected = Metrics;
	let user: User = {
		id: 0,
		managerId: 0,
		firstName: '',
		lastName: '',
		userName: '',
		password: '',
		accessLevel: 0
	};

	const routeToTime = () => (selected = TimeEntry);
	const routeToMetrics = () => (selected = Metrics);
	const routeToSettings = () => (selected = Settings);
	const routeToReview = () => (selected = ManagerReview);
	const routeToProjects = () => (selected = ProjectOverview);
	const routeToUserAdmin = () => (selected = UserAdministration);
</script>

<div>
	<h1 class="text-center text-3xl font-bold underline font-arial">TruNorth Automation</h1>
	<img class="absolute top-5 right-5" src={TruNorth} alt="TruNorth Decal" />
	{#if $authStore.accessLevel <= 0}
		<Modal title="Login" open={true}>
			<svelte:fragment slot="body">
				<table>
					<tr>
						<td class="text-right"> Username: </td>
						<td>
							<input class="m-1 p-1 border rounded" bind:value={user.userName} />
						</td>
					</tr>
					<tr>
						<td class="text-right"> Password: </td>
						<td>
							<input type="password" class="m-1 p-1 border rounded" bind:value={user.password} />
						</td>
					</tr>
				</table>

				<button
					class="rounded bg-slate-200 hover:bg-transparent hover:border-black border m-2 p-1"
					on:click={() => authStore.login(user)}>Submit</button
				>
			</svelte:fragment>
		</Modal>
	{:else}
		<div class="columns-2 flex">
			<div class="break-after-column m-2 w-fit">
				<div class="pl-5 grid gap-1">
					<div>
						<NavButton onClick={() => routeToSettings()} label="Settings" />
					</div>
					<div>
						<NavButton onClick={() => routeToTime()} label="Time Entry" />
					</div>
					{#if $authStore.accessLevel > 1}
						<div>
							<NavButton onClick={() => routeToReview()} label="Manager Review" />
						</div>
						<div>
							<NavButton onClick={() => routeToProjects()} label="Project Overview" />
						</div>
						<div>
							<NavButton onClick={() => routeToMetrics()} label="Metrics" />
						</div>
						<div>
							<NavButton onClick={() => routeToUserAdmin()} label="User Administration" />
						</div>
					{/if}
				</div>
			</div>
			<div>
				<svelte:component this={selected} />
			</div>
		</div>
	{/if}
	<Toast />
</div>
