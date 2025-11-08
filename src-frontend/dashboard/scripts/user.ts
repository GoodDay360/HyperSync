// Axios Imports
import axios from 'axios';

// Scripts Imports
import { API_HOST } from "@src/app/scripts/configs";

// Types Imports
import { Users, GetAllUser } from "@src/dashboard/types/user";

export const get_all_users = async (page:number, search:string): Promise<Users> => {
    return new Promise<Users>((resolve, reject) => {
        const token = localStorage.getItem("admin_token");
        axios.post<GetAllUser>(
            `${API_HOST}/api/admin/get_all_user`,
            { page, search },
            {
                headers: {
                    "Authorization": token
                }
            }
        )
            .then(async (res) => {
                const data = res.data;
                console.log(data)
                resolve(data.data);
            }).catch(err => {
                console.error(err);
                reject(err?.response?.data?.message);
            });
    })
    
}

export const create_user = async (email:string, username:string, password:string): Promise<void> => {
    return new Promise<void>((resolve, reject) => {
        const token = localStorage.getItem("admin_token");
        axios.post(
            `${API_HOST}/api/admin/create_user`,
            { email, username, password },
            {
                headers: {
                    "Authorization": token
                }
            }
        )
            .then(async () => {
                resolve();
            }).catch(err => {
                console.error(err);
                reject(err?.response?.data?.message);
            });
    })
    
}

export const delete_user = async (data:string[]): Promise<void> => {
    return new Promise<void>((resolve, reject) => {
        const token = localStorage.getItem("admin_token");
        axios.post(
            `${API_HOST}/api/admin/delete_user`,
            { data },
            {
                headers: {
                    "Authorization": token
                }
            }
        )
            .then(async () => {
                resolve();
            }).catch(err => {
                console.error(err);
                reject(err?.response?.data?.message);
            });
    })
    
}