/* @refresh reload */
import { render } from "solid-js/web";
import { Router, Route } from "@solidjs/router";

// Component Imports
import App from "@src/app/components/app";

render(() => (
    <Router>
        <Route path="/admin/*" component={App}/>
    </Router>
), document.getElementById("root") as HTMLElement);