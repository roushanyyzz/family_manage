#![allow(non_snake_case)]
use crate::views::home::Home;
use crate::views::page_not_found::PageNotFound;
use crate::views::signin::SignIn;
use dioxus::prelude::*;
pub use dioxus_router::prelude::*;

#[rustfmt::skip]
#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/sign")]
    SignIn {},
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
