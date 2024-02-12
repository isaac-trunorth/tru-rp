import type { Project } from "$lib/model/general";
import type { TimeCardKey } from "$lib/model/timelogs";
import type { User } from "$lib/model/users";

export const days = ['Mon', 'Tues', 'Wed', 'Thurs', 'Fri', 'Sat', 'Sun'];
export const month = [
    'Jan',
    'Feb',
    'Mar',
    'Apr',
    'May',
    'Jun',
    'Jul',
    'Aug',
    'Sep',
    'Oct',
    'Nov',
    'Dec'
];

export function getDateAfterMonday(monday: Date, daysSinceMonday: number): Date {
    const timediff = daysSinceMonday * 24 * 60 * 60 * 1000;
    return new Date(monday.getTime() + timediff);
}

export function getDate(monday: Date, daysSinceMonday: number): string {
    const diff = getDateAfterMonday(monday, daysSinceMonday);
    let day = diff.getDay() - 1;
    if (day < 0) day = 6;
    return `${diff.getDate()}-${month[diff.getMonth()]}\n${days[day]}`;
}

export function formatDate(date: Date): string {
    return `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate().toString().padStart(2, '0')}`
}

export function storeKey(key: TimeCardKey): string {
    const { date, projectId, workCode } = key;
    return `${formatDate(date)}_${projectId}_${workCode}`
}

export function getMonday(dateString: string): Date {
    const date = new Date(dateString);
    let daysSinceMonday = date.getDay();
    if (daysSinceMonday < 0) daysSinceMonday = 6;
    const timediff = (daysSinceMonday - 1) * 24 * 60 * 60 * 1000;
    const diff = new Date(date.getTime() - timediff);
    return diff;
}
export function getMondayFromDate(date: Date): Date {
    let daysSinceMonday = date.getDay();
    if (daysSinceMonday < 0) daysSinceMonday = 6;
    const timediff = (daysSinceMonday - 1) * 24 * 60 * 60 * 1000;
    const diff = new Date(date.getTime() - timediff);
    return diff;
}

export function getProjectId(projects: Project[], id: number): number {
    let foundId = 0;
    projects.forEach(row => {
        if (row.id == id) foundId = row.jobNumber;
    });
    return foundId;
}

export function getProjectById(projects: Project[], id: number): Project {
    let foundProject: Project = {
        id: 0,
        jobNumber: 0,
        jobDescription: "Not Found",
        jobActive: false
    };
    projects.forEach(row => {
        if (row.id == id) foundProject = row;
    });
    return foundProject;
}

export function getUserName(users: User[], id: number): string {
    let found = '';
    users.forEach(user => {
        if (user.id == id) found = `${user.firstName} ${user.lastName}`;
    });
    return found;
}