import { writable } from "svelte/store";

const { subscribe, set, update } = writable<string[]>([]);

export const notificationStore = {
    subscribe,
    set,
    update,
    addNew: (msg: string, ms = 1000) => {
        update(data => {
            data.push(msg);
            return data;
        });
        setTimeout(() => {
            update(data => {
                data = data.filter(row => row != msg);
                return data;
            })
        }, ms);
    }
}

