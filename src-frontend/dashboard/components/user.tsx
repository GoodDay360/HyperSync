// SolidJS Imports
import { onMount } from "solid-js";
// import { useNavigate } from "@solidjs/router";


// SUID Imports

import {  IconButton } from "@suid/material"

// SUID Icons Imports
import MenuIcon from '@suid/icons-material/Menu';

// Solid Toast
// import { Toaster, toast } from 'solid-toast';

// Scripts Imports


// Style Imports
import styles from "../styles/user.module.css";


export default function User() {
    

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