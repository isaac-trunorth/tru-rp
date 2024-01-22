<script lang="ts">
	import { Status, TimeCardRow, UiStatus } from '$lib/model/timelogs';
	import { timelogStore } from '$lib/stores/timelog';
	import { userStore } from '$lib/stores/user';
	import { getDate, getMonday } from '$lib/utilities/utilities';
	import { onMount } from 'svelte';
	import { projects } from '$lib/stores/projects';
	import { WorkCodesIndexer } from '$lib/model/general';
	import Modal from './modal.svelte';

	let showModal = false;

	let newProject = 'Add project';
	let showButtons = false;
	const now = new Date();
	let date: string = `${now.getFullYear()}-${(now.getMonth() + 1).toString().padStart(2, '0')}-${now
		.getDate()
		.toString()
		.padStart(2, '0')}`;

	$: monday = getMonday(date);
	$: totalHours = $timelogStore
		.filter((row) => row.key.date.toDateString() == monday.toDateString())
		.reduce(
			(partialSum, a) =>
				partialSum + a.entries.reduce((rowSum, b) => rowSum + b.info.hoursWorked, 0),
			0
		);

	onMount(async () => {
		await timelogStore.getNewLogs($userStore);
	});

	function monitor(projectName: string) {
		const nextProject = projects.filter((row) => row.name == projectName)[0];
		if (nextProject == undefined) return;
		if (nextProject.name != 'Add project' && projects.includes(nextProject)) {
			timelogStore.addLog(
				new TimeCardRow(nextProject.name, {
					date: monday,
					projectNumber: nextProject.id,
					workCode: 0
				})
			);
			newProject = 'Add project';
		}
	}

	function removeRow(row: TimeCardRow) {
		timelogStore.removeLog(row, $userStore);
	}
	function addRow(row: TimeCardRow) {
		const { date, projectNumber } = row.key;
		const newRow = new TimeCardRow(row.name, { date, projectNumber, workCode: 0 });
		timelogStore.addLog(newRow);
	}
	async function finalHourSubmission() {
		$timelogStore.forEach((row) => {
			row.entries.forEach((entry) => {
				if (entry.info.hoursWorked > 0 && entry.info.submitStatus != Status.Approved) {
					entry.info.submitStatus = Status.Submitted;
				}
				if (entry.status != UiStatus.New) entry.status = UiStatus.Changed;
			});
		});
		await timelogStore.submitLogs($userStore);
		showModal = false;
	}
</script>

<div class="place-content-center">
	<h1 class="font-bold">Employee Time Entry</h1>
	<input type="date" bind:value={date} />
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
		{#each $timelogStore.filter((row) => row.key.date.toDateString() == monday.toDateString()) as row}
			<tr>
				<td
					class="border border-slate-800"
					on:mouseenter={() => (showButtons = true)}
					on:mouseleave={() => (showButtons = false)}
				>
					{row.name}
					{#if showButtons}
						<button
							on:click={() => addRow(row)}
							class="border-slate-800 border rounded px-1 hover:bg-slate-200">+</button
						>
						<button
							on:click={() => removeRow(row)}
							class="border-slate-800 border bg-red-200 rounded px-1 hover:bg-red-400">-</button
						>
					{/if}
				</td>
				<td class="border border-slate-800">
					{#if row.key.workCode > 0}
						{WorkCodesIndexer[row.key.workCode]}
					{:else}
						<input bind:value={row.key.workCode} list="workCodes" />
						<datalist id="workCodes">
							{#each WorkCodesIndexer as workOption}
								<option value={WorkCodesIndexer.indexOf(workOption)}>{workOption}</option>
							{/each}
						</datalist>
					{/if}
				</td>
				{#each Array(7) as _, i}
					<td class="border border-slate-800">
						{#if row.entries[i].info.submitStatus != Status.Approved}
							<input
								class="w-12"
								type="number"
								bind:value={row.entries[i].info.hoursWorked}
								on:change={() => row.entries[i].UpdateStatus()}
							/>
						{:else}
							<input
								class="bg-green-200"
								disabled={true}
								bind:value={row.entries[i].info.hoursWorked}
							/>
						{/if}
					</td>
				{/each}
				<td class="border border-slate-800"
					>{row.entries.reduce((partialSum, a) => partialSum + a.info.hoursWorked, 0)}</td
				>
			</tr>
		{/each}
		<tr>
			<td
				><input
					class="text-slate-400 hover:bg-slate-200"
					bind:value={newProject}
					on:click={() => (newProject = '')}
					on:focusout={() => monitor(newProject)}
					on:keydown={(key) => {
						if (key.key == 'Enter') {
							monitor(newProject);
						}
					}}
					list="projects"
				/>
				<datalist id="projects">
					<option>Add project</option>
					{#each projects as project}
						<option>{project.name}</option>
					{/each}
				</datalist></td
			>
			<td class="border border-slate-800 text-right pr-5" colspan="9">Total Hours: {totalHours}</td>
		</tr>
	</table>
	<button
		class="border hover:border-black p-1 bg-slate-200 hover:bg-transparent rounded m-2"
		on:click={() => timelogStore.submitLogs($userStore)}>Save Entries</button
	>
	<button
		class="border hover:border-black p-1 bg-slate-200 hover:bg-transparent rounded m-2"
		on:click={() => {
			showModal = true;
		}}>Submit For Review</button
	>
	<Modal title="Submit hours?" open={showModal} on:close={() => (showModal = false)}>
		<svelte:fragment slot="body">
			Total: {totalHours} hours for the week of {monday.toDateString()}
			<button
				class="rounded bg-slate-200 hover:bg-transparent hover:border-black border m-2 p-1"
				on:click={finalHourSubmission}>Submit</button
			>
		</svelte:fragment>
	</Modal>
</div>
