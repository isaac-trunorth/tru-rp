<script lang="ts">
	import { authStore } from '$lib/stores/user';
	import Modal from '$lib/components/modal.svelte';
	import type { User } from '$lib/model/users';
	import TopBanner from '$lib/components/TopBanner.svelte';

	let user: User = {
		id: 0,
		managerId: 0,
		firstName: '',
		lastName: '',
		userName: '',
		password: '',
		accessLevel: 0
	};
</script>

<div>
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
		<TopBanner />
	{/if}
</div>
