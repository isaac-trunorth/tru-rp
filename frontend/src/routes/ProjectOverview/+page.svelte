<script lang="ts">
	import TopBanner from '$lib/components/TopBanner.svelte';
	import { type Project } from '$lib/model/general';
	import { projects } from '$lib/stores/projects';
	import { authStore } from '$lib/stores/user';
	const newProject: Project = {
		id: 0,
		jobNumber: 999000,
		jobDescription: '',
		jobActive: true
	};
	async function createNewProject() {
		await projects.createNew(newProject, $authStore);
	}
</script>

<div>
	<TopBanner />
	<div class="pl-5">
		<h1 class="font-bold">Project Overview:</h1>
		<ul class="list-disc m-5 p-1 list-inside">
			Projects:
			{#each $projects.sort((a, b) => a.jobNumber - b.jobNumber) as project}
				<li>
					{project.jobNumber} - {project.jobDescription}
				</li>
			{/each}
		</ul>
		<div>
			<h1 class="font-bold">Create New Project:</h1>
			<input
				class="border border-black rounded p-0.5"
				type="number"
				bind:value={newProject.jobNumber}
			/>
			Project Number
			<br />
			<input class="border border-black rounded p-0.5" bind:value={newProject.jobDescription} />
			Project Name
			<br />
			<button
				class="border border-black rounded bg-slate-200 hover:bg-transparent p-1"
				on:click={createNewProject}>Create New Project</button
			>
		</div>
	</div>
</div>
