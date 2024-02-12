<script lang="ts">
	import type { User } from '$lib/model/users';
	import { authStore } from '$lib/stores/user';
	import { usersStore } from '$lib/stores/admin';
	import { onMount } from 'svelte';
	import { notificationStore } from '$lib/stores/notifications';
	import SectionView from './SectionView.svelte';

	const emptyUser: User = {
		id: 0,
		firstName: '',
		lastName: '',
		userName: '',
		password: '',
		accessLevel: 1,
		managerId: 1
	};
	let newUser = Object.assign({}, emptyUser);

	let viewedUser = Object.assign({}, emptyUser);
	let viewUsername: string;

	onMount(() => {
		usersStore.getAll($authStore);
	});

	async function submitNew() {
		if (newUser.userName == '') return;
		try {
			await usersStore.createNew($authStore, newUser);
			notificationStore.addNew('New user created');
		} catch {
			notificationStore.addNew('New user creation failed. Verify no name conflicts', 2000);
		}
		newUser = Object.assign({}, emptyUser);
	}

	async function updateUser() {
		if (viewedUser == null) return;
		await usersStore.updateUser($authStore, viewedUser);
		notificationStore.addNew('User updated');
	}
	function updateViewUser() {
		const crntUser = $usersStore.filter((row) => row.userName == viewUsername);
		if (crntUser.length > 0) {
			viewedUser.id = crntUser[0].id;
			viewedUser.firstName = crntUser[0].firstName;
			viewedUser.lastName = crntUser[0].lastName;
			viewedUser.userName = crntUser[0].userName;
			viewedUser.accessLevel = crntUser[0].accessLevel;
			viewedUser.managerId = crntUser[0].managerId;
		} else {
			viewedUser = Object.assign({}, emptyUser);
		}
	}
</script>

<div>
	<h1 class="font-bold underline text-lg">User Administration</h1>
	<SectionView header="New User">
		<table class="text-right p-1 m-1">
			<tr>
				<td> Username: </td>
				<td>
					<input class="border rounded black" bind:value={newUser.userName} />
				</td>
			</tr>
			<tr>
				<td> First name: </td>
				<td>
					<input class="border rounded black" bind:value={newUser.firstName} />
				</td>
			</tr>
			<tr>
				<td> Last name: </td>
				<td>
					<input class="border rounded black" bind:value={newUser.lastName} />
				</td>
			</tr>
			<tr>
				<td> Manager: </td>
				<td>
					<select bind:value={newUser.managerId}>
						{#each $usersStore.filter((row) => row.accessLevel > 1) as manager}
							<option value={manager.id}>{manager.userName}</option>
						{/each}
					</select>
				</td>
			</tr>
			<tr>
				<td> Access Level: </td>
				<td>
					<input class="border rounded black" type="number" bind:value={newUser.accessLevel} />
				</td>
			</tr>
			<tr>
				<td> Password: </td>
				<td>
					<input class="border rounded black" bind:value={newUser.password} />
				</td>
			</tr>
			<tr>
				<td> </td>
				<td>
					<button
						class="rounded border bg-slate-100 hover:bg-slate-200 border-black p-0.5"
						on:click={submitNew}>Submit New User</button
					>
				</td>
			</tr>
		</table>
	</SectionView>
	<SectionView header="Current Users">
		<input
			class="border p-1 m-1"
			list="users"
			bind:value={viewUsername}
			on:focusout={updateViewUser}
			on:keydown={(key) => {
				if (key.key == 'Enter') {
					updateViewUser();
				}
			}}
			on:click={() => (viewUsername = '')}
		/>
		<datalist id="users">
			{#each $usersStore as user}
				<option>{user.userName}</option>
			{/each}
		</datalist>
		{#if viewedUser != null}
			<table class="text-right p-1 m-1">
				<tr>
					<td> Username: </td>
					<td>
						<input class="border rounded black" bind:value={viewedUser.userName} />
					</td>
				</tr>
				<tr>
					<td> First name: </td>
					<td>
						<input class="border rounded black" bind:value={viewedUser.firstName} />
					</td>
				</tr>
				<tr>
					<td> Last name: </td>
					<td>
						<input class="border rounded black" bind:value={viewedUser.lastName} />
					</td>
				</tr>
				<tr>
					<td> Manager: </td>
					<td>
						<select bind:value={viewedUser.managerId}>
							{#each $usersStore.filter((row) => row.accessLevel > 1) as manager}
								<option value={manager.id}>{manager.userName}</option>
							{/each}
						</select>
					</td>
				</tr>
				<tr>
					<td> Access Level: </td>
					<td>
						<input class="border rounded black" type="number" bind:value={viewedUser.accessLevel} />
					</td>
				</tr>
			</table>
			<button
				class="rounded border bg-slate-100 hover:bg-slate-200 border-black p-0.5"
				on:click={updateUser}>Update User</button
			>
		{/if}
	</SectionView>
</div>
