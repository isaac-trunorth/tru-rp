import { approveTimelogs, getTimeLogs } from "$lib/api/timelog";
import { getEmployees } from "$lib/api/users";
import { convertTimeEntries, TimeCardRow, type TimeCardStore } from "$lib/model/timelogs";
import type { Manager } from "$lib/model/users";
import { writable } from "svelte/store";

export const userStore = writable(1);

const { subscribe, set, update } = writable<Manager>({ currentTimecard: [], employees: [] });

export const managerStore = {
    subscribe,
    set,
    update,
    fetchEmployees: async (user: number) => {
        const employees = getEmployees(user);
        set({ currentTimecard: [], employees, });
    },
    fetchTimecard: async (user: number) => {
        const timelogs = await getTimeLogs(user);
        update(data => { data.currentTimecard = convertTimeEntries(timelogs); return data; });
    },
    approveHours: async (user: number, logs: TimeCardRow[]) => {
        await approveTimelogs(user, logs);
        const employees = getEmployees(user);
        set({ currentTimecard: [], employees, });
    }

}