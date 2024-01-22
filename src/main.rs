#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use family_manage::route::router::Route;

fn main() {
    dioxus_web::launch(App)
}

fn App(cx: Scope) -> Element {
    render! { Router::<Route> {} }
}
