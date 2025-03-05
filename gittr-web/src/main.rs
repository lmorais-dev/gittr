mod components;
mod views;

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
use stylance::import_crate_style;
use views::home::HomeView;

use components::gittr::main_header::MainHeader;

import_crate_style!(style, "src/app.module.css");

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| {
        view! {
            <div class=style::container>
                <App />
            </div>
        }
    })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <MainHeader />
            <main>
                <Routes fallback=|| "Not Found">
                    <Route path=path!("/") view=HomeView />
                </Routes>
            </main>
        </Router>
    }
}
