// Axios Imports
import axios from 'axios';

// Scripts Imports
import { API_HOST } from "./configs";

export const admin_login = async (username:string, password:string): Promise<void> => {
    return new Promise<void>((resolve, reject) => {
        axios.post(
            `${API_HOST}/api/admin/login`,
            { username, password }
        )
            .then(async (res) => {
                const data = res.data;
                const token:string = data.token;
                if (token.trim()) {
                    localStorage.setItem("admin_token", token);
                    resolve();
                }else{
                    reject("Invalid username or password.");
                }
            }).catch(err => {
                console.error(err);
                reject(err?.response?.data?.message);
            });
    })
    
}


export const verify_admin_login = async (): Promise<boolean> => {
    return new Promise<boolean>((resolve, reject) => {
        const token = localStorage.getItem("admin_token");
        axios.post(
            `${API_HOST}/api/admin/verify`,
            { token }
        )
            .then(async (res) => {
                const data = res.data;
                resolve(data);
            }).catch(err => {
                console.error(err);
                reject(err?.response?.data?.message);
            });
    })
    
}