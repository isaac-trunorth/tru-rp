import { WorkCode } from "./timelogs"
export const WorkCodesIndexer = [
    WorkCode.Unset,
    WorkCode.Meetings,
    WorkCode.SoftwareDev,
    WorkCode.Checkout,
]

export interface Project {
    id: number,
    jobNumber: number,
    jobDescription: string,
    jobActive: boolean,
}