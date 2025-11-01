// SolidJS Imports
import { createSignal, onMount } from "solid-js";
import { Router, Route, useNavigate } from "@solidjs/router";


// SUID Imports
import { createTheme, ThemeProvider } from "@suid/material/styles";
import { TextField, Button, CircularProgress, ButtonBase, IconButton } from "@suid/material"

// SUID Icons Imports
import MenuIcon from '@suid/icons-material/Menu';

// Solid Toast
import { Toaster, toast } from 'solid-toast';

// Scripts Imports
import { verify_admin_login } from "@src/app/scripts/app";

// Style Imports
import styles from "../styles/dashboard.module.css";


export default function Dashboard() {
    const navigate = useNavigate();

    const [is_loading, set_is_loading] = createSignal(true);

    onMount(()=>{
        verify_admin_login()
            .then((state) => {
                if (!state) {
                    navigate("/admin");
                }
                set_is_loading(false);
            })
            .catch(err => {
                console.error(err);
                navigate("/admin");
                set_is_loading(false);
            })
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
                        <ButtonBase
                            sx={{
                                padding: "6px 12px",
                                color:"var(--color-1)",
                                fontSize: "calc((100vw + 100vh)/2*0.02)"
                            }}

                        >Manage User</ButtonBase>
                    </div>

                    <div class={styles.menu_item_bottom_container}>
                        <Button color="error" variant="contained"
                            sx={{
                                fontSize: "calc((100vw + 100vh)/2*0.0125)"
                            }}
                        >Logout</Button>
                    </div>
                </div>
                <div class={styles.content_container}>
                    <div class={styles.content_header_container}>
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
            </div>
        }
    </>);
}