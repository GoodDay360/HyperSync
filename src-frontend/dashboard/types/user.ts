export type Users = {
    id: string;
    email: string;
    username: string;
    timestamp: number;
    status: boolean;
}[]

export interface GetAllUser{
    status: number;
    data: Users;
}
