export type Project = { name: string, id: number }

export const enum WorkCodes {
    Unset = "NotSet",
    Meetings = "Meetings",
    SoftwareDesign = "SoftwareDesign",
    Checkout = "Checkout",
}

export const WorkCodesIndexer = [
    WorkCodes.Unset,
    WorkCodes.Meetings,
    WorkCodes.SoftwareDesign,
    WorkCodes.Checkout,
]