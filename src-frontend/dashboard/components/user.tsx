// SolidJS Imports
import { onMount, For, createSignal } from "solid-js";
// import { useNavigate } from "@solidjs/router";


// SUID Imports

import {  IconButton, Checkbox } from "@suid/material"

// SUID Icons Imports
import ChevronRightRoundedIcon from '@suid/icons-material/ChevronRightRounded';
import ChevronLeftRoundedIcon from '@suid/icons-material/ChevronLeftRounded';
import AddCircleRoundedIcon from '@suid/icons-material/AddCircleRounded';
import BackspaceRoundedIcon from '@suid/icons-material/BackspaceRounded';
import DeleteForeverRoundedIcon from '@suid/icons-material/DeleteForeverRounded';

// Solid Toast
import { toast } from 'solid-toast';

// DayJS Imports
import dayjs from 'dayjs';
import utc from 'dayjs/plugin/utc';
import advancedFormat from 'dayjs/plugin/advancedFormat';
import timezone from 'dayjs/plugin/timezone';

dayjs.extend(utc);
dayjs.extend(advancedFormat);
dayjs.extend(timezone);

// Scripts Imports
import { get_all_users, delete_user } from "../scripts/user";

// Style Imports
import styles from "../styles/user.module.css";

// Types Imports
import { Users } from "../types/user";

// Component Imports
import AddUser from "./add_user";


export default function User() {
    
    const [USER_DATA, SET_USER_DATA] = createSignal<Users>([]);

    const [delete_user_id, set_delete_user_id] = createSignal<string[]>([]);

    const [is_loading, set_is_loading] = createSignal(true);
    const [page, set_page] = createSignal<number>(1);
    const [search, set_search] = createSignal<string>("");

    const [is_add_user, set_is_add_user] = createSignal(false);


    const get_data = () => {
        console.log(page(), search());
        set_is_loading(true);
        get_all_users(page(), search())
            .then(data => {
                console.log(data);
                SET_USER_DATA(data);
            })
            .catch(err => {
                console.error(err);
                toast.remove()
                toast.error(err, { style: {color:"red"}});
            })
            .finally(() => {
                set_is_loading(false);
            });
    }

    onMount(()=>{
        console.log("User");
        get_data();
    })

    return (<>
        <div class={styles.container}>
            <div class={styles.header_container}>
                <form class={styles.search_container}
                    onSubmit={(e)=>{
                        e.preventDefault();
                        set_page(1);
                        get_data();
                    }}
                >
                    <input class={styles.search_input} type="text" placeholder="Search"
                        value={search()}
                        onChange={(e)=>{
                            set_search(e.target.value)
                        }}
                    />
                    <button type="submit" style="display: none;"></button>
                </form>
                {(delete_user_id().length > 0) &&
                    <IconButton disabled={is_loading()}
                        sx={{
                            color:"var(--color-1)",
                            fontSize: "calc((100vw + 100vh)/2*0.025)"
                        }}
                        onClick={() => {
                            set_is_loading(true);
                            delete_user(delete_user_id())
                                .then(() => {
                                    toast.remove();
                                    toast.success("User deleted successfully.", { style: {color:"green"}});
                                    set_delete_user_id([]);
                                    get_data();
                                })
                                .catch(err => {
                                    console.error(err);
                                    toast.remove()
                                    toast.error(err, { style: {color:"red"}});
                                })
                                .finally(() => {
                                    set_is_loading(false);
                                })
                        }}
                    >
                        <DeleteForeverRoundedIcon color="error" fontSize="inherit" />
                    </IconButton>
                }

                {delete_user_id().length === 0 &&
                    <IconButton disabled={is_loading()}
                        sx={{
                            background: "var(--background-2)",
                            color:"var(--color-1)",
                            fontSize: "calc((100vw + 100vh)/2*0.025)"
                        }}
                        onClick={() => {
                            set_is_add_user(true);
                        }}
                    >
                        <AddCircleRoundedIcon color="inherit" fontSize="inherit"/>
                    </IconButton>
                }
            </div>

            <div class={styles.body_container}>
                <div class={styles.table_container}>
                    <table class={styles.table}>
                        <thead>
                            <tr>
                                <th><BackspaceRoundedIcon color="error" fontSize="inherit"/></th>
                                <th>Email</th>
                                <th>Username</th>
                                <th>Favorite</th>
                                <th>Watch State</th>
                                <th>Status</th>
                                <th>Datetime</th>
                            </tr>
                        </thead>
                        <tbody>
                            <For each={USER_DATA()}>
                                {(item) => (
                                    <tr>
                                        <td>
                                            <Checkbox 
                                                sx={{
                                                    color: "var(--color-1)"
                                                }}
                                                value={delete_user_id().includes(item.id)}
                                                onChange={(e)=>{
                                                    if (e.target.checked) {
                                                        set_delete_user_id([...delete_user_id(), item.id]);
                                                    }else{
                                                        set_delete_user_id(delete_user_id().filter(id => id !== item.id));
                                                    }
                                                }}
                                            />
                                            
                                        </td>
                                        <td>{item.email}</td>
                                        <td>{item.username}</td>
                                        <td>{item.favorite_count}</td>
                                        <td>{item.watch_state_count}</td>
                                        <td>{String(item.status)}</td>
                                        <td>{dayjs.utc(item.timestamp).tz(dayjs.tz.guess()).format('DD MMMM YYYY | hh:mm:ss a')}</td>
                                    </tr>
                                )}
                            </For>
                        </tbody>
                    </table>
                </div>
                <div class={styles.pagination_container}>
                    <IconButton
                        sx={{
                            background: "var(--background-2)",
                            color:"var(--color-1)",
                            fontSize: "calc((100vw + 100vh)/2*0.02)"
                        }}
                        onClick={() => {
                            if (is_loading()) return;
                            let new_page = page()-1 < 1 ? 1 : page()-1;
                            set_page(new_page);
                            get_data();
                        }}
                    >
                        <ChevronLeftRoundedIcon color="inherit" fontSize="inherit" />
                    </IconButton>
                    <span class={styles.page_text}>{page()}</span>
                    <IconButton
                        sx={{
                            background: "var(--background-2)",
                            color:"var(--color-1)",
                            fontSize: "calc((100vw + 100vh)/2*0.02)"
                        }}
                        onClick={() => {
                            if (is_loading()) return;
                            set_page(page()+1);
                            get_data();
                        }}
                    >
                        <ChevronRightRoundedIcon color="inherit" fontSize="inherit" />
                    </IconButton>
                </div>
            </div>

            

        </div>
        
        {is_add_user() &&
            <AddUser 
                onClose={()=>{
                    set_is_add_user(false);
                }}

                onSuccess={()=>{
                    get_data();
                }}
            />
        }
        

    </>);
}