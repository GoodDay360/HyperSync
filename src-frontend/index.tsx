/* @refresh reload */
import { render } from "solid-js/web";
import { Router, Route, useNavigate } from "@solidjs/router";

// Component Imports
import App from "@src/app/components/app";
import Dashboard from "@src/dashboard/components/dashboard";

render(() => (
    <Router>
        <Route path="/admin" component={App}/>
        <Route path="/admin/dashboard" component={Dashboard} />
    </Router>
), document.getElementById("root") as HTMLElement);