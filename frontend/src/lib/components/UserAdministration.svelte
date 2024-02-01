<script lang="ts">
	import type { User } from '$lib/model/users';
	import { userStore } from '$lib/stores/user';
	let newUser: User = {
		id: 0,
		name: '',
		password: '',
		accessLevel: 1,
		managerId: 1
	};
</script>

<div>
	<h1 class="font-bold">User Administration</h1>
	TODO
	<input bind:value={newUser.name} />
</div>
