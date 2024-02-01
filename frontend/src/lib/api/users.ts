import type { Token, User } from "$lib/model/users";
import { jwtDecode } from "jwt-decode";
import { url } from "./timelog";

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
            managerId: managerId,
            name: exampleEmployees[i],
            password: "",
            accessLevel: 1,
        });
    }
    return users;
}

export async function login(user: User): Promise<Token> {
    const res = await fetch([url, 'login'].join("/"), {
        method: "POST",
        headers: [['content-type', 'application/json']],
        body: JSON.stringify(user)
    })
    const body = await res.text();
    const decoded = jwtDecode(body) as unknown as Token;
    const token = {
        userId: decoded.userId,
        accessLevel: decoded.accessLevel,
        exp: decoded.exp,
        tokenText: body,
    };
    return token;
}

export async function createUser(user: User, auth: Token): Promise<User> {
    const res = await fetch([url, 'users'].join("/"), {
        method: "POST",
        headers: [['Authorization', auth.tokenText], ['content-type', 'application/json']],
        body: JSON.stringify(user)
    })
    const body: User = await res.json();
    return body;
}