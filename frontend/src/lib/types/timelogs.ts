enum Status {
    Approved,
    Initial,
    Submitted,
}

interface TimeEntry {
    id: number,
    jobNumber: number,
    jobDescription: string,
    workCode: number,
    employeeId: number,
    dateOfWork: Date,
    hoursWorked: number,
    submitStatus: Status,
}

type Optional<Type> = {
    [Property in keyof Type]+?: Type[Property];
};

interface ConcreteRequest {
    userId: number,
    managerId: number,
    weekEndDate: Date,
    status: Status,
}


type TimelogRequest = Optional<ConcreteRequest>;

interface ApprovedIds {
    ids: Array<number>,
}