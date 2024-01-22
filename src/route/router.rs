#![allow(non_snake_case)]
use crate::views::signin::SignIn;
use dioxus::prelude::*;
pub use dioxus_router::prelude::*;

#[derive(Routable, Clone)]
pub enum Route {
    #[route("/sign-in")]
    SignIn {},
}
