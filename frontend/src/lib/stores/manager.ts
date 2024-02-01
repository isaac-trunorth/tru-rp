
import { approveTimelogs, getTimeLogs } from "$lib/api/timelog";
import { getEmployees } from "$lib/api/users";
import { convertTimeEntries, TimeCardRow } from "$lib/model/timelogs";
import type { Manager, Token } from "$lib/model/users";
import { writable } from "svelte/store";

const { subscribe, set, update } = writable<Manager>({ currentTimecard: [], employees: [] });

export const managerStore = {
    subscribe,
    set,
    update,
    fetchEmployees: async (user: number) => {
        const employees = getEmployees(user);
        set({ currentTimecard: [], employees, });
    },
    fetchTimecard: async (user: number, auth: Token) => {
        const timelogs = await getTimeLogs(user, auth);
        update(data => { data.currentTimecard = convertTimeEntries(timelogs); return data; });
    },
    approveHours: async (user: number, logs: TimeCardRow[], auth: Token) => {
        await approveTimelogs(user, logs, auth);
        const employees = getEmployees(user);
        set({ currentTimecard: [], employees, });
    }

}