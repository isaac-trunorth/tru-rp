import type { TimeCardRow } from "$lib/model/timelogs";

export interface User {
    id: number,
    managerId?: number,
    name: string,
    password: string,
}

export interface Manager {
    employees: User[],
    currentTimecard: TimeCardRow[],
}