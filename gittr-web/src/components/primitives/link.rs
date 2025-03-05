use leptos::prelude::*;

stylance::import_crate_style!(style, "src/components/primitives/link.module.css");

#[component]
pub fn Link(
    href: &'static str,
    #[prop(optional)] external: bool,
    #[prop(optional)] active: bool,
    children: Children,
) -> impl IntoView {
    let link_class = style::link;
    let active_class = style::link_active;

    let merged_class = if active {
        format!("{} {}", link_class, active_class)
    } else {
        link_class.to_owned()
    };

    view! {
        <a class=merged_class href=href target=move || if external { "_blank" } else { "" } >{children()}</a>
    }
}
