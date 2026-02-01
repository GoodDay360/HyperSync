export type Users = {
    id: string;
    email: string;
    username: string;
    favorite_count: number,
    watch_state_count: number,
    timestamp: number;
    status: boolean;
}[]

export interface GetAllUser{
    status: number;
    data: Users;
}
