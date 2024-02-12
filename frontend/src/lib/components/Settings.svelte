<script lang="ts">
	import type { User } from '$lib/model/users';
	import { notificationStore } from '$lib/stores/notifications';
	import { authStore } from '$lib/stores/user';
	import SectionView from './SectionView.svelte';
	let newPwd = '';
	let confirmPwd = '';
	$: match = newPwd == confirmPwd && newPwd != '';
	async function updateUserPassword() {
		const newUserPwd: User = {
			id: $authStore.userId,
			managerId: 1,
			firstName: 'na',
			lastName: 'na',
			userName: 'na',
			password: newPwd,
			accessLevel: 0
		};
		const res = await authStore.updatePassword(newUserPwd, $authStore);
		notificationStore.addNew(res);
		$authStore.accessLevel = 0;
	}
</script>

<div>
	<SectionView header="Update Password">
		<div class="grid grid-cols-2">
			<div class="text-right">New Password:</div>
			<div>
				<input class="border rounded black" type="password" bind:value={newPwd} />
			</div>
			<div class="text-right">Confirm Password:</div>
			<div>
				<input class="border rounded black" type="password" bind:value={confirmPwd} />
			</div>
			{#if match}
				<div class="col-span-2">
					<button
						class="rounded border bg-slate-100 hover:bg-slate-200 border-black p-0.5"
						on:click={updateUserPassword}>Update Password</button
					>
				</div>
			{:else}
				<div class="bg-red-200 col-span-2">Passwords don't match</div>
			{/if}
		</div>
	</SectionView>
</div>
