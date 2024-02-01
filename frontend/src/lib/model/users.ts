import type { TimeCardRow } from "$lib/model/timelogs";

export interface User {
    id: number,
    managerId: number,
    name: string,
    password: string,
    accessLevel: number,
}

export interface Manager {
    employees: User[],
    currentTimecard: TimeCardRow[],
}

export interface Token {
    userId: number,
    accessLevel: number,
    exp: number,
    tokenText: string,
}