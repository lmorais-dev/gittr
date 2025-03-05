use leptos::prelude::*;
use stylance::import_crate_style;

use crate::components::primitives::link::*;

import_crate_style!(style, "src/components/gittr/main_header.module.css");

#[component]
pub fn MainHeader(#[prop(optional)] is_admin: bool) -> impl IntoView {
    view! {
        <header>
            <hgroup class=style::headers>
                <h1 class=style::title>gittr</h1>
                <h2 class=style::sub_title>minimalist, stealth, fast git client</h2>
            </hgroup>
            <nav>
                <Link href="/" active=true>Home</Link>
                <Link href="/repositories">Repositories</Link>
                <Link href="/profile">Profile</Link>
                <Link href="/admin">Admin Area</Link>
                <Link href="/logout">Log Out</Link>
            </nav>
        </header>
    }
}
