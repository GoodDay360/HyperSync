// SolidJS Imports
import { createSignal, onMount, For } from "solid-js";
import { useNavigate, useLocation } from "@solidjs/router";


// SUID Imports

import {  Button, CircularProgress, ButtonBase, IconButton } from "@suid/material"

// SUID Icons Imports
import ArrowBackIosNewRoundedIcon from '@suid/icons-material/ArrowBackIosNewRounded';
import MenuRoundedIcon from '@suid/icons-material/MenuRounded';

// Solid Toast
// import { Toaster, toast } from 'solid-toast';

// Scripts Imports


// Style Imports
import styles from "../styles/dashboard.module.css";

// Component Imports
import User from "./user";

export default function Dashboard({
    onLogout= ()=>{}
}:{
    onLogout: ()=>void
}) {
    const location = useLocation();
    const navigate = useNavigate();

    const [current_menu, set_current_menu] = createSignal<string>("user");

    const [is_loading, set_is_loading] = createSignal(true);
    const [hide_menu, set_hide_menu] = createSignal(false);

    onMount(()=>{
        set_is_loading(true);
        console.log(location.pathname);
        const default_path = "/admin/dashboard/user";
        if (["/admin/dashboard", "/admin"].includes(location.pathname)) {
            navigate(default_path, { replace: true });
            set_current_menu(default_path);
        }else{
            const is_exist = MENU.find((item) => item.path === location.pathname);
            if (!is_exist) {
                navigate(default_path, { replace: true });
                set_current_menu(default_path);
            }else{
                set_current_menu(location.pathname);
            }
        }
        set_is_loading(false);
    })

    return (<>
        {is_loading() && 
            <div class={styles.loader_container}>
                <CircularProgress color="primary" />
            </div>
        }

        {!is_loading() && 
            <div class={styles.container}>
                <div class={styles.menu_container}
                    style={{
                        background: hide_menu() ? "transparent" : "var(--background-2)"
                    }}
                >
                    {!hide_menu() 
                        ? <>
                            <div class={styles.menu_header_container}>
                                <h2 class={styles.menu_title}>Dashboard</h2>
                                <ButtonBase
                                    sx={{
                                        color:"var(--color-1)",
                                        fontSize: "calc((100vw + 100vh)/2*0.02)",
                                        height: "100%"
                                    }}
                                    onClick={() => set_hide_menu(true)}
                                >
                                    <ArrowBackIosNewRoundedIcon/>
                                </ButtonBase>
                                
                            </div>
                            <div class={styles.menu_item_container}>
                                <For each={MENU}>
                                    {(item) => (
                                        <ButtonBase
                                            sx={{
                                                padding: "6px 12px",
                                                color:"var(--color-1)",
                                                fontSize: "calc((100vw + 100vh)/2*0.02)",
                                                background: location.pathname === item.path ? "var(--background-3)" : "transparent"
                                            }}
                                            onClick={() => {
                                                navigate(item.path);
                                                set_current_menu(item.path);
                                            }}
                                        >{item.label}</ButtonBase>
                                    )}
                                </For>
                            </div>

                            <div class={styles.menu_item_bottom_container}>
                                <Button color="error" variant="contained"
                                    sx={{
                                        fontSize: "calc((100vw + 100vh)/2*0.0125)"
                                    }}
                                    onClick={() => {
                                        localStorage.removeItem("admin_token");
                                        navigate("/admin");
                                        onLogout();
                                    }}
                                >Logout</Button>
                            </div>
                        </>
                        : <div
                            style={{
                                "padding-top": "12px",
                                "padding-left": "12px"
                            }}
                        >
                            <IconButton
                                sx={{
                                    background: "var(--background-2)",
                                    color:"var(--color-1)",
                                    fontSize: "calc((100vw + 100vh)/2*0.025)"
                                }}
                                onClick={() => set_hide_menu(false)}
                            >
                                <MenuRoundedIcon color="inherit" fontSize="inherit"/>
                            </IconButton>
                        </div>
                    }

                </div>

                <For each={MENU}>
                    {(item) => (
                        <>{(current_menu() === item.path) &&
                            <item.component />
                        }</>
                    )}
                </For>
                
            </div>
        }
    </>);
}

const MENU = [{
    label: "User",
    path: "/admin/dashboard/user",
    component: User
}]