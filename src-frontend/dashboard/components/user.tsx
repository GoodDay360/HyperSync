// SolidJS Imports
import { createSignal, onMount } from "solid-js";
import { useNavigate, Route, Router } from "@solidjs/router";


// SUID Imports

import {  Button, CircularProgress, ButtonBase, IconButton } from "@suid/material"

// SUID Icons Imports
import MenuIcon from '@suid/icons-material/Menu';

// Solid Toast
// import { Toaster, toast } from 'solid-toast';

// Scripts Imports
import { verify_admin_login } from "@src/app/scripts/app";

// Style Imports
import styles from "../styles/user.module.css";


export default function User() {
    const navigate = useNavigate();

    const [is_loading, set_is_loading] = createSignal(true);

    onMount(()=>{
        
    })

    return (<>
        <div class={styles.container}>
            <div class={styles.header_container}>
                <IconButton
                    sx={{
                        color:"var(--color-1)",
                        fontSize: "calc((100vw + 100vh)/2*0.035)"
                    }}
                >
                    <MenuIcon color="inherit" fontSize="inherit"/>
                </IconButton>
            </div>

        </div>
    </>);
}