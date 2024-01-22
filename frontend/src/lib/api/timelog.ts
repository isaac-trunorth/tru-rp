import { Status, UiStatus, TimeCardRow, type TimeCardStore, type TimeEntry, type IdList } from '$lib/model/timelogs';
import { formatDate } from '$lib/utilities/utilities';
const url = "http://localhost:4000";
const timelogs = 'timelogs';

export async function getTimeLogs(user: number): Promise<TimeEntry[]> {
    const res = await fetch(url + "/" + timelogs + "/" + user.toString(), {
        method: 'GET',
        headers: [['Authorization', 'test']],
    })
    const asJson: TimeEntry[] = await res.json();
    return asJson;
}

function dateToString(date: Date) {
    return `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate()}`;
}

export async function submitTimelogs(logs: TimeCardStore, user: number) {
    const newLogs: TimeEntry[] = [];
    const updateLogs: TimeEntry[] = [];
    const deleteLogs: IdList = { ids: [] };
    logs.forEach(log => {
        log.entries.forEach(hour => {
            if (hour.status == UiStatus.New && hour.info.hoursWorked > 0) {
                newLogs.push({
                    id: 0,
                    jobNumber: log.key.projectNumber,
                    jobDescription: log.name,
                    workCode: parseInt(log.key.workCode.toString()),
                    employeeId: user,
                    dateOfWork: dateToString(hour.date),
                    hoursWorked: hour.info.hoursWorked,
                    submitStatus: hour.info.submitStatus,
                });
            }
            else if (hour.status == UiStatus.Changed && hour.info.hoursWorked > 0) {
                updateLogs.push({
                    id: hour.id,
                    jobNumber: log.key.projectNumber,
                    jobDescription: log.name,
                    workCode: parseInt(log.key.workCode.toString()),
                    employeeId: user,
                    dateOfWork: dateToString(hour.date),
                    hoursWorked: hour.info.hoursWorked,
                    submitStatus: hour.info.submitStatus,
                });
            }
            else if (hour.status == UiStatus.Deleted) {
                deleteLogs.ids.push(hour.id);
            }
        })
    })
    // new:
    if (newLogs.length > 0) {
        const body = JSON.stringify(newLogs);
        await fetch(url + "/" + timelogs, {
            method: 'POST',
            headers: [['Authorization', 'test'], ['content-type', 'application/json']],
            body,
        });
    }
    // update:
    if (updateLogs.length > 0) {
        const updatedLogs = JSON.stringify(updateLogs);
        await fetch(url + "/" + timelogs, {
            method: 'PUT',
            headers: [['Authorization', 'test'], ['content-type', 'application/json']],
            body: updatedLogs,
        });
    }
    // delete:
    if (deleteLogs.ids.length > 0) {
        console.log("deleted:");
        console.log(deleteLogs);
        const deleted = JSON.stringify(deleteLogs);
        await fetch(url + "/" + timelogs, {
            method: 'DELETE',
            headers: [['Authorization', 'test'], ['content-type', 'application/json']],
            body: deleted,
        });
    }
}

export async function approveTimelogs(user: number, logs: TimeCardRow[]) {
    const approved: IdList = { ids: [] };
    const changedTls: TimeEntry[] = [];
    logs.forEach((row) => {
        row.entries.forEach(entry => {
            if (entry.info.submitStatus == Status.Approved) approved.ids.push(entry.id)
            else if (entry.status == UiStatus.Changed) changedTls.push({
                id: entry.id,
                jobNumber: row.key.projectNumber,
                jobDescription: row.name,
                workCode: parseInt(row.key.workCode.toString()),
                employeeId: user,
                dateOfWork: formatDate(entry.date),
                hoursWorked: entry.info.hoursWorked,
                submitStatus: entry.info.submitStatus,
            });
        })
    })
    const body = JSON.stringify(approved);
    const res = await fetch(url + "/" + timelogs + "/approve", {
        method: 'PUT',
        headers: [['Authorization', 'test'], ['content-type', 'application/json']],
        body,
    })
    // update:
    if (changedTls.length > 0) {
        const updatedLogs = JSON.stringify(changedTls);
        await fetch(url + "/" + timelogs, {
            method: 'PUT',
            headers: [['Authorization', 'test'], ['content-type', 'application/json']],
            body: updatedLogs,
        });
    }
}