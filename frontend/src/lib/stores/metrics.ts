
import { writable } from "svelte/store";
import { getTimeLogs } from "../api/timelog";
import { type TimeEntry, type TimelogRequest, } from "$lib/model/timelogs";
import type { Token } from "$lib/model/users";

const { subscribe, set, update } = writable<TimeEntry[]>([]);

export const metricsStore = {
    subscribe,
    set,
    update,
    getLogs: async (filters: TimelogRequest, auth: Token) => {
        const newLogs = await getTimeLogs(filters, auth);
        set(newLogs);
    }
}