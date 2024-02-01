import type { Project } from "$lib/model/general";
import type { Token } from "$lib/model/users";
import { url } from "./timelog";

const projects = "projects";

export async function getProjects(auth: Token): Promise<Project[]> {
    const res = await fetch(url + "/" + projects, {
        method: 'GET',
        headers: [['Authorization', auth.tokenText]],
    })
    const asJson: Project[] = await res.json();
    return asJson;
}

export async function createProject(project: Project, auth: Token): Promise<Project> {
    const res = await fetch([url, projects].join("/"), {
        method: "POST",
        headers: [['Authorization', auth.tokenText], ['content-type', 'application/json']],
        body: JSON.stringify(project)
    })
    return await res.json();
}