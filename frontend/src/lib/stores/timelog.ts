import { writable } from "svelte/store";
import { submitTimelogs, getTimeLogs } from "../api/timelog";
import { TimeCardRow, type TimeEntry, type TimeCardStore, convertTimeEntries, Status, } from "$lib/model/timelogs";
import type { Token } from "$lib/model/users";

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
    getNewLogs: async (auth: Token, user?: number) => {
        if (!user) user = auth.userId;
        const newLogs = await getTimeLogs(user, auth);
        set(convertTimeEntries(newLogs));
    },
    submitLogs: async (auth: Token) => {
        let data: TimeCardStore;
        update(dataUser => {
            data = dataUser;
            return dataUser;
        });
        await submitTimelogs(data!, auth);
    },
    addLog: (newLog: TimeCardRow) => {
        update(data => {
            data.push(newLog);
            return data;
        });
    },
    removeLog: async (log: TimeCardRow, auth: Token) => {
        log.entries.forEach((entry) => {
            if (entry.info.submitStatus != Status.Approved) {
                entry.info.hoursWorked = 0;
                entry.UpdateStatus();
            }
        });
        submitTimelogs([log], auth);
        const newLogs = await getTimeLogs(auth.userId, auth);
        set(convertTimeEntries(newLogs));
    }
}
