// SolidJS Imports
import { createSignal, onMount } from "solid-js";
import { useNavigate } from "@solidjs/router";


// SUID Imports
import { createTheme, ThemeProvider } from "@suid/material/styles";
import { TextField, Button, CircularProgress } from "@suid/material"

// Solid Toast
import { Toaster, toast } from 'solid-toast';

// Scripts Imports
import { admin_login, verify_admin_login } from "../scripts/app";

// Style Imports
import 'bootstrap/dist/css/bootstrap.min.css';
import 'animate.css';
import "../styles/app.css";
import styles from "../styles/app.module.css";

const theme = createTheme({
    typography: {
        fontFamily: 'var(--font-family)',
    },
});




export default function App() {
    const navigate = useNavigate();

    const [is_loading, set_is_loading] = createSignal(true);

    const [admin_username, set_admin_username] = createSignal("");
    const [admin_password, set_admin_password] = createSignal("");
    
    onMount(()=>{
        verify_admin_login()
            .then((state) => {
                if (state) {
                    navigate("/admin/dashboard");
                }else{
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

    return (<ThemeProvider theme={theme}>
        <Toaster 
            position="bottom-center"
            gutter={5}
            toastOptions={{
                duration: 5000,
                style: {
                    "min-width": "fit-content",
                    border:"2px solid var(--background-1)",
                    background: 'var(--background-2)',
                    color: "var(--color-1)",
                    "font-family": "var(--font-family)",
                    "margin-bottom": "env(safe-area-inset-bottom, 0)"
                },
            }}
        />
        <div class="app">
            {is_loading() && 
                <div class={styles.loader_container}>
                    <CircularProgress color="primary" />
                </div>
            }
            
            {!is_loading() && 
                <div class={styles.container}>
                    <form class={styles.form_container}
                        onSubmit={(e)=>{
                            e.preventDefault();
                            admin_login(admin_username(), admin_password())
                                .then(() => {
                                    navigate("/admin/dashboard");
                                })
                                .catch(err => {
                                    toast.remove();
                                    toast.error(err, {style:{color:"red"}});
                                });
                        }}
                    >
                        <h2 class={styles.form_title}>Admin</h2>
                        <div class={styles.input_box}>
                            <TextField label="Username" variant="filled" required
                                sx={{
                                    "& .MuiInputLabel-root": {
                                        color:"var(--color-1)"
                                    }
                                }}
                                inputProps={{ style: { color: "var(--color-1)" } }}
                                value={admin_username()}
                                onChange={(e)=>{
                                    set_admin_username(e.target.value);
                                }}
                            />
                            <TextField label="Password" variant="filled" type="password" required
                                sx={{
                                    "& .MuiInputLabel-root": {
                                        color:"var(--color-1)"
                                    }
                                }}
                                inputProps={{ style: { color: "var(--color-1)" } }}
                                value={admin_password()}
                                onChange={(e)=>{
                                    set_admin_password(e.target.value);
                                }}
                            />
                            <Button variant="contained" color="primary" type="submit"
                                sx={{
                                    fontSize: "calc((100vw + 100vh)/2*0.015)"
                                }}
                            >Login</Button>
                        </div>
                    </form>
                </div>
            }


            
        </div>
    </ThemeProvider>);
}