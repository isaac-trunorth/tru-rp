import type { Manager, User } from "$lib/model/users";

const exampleEmployees = [
    'Employee0',
    'Isaac',
    'John Doe',
    'Jane Doe',
]
export function getEmployees(managerId: number): User[] {
    const users: User[] = [];
    for (let i = 0; i < 3; i++) {
        users.push({
            id: i,
            name: exampleEmployees[i],
            password: ""
        });
    }
    return users;
}