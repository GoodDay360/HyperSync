// SolidJS Imports
import { createSignal, onMount, For } from "solid-js";
import { useNavigate, Route, Router, useLocation } from "@solidjs/router";


// SUID Imports

import {  Button, CircularProgress, ButtonBase } from "@suid/material"

// SUID Icons Imports


// Solid Toast
// import { Toaster, toast } from 'solid-toast';

// Scripts Imports
import { verify_admin_login } from "@src/app/scripts/app";

// Style Imports
import styles from "../styles/dashboard.module.css";

// Component Imports
import User from "./user";

export default function Dashboard() {
    const location = useLocation();
    const navigate = useNavigate();

    const [is_loading, set_is_loading] = createSignal(true);

    onMount(()=>{
        verify_admin_login()
            .then((state) => {
                if (!state) {
                    navigate("/admin/dashboard");
                }
                set_is_loading(false);
            })
            .catch(err => {
                console.error(err);
                navigate("/admin");
                set_is_loading(false);
            })

        if (location.pathname === "/admin/dashboard") {
            navigate("/admin/dashboard/user");
        }
    })

    return (<>
        {is_loading() && 
            <div class={styles.loader_container}>
                <CircularProgress color="primary" />
            </div>
        }

        {!is_loading() && 
            <div class={styles.container}>
                <div class={styles.menu_container}>
                    <h2 class={styles.menu_title}>Dashboard</h2>
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
                                    onClick={() => navigate(item.path)}
                                >{item.label}</ButtonBase>
                            )}
                        </For>
                    </div>

                    <div class={styles.menu_item_bottom_container}>
                        <Button color="error" variant="contained"
                            sx={{
                                fontSize: "calc((100vw + 100vh)/2*0.0125)"
                            }}
                        >Logout</Button>
                    </div>
                </div>
                <Router>
                    <Route path="/user" component={User} />
                </Router>
            </div>
        }
    </>);
}

const MENU = [{
    label: "User",
    path: "/admin/dashboard/user"
}]