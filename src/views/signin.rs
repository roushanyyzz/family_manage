#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn SignIn(cx: Scope) -> Element {
    render!(
        div {
            class: "flex min-h-screen flex-col",
            style: "background-image: url(http://csde-file.trinablue.com/SMART-BLUE/20240108/439EF3DE274A464CB2853971DACF9B3F_659bae6fd5dedeee667709fa.jpg)",
            div { class: "m-auto max-w-80 max-h-80 ",
                div { class: "h-12" }
                "重试测试"
            }
        }
    )
}
