<script lang="ts">
	import { Status, TimeCardRow, UiStatus, WorkCode } from '$lib/model/timelogs';
	import { timelogStore } from '$lib/stores/timelog';
	import { authStore } from '$lib/stores/user';
	import { getDate, getMonday, getProjectId } from '$lib/utilities/utilities';
	import { onMount } from 'svelte';
	import { projects } from '$lib/stores/projects';
	import { WorkCodesIndexer } from '$lib/model/general';
	import Modal from './modal.svelte';
	import { notificationStore } from '$lib/stores/notifications';

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
		await projects.getProjects($authStore);
		await timelogStore.getNewLogs($authStore);
	});

	function monitor(projectName: string) {
		const nextProject = $projects.filter(
			(row) => `${row.jobNumber} - ${row.jobDescription}` == projectName
		)[0];
		if (nextProject == undefined) return;
		if (nextProject.jobDescription != 'Add project' && $projects.includes(nextProject)) {
			timelogStore.addLog(
				new TimeCardRow({
					date: monday,
					projectId: nextProject.id,
					workCode: WorkCode.Unset,
					userId: $authStore.userId
				})
			);
			newProject = 'Add project';
		}
	}

	async function removeRow(row: TimeCardRow) {
		await timelogStore.removeLog(row, $authStore);
		notificationStore.addNew('Logs removed');
	}
	function addRow(row: TimeCardRow) {
		const { date, projectId } = row.key;
		const newRow = new TimeCardRow({
			date,
			projectId,
			workCode: WorkCode.Unset,
			userId: $authStore.userId
		});
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
		await timelogStore.submitLogs($authStore);
		notificationStore.addNew('Submission successfull', 1000);
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
					{`${getProjectId($projects, row.key.projectId)} - ${
						$projects.filter((proj) => proj.id == row.key.projectId)[0].jobDescription
					}`}
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
					{#if row.key.workCode != WorkCode.Unset && row.key.workCode.toString() != ''}
						{row.key.workCode}
					{:else}
						<select
							bind:value={row.key.workCode}
							on:click={() => (row.key.workCode = WorkCode[''])}
						>
							{#each WorkCodesIndexer as workOption}
								<option>{workOption}</option>
							{/each}
						</select>
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
								class="bg-green-200 w-12"
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
					{#each $projects as project}
						<option>{project.jobNumber} - {project.jobDescription}</option>
					{/each}
				</datalist></td
			>
			<td class="border border-slate-800 text-right pr-5" colspan="9">Total Hours: {totalHours}</td>
		</tr>
	</table>
	<button
		class="border hover:border-black p-1 bg-slate-200 hover:bg-transparent rounded m-2"
		disabled={$timelogStore.filter((row) => row.key.workCode == WorkCode.Unset).length > 0}
		on:click={async () => {
			await timelogStore.submitLogs($authStore);
			timelogStore.getNewLogs($authStore);
			notificationStore.addNew('Entries saved', 1000);
		}}>Save Entries</button
	>
	<button
		class="border hover:border-black p-1 bg-slate-200 hover:bg-transparent rounded m-2"
		disabled={$timelogStore.filter((row) => row.key.workCode == WorkCode.Unset).length > 0}
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
