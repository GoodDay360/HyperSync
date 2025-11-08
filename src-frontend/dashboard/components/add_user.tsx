// SolidJS Imports
import { onMount, createSignal } from "solid-js";
// import { useNavigate } from "@solidjs/router";


// SUID Imports

import {  TextField, Button } from "@suid/material"

// SUID Icons Imports


// Solid Toast
import { toast } from 'solid-toast';

// DayJS Imports
import dayjs from 'dayjs';
import utc from 'dayjs/plugin/utc';
import advancedFormat from 'dayjs/plugin/advancedFormat';

dayjs.extend(utc);
dayjs.extend(advancedFormat);



// Style Imports
import styles from "../styles/add_user.module.css";

// Scripts Imports
import { create_user } from "../scripts/user";



export default function AddUser({
    onClose=()=>{},
    onSuccess=()=>{}
}:{
    onClose: ()=>void
    onSuccess: ()=>void
}) {
    
    const [email, set_email] = createSignal("");
    const [username, set_username] = createSignal("");
    const [password, set_password] = createSignal("");


    onMount(()=>{

    })

    return (<>
        <div class={styles.container}>
            <form class={styles.box}
                onSubmit={(e)=>{
                    e.preventDefault();
                    if (!email() || !username() || !password()) return;

                    create_user(email(), username(), password())
                        .then(()=>{
                            toast.remove();
                            toast.success("User created successfully.", { style: {color:"green"}});
                            onSuccess();
                            onClose();
                        })
                        .catch(err => {
                            console.error(err);
                            toast.remove();
                            toast.error(err, { style: {color:"red"}});
                        })
                    
                }}
            >
                <div class={styles.header_container}>
                    <h2 class={styles.header_title}>Add User</h2>
                </div>

                <div class={styles.body_container}>
                    <TextField label="Email" variant="filled" required
                        sx={{
                            "& .MuiInputLabel-root": {
                                color:"var(--color-1)"
                            }
                        }}
                        inputProps={{ style: { color: "var(--color-1)" } }}
                        value={email()}
                        onChange={(e)=>{
                            set_email(e.target.value);
                        }}
                    />

                    <TextField label="Username" variant="filled" required
                        sx={{
                            "& .MuiInputLabel-root": {
                                color:"var(--color-1)"
                            }
                        }}
                        inputProps={{ style: { color: "var(--color-1)" } }}
                        value={username()}
                        onChange={(e)=>{
                            set_username(e.target.value);
                        }}
                    />

                    <TextField label="Password" variant="filled" required type="password"
                        sx={{
                            "& .MuiInputLabel-root": {
                                color:"var(--color-1)"
                            }
                        }}
                        inputProps={{ style: { color: "var(--color-1)" } }}
                        value={password()}
                        onChange={(e)=>{
                            set_password(e.target.value);
                        }}
                    />
                </div>

                <div class={styles.button_container}>
                    <Button variant="text" color="error" type="button"
                        sx={{
                            fontSize: "calc((100vw + 100vh)/2*0.015)"
                        }}
                        onClick={onClose}
                    >Cancel</Button>
                    <Button variant="contained" color="primary" type="submit"
                        sx={{
                            fontSize: "calc((100vw + 100vh)/2*0.015)"
                        }}
                    >Create</Button>
                </div>
            </form>

        </div>
    </>);
}