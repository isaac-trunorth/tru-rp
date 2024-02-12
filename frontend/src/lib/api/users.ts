import type { Token, User } from "$lib/model/users";
import { jwtDecode } from "jwt-decode";
import { url } from "./timelog";

const exampleEmployees = [
    'Employee0',
    'Isaac',
    'John Doe',
    'Jane Doe',
]

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
export async function updatePassword(user: User, auth: Token): Promise<string> {
    const res = await fetch([url, 'users', 'update_pwd'].join("/"), {
        method: "POST",
        headers: [['Authorization', auth.tokenText], ['content-type', 'application/json']],
        body: JSON.stringify(user)
    })
    const body = await res.text();
    return body;
}

export async function updateUser(user: User, auth: Token): Promise<String> {
    const res = await fetch([url, 'users'].join("/"), {
        method: "PUT",
        headers: [['Authorization', auth.tokenText], ['content-type', 'application/json']],
        body: JSON.stringify(user)
    })
    return await res.text();
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

export async function getUsers(auth: Token): Promise<User[]> {
    const res = await fetch([url, 'users'].join("/"), {
        method: "GET",
        headers: [['Authorization', auth.tokenText], ['content-type', 'application/json']],
    })
    const body: User[] = await res.json();
    return body;
}

export async function getManagersEmployees(auth: Token): Promise<User[]> {
    const res = await fetch([url, 'users', 'by_manager', `${auth.userId}`].join("/"), {
        method: "GET",
        headers: [['Authorization', auth.tokenText], ['content-type', 'application/json']],
    })
    const body: User[] = await res.json();
    return body;
}