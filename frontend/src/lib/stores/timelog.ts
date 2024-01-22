import { writable } from "svelte/store";
import { submitTimelogs, getTimeLogs } from "../api/timelog";
import { TimeCardRow, type TimeEntry, type TimeCardStore, convertTimeEntries, } from "$lib/model/timelogs";

const { subscribe, set, update } = writable<TimeCardStore>([]);


function countTotalHours(entries: TimeCardStore): number {
    return 1;
}

export const timelogStore = {
    subscribe,
    set,
    update,
    totalHours: () => {
        let data: TimeCardStore;
        update(dataUser => {
            data = dataUser;
            return dataUser;
        });
        return countTotalHours(data!);
    },
    getNewLogs: async (user: number) => {
        const newLogs = await getTimeLogs(user);
        set(convertTimeEntries(newLogs));
    },
    submitLogs: (user: number) => {
        let data: TimeCardStore;
        update(dataUser => {
            data = dataUser;
            return dataUser;
        });
        submitTimelogs(data!, user);
    },
    addLog: (newLog: TimeCardRow) => {
        update(data => {
            data.push(newLog);
            return data;
        });
    },
    removeLog: async (log: TimeCardRow, user: number) => {
        log.entries.forEach((entry) => {
            entry.info.hoursWorked = 0;
            entry.UpdateStatus();
        });
        submitTimelogs([log], user);
        const newLogs = await getTimeLogs(user);
        set(convertTimeEntries(newLogs));
    }
}
