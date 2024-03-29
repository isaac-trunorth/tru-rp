import { Status, UiStatus, TimeCardRow, type TimeCardStore, type TimeEntry, type IdList, type TimelogRequest } from '$lib/model/timelogs';
import type { Token } from '$lib/model/users';
import { formatDate } from '$lib/utilities/utilities';
export const url = "http://localhost:4000";
const timelogs = 'timelogs';

export async function getTimeLogs(request: TimelogRequest, auth: Token): Promise<TimeEntry[]> {
    const requestParams = new URLSearchParams();
    Object.keys(request).forEach(key => {
        let typedKey = key as unknown as keyof TimelogRequest;
        if (!!request[typedKey]) requestParams.append(key, request[typedKey]!.toString());
    });
    const res = await fetch(url + "/" + timelogs + '?' + requestParams.toString(), {
        method: 'GET',
        headers: [['Authorization', auth.tokenText]],
    })
    const asJson: TimeEntry[] = await res.json();
    return asJson;
}

function dateToString(date: Date) {
    return `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate()}`;
}

export async function submitTimelogs(logs: TimeCardStore, user: Token) {
    const newLogs: TimeEntry[] = [];
    const updateLogs: TimeEntry[] = [];
    const deleteLogs: IdList = { ids: [] };
    logs.forEach(log => {
        log.entries.forEach(hour => {
            if (hour.status == UiStatus.New && hour.info.hoursWorked > 0) {
                newLogs.push({
                    id: 0,
                    jobId: log.key.projectId,
                    workCode: log.key.workCode,
                    employeeId: user.userId,
                    dateOfWork: dateToString(hour.date),
                    hoursWorked: hour.info.hoursWorked,
                    submitStatus: hour.info.submitStatus,
                });
            }
            else if (hour.status == UiStatus.Changed && hour.info.hoursWorked > 0) {
                updateLogs.push({
                    id: hour.id,
                    jobId: log.key.projectId,
                    workCode: log.key.workCode,
                    employeeId: user.userId,
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
            headers: [['Authorization', user.tokenText], ['content-type', 'application/json']],
            body,
        });
    }
    // update:
    if (updateLogs.length > 0) {
        const updatedLogs = JSON.stringify(updateLogs);
        await fetch(url + "/" + timelogs, {
            method: 'PUT',
            headers: [['Authorization', user.tokenText], ['content-type', 'application/json']],
            body: updatedLogs,
        });
    }
    // delete:
    if (deleteLogs.ids.length > 0) {
        const deleted = JSON.stringify(deleteLogs);
        await fetch(url + "/" + timelogs, {
            method: 'DELETE',
            headers: [['Authorization', user.tokenText], ['content-type', 'application/json']],
            body: deleted,
        });
    }
}

export async function approveTimelogs(logs: TimeCardRow[], auth: Token) {
    const approved: IdList = { ids: [] };
    const changedTls: TimeEntry[] = [];
    logs.forEach((row) => {
        row.entries.forEach(entry => {
            if (entry.info.submitStatus == Status.Approved) approved.ids.push(entry.id)
            else if (entry.status == UiStatus.Changed) changedTls.push({
                id: entry.id,
                jobId: row.key.projectId,
                workCode: row.key.workCode,
                employeeId: row.key.userId,
                dateOfWork: formatDate(entry.date),
                hoursWorked: entry.info.hoursWorked,
                submitStatus: entry.info.submitStatus,
            });
        })
    })
    const body = JSON.stringify(approved);
    const res = await fetch(url + "/" + timelogs + "/approve", {
        method: 'PUT',
        headers: [['Authorization', auth.tokenText], ['content-type', 'application/json']],
        body,
    })
    // update:
    if (changedTls.length > 0) {
        const updatedLogs = JSON.stringify(changedTls);
        await fetch(url + "/" + timelogs, {
            method: 'PUT',
            headers: [['Authorization', auth.tokenText], ['content-type', 'application/json']],
            body: updatedLogs,
        });
    }
}