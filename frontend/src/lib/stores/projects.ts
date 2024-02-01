import { createProject, getProjects } from "$lib/api/projects";
import type { Project } from "$lib/model/general";
import type { Token } from "$lib/model/users";
import { writable } from "svelte/store";

const { subscribe, set, update } = writable<Project[]>([]);

export const projects = {
    subscribe,
    set,
    update,
    getProjects: async (auth: Token) => {
        set(await getProjects(auth));
    },
    createNew: async (project: Project, auth: Token) => {
        const newProject = await createProject(project, auth);
        update(data => {
            data.push(newProject);
            return data;
        })
    }
}