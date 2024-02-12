<script lang="ts">
	import { usersStore } from '$lib/stores/admin';
	import {
		FilterOptions,
		WorkCode,
		type TimeEntry,
		type TimelogRequest
	} from '$lib/model/timelogs';
	import { authStore } from '$lib/stores/user';
	import { metricsStore } from '$lib/stores/metrics';
	import { projects } from '$lib/stores/projects';
	import SectionView from './SectionView.svelte';
	import { notificationStore } from '$lib/stores/notifications';
	import { onMount } from 'svelte';
	import { getProjectById, getProjectId, getUserName } from '$lib/utilities/utilities';
	import { WorkCodesIndexer, type Project } from '$lib/model/general';

	const filters: TimelogRequest = {};
	let dateRangeType = 'All';
	const now = new Date();
	const today: string = `${now.getFullYear()}-${(now.getMonth() + 1)
		.toString()
		.padStart(2, '0')}-${now.getDate().toString().padStart(2, '0')}`;
	let weekEndDate: string = today;
	let rangeStartDate: string = today;
	let rangeEndDate: string = today;
	const rangeOptions = ['All', 'Single week', 'Date Range'];
	const filterOptions = { project: false, user: false, workCode: false };

	const href = formatData($metricsStore);

	function formatData(formatData: TimeEntry[]): string {
		const output: string[] = ['Project Number,Employee,Date of work,Hours,Submit Status,Work Code'];
		$metricsStore.forEach(
			({ jobId, employeeId, dateOfWork, hoursWorked, submitStatus, workCode }) => {
				output.push(
					`${getProjectId($projects, jobId)},${getUserName(
						$usersStore,
						employeeId
					)},${dateOfWork},${hoursWorked},${submitStatus},${workCode}`
				);
			}
		);
		return encodeURI('data:text/csv;charset=utf-8,' + output.join('\n'));
	}

	onMount(() => {
		usersStore.getAll($authStore);
		projects.getProjects($authStore);
		metricsStore.getLogs(filters, $authStore);
	});

	function updateFilters() {
		if (dateRangeType == 'All') {
			filters.weekEndDate = undefined;
			filters.startDate = undefined;
			filters.endDate = undefined;
		} else if (dateRangeType == 'Single week') {
			filters.weekEndDate = weekEndDate;
			filters.startDate = undefined;
			filters.endDate = undefined;
			if (filters.weekEndDate == undefined) return;
		} else if (dateRangeType == 'Date Range') {
			filters.endDate = rangeEndDate;
			filters.startDate = rangeStartDate;
			filters.weekEndDate = undefined;
			if (filters.endDate == undefined || filters.startDate == undefined) return;
		}
		if (!filterOptions.project) filters.projectId = undefined;
		if (!filterOptions.user) filters.userId = undefined;
		if (!filterOptions.workCode) filters.workCode = undefined;
		metricsStore.getLogs(filters, $authStore);
		notificationStore.addNew('New logs fetched');
	}
	function displayProject(projects: Project[], id: number): string {
		const proejct = getProjectById(projects, id);
		return `${proejct.jobNumber} ${proejct.jobDescription}`;
	}
</script>

<div>
	<h1 class="font-bold">Metrics</h1>
	<div class="grid grid-cols-2">
		<div class="m-2">
			<button
				class="border hover:border-black p-1 bg-slate-200 hover:bg-transparent rounded m-2"
				on:click={updateFilters}>Fetch New Metrics</button
			>
			<div>
				<SectionView header="Filter Criteria">
					<div>
						<input
							type="checkbox"
							bind:checked={filterOptions.project}
							on:change={() => {
								if (!filterOptions.project) filters.projectId = 0;
							}}
						/>
						Project:
						<select
							class="border rounded"
							bind:value={filters.projectId}
							on:change={() => (filterOptions.project = filters.projectId != 0)}
						>
							<option></option>
							{#each $projects as project}
								<option value={project.id}
									>{`${project.jobNumber} ${project.jobDescription}`}</option
								>
							{/each}
						</select>
					</div>
					<div>
						<input
							type="checkbox"
							bind:checked={filterOptions.user}
							on:change={() => {
								if (!filterOptions.user) filters.userId = 0;
							}}
						/>
						User:
						<select
							class="border rounded"
							bind:value={filters.userId}
							on:change={() => (filterOptions.user = filters.userId != 0)}
						>
							<option></option>
							{#each $usersStore as user}
								<option value={user.id}>{`${user.firstName} ${user.lastName}`}</option>
							{/each}
						</select>
					</div>
					<div>
						<input
							type="checkbox"
							bind:checked={filterOptions.workCode}
							on:change={() => {
								if (!filterOptions.workCode) filters.workCode = undefined;
							}}
						/>
						Work Code:
						<select
							class="border rounded"
							bind:value={filters.workCode}
							on:change={() => (filterOptions.workCode = filters.workCode != '')}
						>
							<option></option>
							{#each WorkCodesIndexer as workOption}
								<option>{workOption}</option>
							{/each}
						</select>
					</div>
				</SectionView>
			</div>
			<div>
				<div>
					<SectionView header="Date Filter ({dateRangeType})">
						<h1>Date Filter:</h1>
						<select bind:value={dateRangeType}>
							{#each rangeOptions as option}
								<option>{option}</option>
							{/each}
						</select>
					</SectionView>
				</div>
				<div>
					{#if dateRangeType == rangeOptions[1]}
						<input type="date" bind:value={weekEndDate} />
					{/if}
					{#if dateRangeType == rangeOptions[2]}
						Start:
						<input type="date" bind:value={rangeStartDate} />
						End:
						<input type="date" bind:value={rangeEndDate} />
					{/if}
				</div>
			</div>
		</div>
		<div>
			<h1 class="font-bold underline">
				Search Results - {$metricsStore.reduce((a, b) => a + b.hoursWorked, 0)} Total Hours
			</h1>
			<table>
				<tr>
					<td class="text-center border m-2 p-0.5 font-bold">Employee</td>
					<td class="text-center border m-2 p-0.5 font-bold">Hours Worked</td>
					<td class="text-center border m-2 p-0.5 font-bold">Work Code</td>
					<td class="text-center border m-2 p-0.5 font-bold">Submit status</td>
					<td class="text-center border m-2 p-0.5 font-bold">Project</td>
					<td class="text-center border m-2 p-0.5 font-bold">Date</td>
				</tr>
				{#each $metricsStore as metric}
					<tr>
						<td class="text-center border m-1 p-0.5"
							>{getUserName($usersStore, metric.employeeId)}</td
						>
						<td class="text-center border m-1 p-0.5">{metric.hoursWorked}</td>
						<td class="text-center border m-1 p-0.5">{metric.workCode}</td>
						<td class="text-center border m-1 p-0.5">{metric.submitStatus}</td>
						<td class="text-center border m-1 p-0.5">{displayProject($projects, metric.jobId)}</td>
						<td class="text-center border m-1 p-0.5">{metric.dateOfWork}</td>
					</tr>
				{/each}
			</table>
			<button class="border hover:border-black p-1 bg-slate-200 hover:bg-transparent rounded m-2"
				><a {href}>Download as csv</a></button
			>
		</div>
	</div>
</div>
