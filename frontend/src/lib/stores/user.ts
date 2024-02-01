import { createUser, login } from "$lib/api/users";
import type { Token, User } from "$lib/model/users";
import { writable } from "svelte/store";

const blankUser: Token = {
    userId: 0,
    accessLevel: 0,
    exp: 0,
    tokenText: '',
}
const { subscribe, set, update } = writable<Token>(blankUser);

export const userStore = {
    subscribe,
    update,
    set,
    login: async (user: User) => {
        const token = await login(user);
        set(token);
    },
    logout: () => {
        set(blankUser);
    },
    createNewUser: async (user: User, auth: Token) => {
        const newUser = await createUser(user, auth);
        return `New user created: ID# ${newUser.id} - ${newUser.name}`;
    }
}
