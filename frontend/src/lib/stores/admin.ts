
import { createUser, getUsers, updateUser } from "$lib/api/users";
import type { Token, User } from "$lib/model/users";
import { writable } from "svelte/store";

const { subscribe, set, update } = writable<User[]>([]);

export const usersStore = {
    subscribe,
    set,
    update,
    getAll: async (auth: Token) => {
        const users = await getUsers(auth);
        set(users);
    },
    createNew: async (auth: Token, user: User) => {
        await createUser(user, auth);
        const users = await getUsers(auth);
        set(users);
    },
    updateUser: async (auth: Token, user: User) => {
        await updateUser(user, auth);
        const users = await getUsers(auth);
        set(users);
    }
}