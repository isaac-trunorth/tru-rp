import { getDateAfterMonday, getMonday, getMondayFromDate } from "$lib/utilities/utilities";
import pkg from 'lodash';
const { groupBy } = pkg;

export enum Status {
    Approved = "Approved",
    Initial = "Initial",
    Submitted = "Submitted",
}

interface HoursWorked {
    hoursWorked: number,
    submitStatus: Status,
}

export interface TimeEntry extends HoursWorked {
    id: number,
    jobNumber: number,
    jobDescription: string,
    workCode: number,
    employeeId: number,
    dateOfWork: string,
}

type Optional<Type> = {
    [Property in keyof Type]+?: Type[Property];
};

interface ConcreteRequest {
    userId: number,
    managerId: number,
    weekEndDate: Date,
    status: Status,
}

export type TimelogRequest = Optional<ConcreteRequest>;

export interface IdList {
    ids: Array<number>,
}

export enum UiStatus {
    New,
    Unchanged,
    Changed,
    Deleted
}

export class EntryField {
    status: UiStatus;
    date: Date;
    id: number;
    info: HoursWorked;
    constructor(id: number, status: UiStatus, date: Date, info: HoursWorked) {
        this.id = id;
        this.status = status;
        this.date = date;
        this.info = info;
    }
    UpdateStatus() {
        if (this.status == UiStatus.Unchanged || this.status == UiStatus.Deleted) {
            if (this.info.hoursWorked > 0) this.status = UiStatus.Changed;
            else this.status = UiStatus.Deleted;
        }
        else if (this.status == UiStatus.Changed) {
            if (this.info.hoursWorked == 0) this.status = UiStatus.Deleted;
        }
    }
}

export type TimeCardKey = { date: Date, projectNumber: number, workCode: number };
export type TimeCardStore = TimeCardRow[];
export class TimeCardRow {
    key: TimeCardKey;
    name: string;
    entries: EntryField[];
    constructor(name: string, key: TimeCardKey) {
        this.key = key;
        this.name = name;
        this.entries = [];
        const monday = getMondayFromDate(key.date);
        for (let i = 0; i < 7; i++) {
            const newEntry = new EntryField(
                0,
                UiStatus.New,
                getDateAfterMonday(monday, i),
                {
                    submitStatus: Status.Initial,
                    hoursWorked: 0,
                }
            );
            this.entries.push(newEntry);
        }
    }

    SetHours(id: number, hours: number, status: Status, date: Date) {
        this.entries.forEach(entry => {
            if (entry.date.toDateString() == date.toDateString()) {
                entry.id = id;
                entry.status = UiStatus.Unchanged;
                entry.info.hoursWorked = hours;
                entry.info.submitStatus = status;
            }
        })
    }
}
export function convertTimeEntries(entries: TimeEntry[]): TimeCardStore {
    const rows: TimeCardStore = [];
    const grouped = groupBy(entries, (val) => getMonday(val.dateOfWork).toDateString());
    Object.keys(grouped).forEach(key => {
        const week = new Date(key);
        const groupedCodes = groupBy(grouped[key], (val) => val.workCode);
        Object.keys(groupedCodes).forEach(codeKey => {
            const newRow: TimeCardRow = new TimeCardRow(
                groupedCodes[codeKey][0].jobDescription,
                {
                    projectNumber: groupedCodes[codeKey][0].jobNumber,
                    workCode: groupedCodes[codeKey][0].workCode,
                    date: week,
                }
            );
            // array is codes in a given week for a given project
            groupedCodes[codeKey].forEach((entry) => {
                let date = new Date(entry.dateOfWork);
                const userTimezoneOffset = date.getTimezoneOffset() * 60000;
                date = new Date(date.getTime() + userTimezoneOffset);
                newRow.SetHours(entry.id, entry.hoursWorked, entry.submitStatus, date);
            })
            rows.push(newRow);
        })
    });
    return rows;
}