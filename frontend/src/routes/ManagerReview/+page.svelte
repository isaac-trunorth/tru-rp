<script lang="ts">
	import TopBanner from '$lib/components/TopBanner.svelte';
	import { Status, UiStatus } from '$lib/model/timelogs';
	import { projects } from '$lib/stores/projects';
	import type { User } from '$lib/model/users';
	import { authStore } from '$lib/stores/user';
	import { managerStore } from '$lib/stores/manager';
	import { getDate, getMonday } from '$lib/utilities/utilities';
	import { onMount } from 'svelte';
	import check from '$lib/assets/check.png';
	import minus from '$lib/assets/minus.png';
	import Modal from '$lib/components/modal.svelte';
	import { notificationStore } from '$lib/stores/notifications';
	let showModal = false;
	const now = new Date();
	let user: User;
	let date: string = `${now.getFullYear()}-${(now.getMonth() + 1).toString().padStart(2, '0')}-${now
		.getDate()
		.toString()
		.padStart(2, '0')}`;

	$: monday = getMonday(date);
	$: totalHours = $managerStore.currentTimecard
		.filter((row) => row.key.date.toDateString() == monday.toDateString())
		.reduce(
			(partialSum, a) =>
				partialSum + a.entries.reduce((rowSum, b) => rowSum + b.info.hoursWorked, 0),
			0
		);
	$: approvedHours = $managerStore.currentTimecard
		.filter((row) => row.key.date.toDateString() == monday.toDateString())
		.reduce(
			(partialSum, a) =>
				partialSum +
				a.entries
					.filter((entry) => entry.info.submitStatus == Status.Approved)
					.reduce((rowSum, b) => rowSum + b.info.hoursWorked, 0),
			0
		);
	let selected: string;

	onMount(() => {
		managerStore.fetchEmployees($authStore);
	});

	function getTimecard() {
		if (selected == 'Select Employee for review') return;
		user = $managerStore.employees.filter((row) => row.userName == selected)[0];
		managerStore.fetchTimecard(user.id, $authStore);
	}

	function markAllApproved() {
		$managerStore.currentTimecard.forEach((row) => {
			if (row.key.date.toDateString() == monday.toDateString()) {
				row.entries.forEach((entry) => {
					if (entry.info.hoursWorked > 0 && entry.info.submitStatus == Status.Submitted) {
						entry.info.submitStatus = Status.Approved;
					}
				});
			}
		});
		$managerStore = $managerStore;
	}

	async function approveHours() {
		await managerStore.approveHours($authStore, $managerStore.currentTimecard);
		showModal = false;
		notificationStore.addNew('Approval complete', 1000);
	}
</script>

<div>
	<TopBanner />
	<div class="pl-5">
		<div class="place-content-center">
			<h1 class="font-bold">
				Manager Review:
				<input type="date" bind:value={date} />
			</h1>
			<select bind:value={selected} on:change={getTimecard}>
				<option>Select Employee for Review</option>
				{#each $managerStore.employees as reportee}
					<option>{reportee.userName}</option>
				{/each}
			</select>
			<table class="border-collapse border border-slate-800 table-auto">
				<thead>
					<tr>
						<th class="border border-slate-800">Projects</th>
						<th class="border border-slate-800">Work Code</th>
						{#each Array(7) as _, i}
							<th class="border border-slate-800 bg-slate-200 p-1">{getDate(monday, i)}</th>
						{/each}
						<th class="border border-slate-800">Total</th>
					</tr>
				</thead>
				{#each $managerStore.currentTimecard.filter((row) => row.key.date.toDateString() == monday.toDateString()) as row}
					<tr>
						<td class="border border-slate-800">
							{$projects.filter((proj) => proj.id == row.key.projectId)[0].jobDescription}
						</td>
						<td class="border border-slate-800">
							{row.key.workCode}
						</td>
						{#each Array(7) as _, i}
							{#if row.entries[i].info.submitStatus == Status.Approved}
								<td class="border border-slate-800 bg-green-200">
									{row.entries[i].info.hoursWorked}
									<button
										on:click={() => {
											row.entries[i].info.submitStatus = Status.Submitted;
											row.entries[i].status = UiStatus.Changed;
										}}
									>
										<img
											class="w-3 bg-center bg-white hover:border hover:border-black"
											src={minus}
											alt="Unapproved"
										/>
									</button>
								</td>
							{:else if row.entries[i].info.submitStatus == Status.Submitted}
								<td class="border border-slate-800 bg-yellow-200">
									{row.entries[i].info.hoursWorked}
									<button
										on:click={() => {
											row.entries[i].info.submitStatus = Status.Approved;
											row.entries[i].status = UiStatus.Changed;
										}}
									>
										<img
											class="w-3 bg-center hover:border hover:border-black"
											src={check}
											alt="Approve"
										/>
									</button>
								</td>
							{:else}
								<td class="border border-slate-800">0</td>
							{/if}
						{/each}
						<td class="border border-slate-800"
							>{row.entries.reduce((partialSum, a) => partialSum + a.info.hoursWorked, 0)}</td
						>
					</tr>
				{/each}
				<tr>
					<td class="border border-slate-800 text-right pr-5" colspan="10"
						>Submitted Hours: {totalHours}</td
					>
				</tr>
				<tr>
					<td class="border border-slate-800 text-right pr-5" colspan="10"
						>Approved Hours: {approvedHours}</td
					>
				</tr>
			</table>

			<button
				class="border hover:border-black p-1 bg-slate-200 hover:bg-transparent rounded m-2"
				on:click={markAllApproved}>Mark All Approved</button
			>

			<button
				class="border hover:border-black p-1 bg-slate-200 hover:bg-transparent rounded m-2"
				on:click={() => {
					showModal = true;
				}}>Approve Hours</button
			>
			<Modal
				title="Approve {selected}'s hours?"
				open={showModal}
				on:close={() => (showModal = false)}
			>
				<svelte:fragment slot="body">
					Total: {approvedHours} hours for the week of {monday.toDateString()}
					<button
						class="rounded bg-slate-200 hover:bg-transparent hover:border-black border m-2 p-1"
						on:click={approveHours}>Submit</button
					>
				</svelte:fragment>
			</Modal>
		</div>
	</div>
</div>
