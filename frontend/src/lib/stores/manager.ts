
import { approveTimelogs, getTimeLogs } from "$lib/api/timelog";
import { getManagersEmployees } from "$lib/api/users";
import { convertTimeEntries, TimeCardRow } from "$lib/model/timelogs";
import type { Manager, Token } from "$lib/model/users";
import { writable } from "svelte/store";

const { subscribe, set, update } = writable<Manager>({ currentTimecard: [], employees: [] });

export const managerStore = {
    subscribe,
    set,
    update,
    fetchEmployees: async (auth: Token) => {
        const employees = await getManagersEmployees(auth);
        set({ currentTimecard: [], employees, });
    },
    fetchTimecard: async (user: number, auth: Token) => {
        const timelogs = await getTimeLogs(user, auth);
        update(data => { data.currentTimecard = convertTimeEntries(timelogs); return data; });
    },
    approveHours: async (auth: Token, logs: TimeCardRow[]) => {
        await approveTimelogs(logs, auth);
        const employees = await getManagersEmployees(auth);
        set({ currentTimecard: [], employees, });
    }
}