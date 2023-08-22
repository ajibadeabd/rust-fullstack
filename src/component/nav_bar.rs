use dioxus::prelude::*;


#[inline_props]
pub fn Navbar(cx: Scope) -> Element {
    cx.render(rsx!(
        header { class: "bg-white w-full h-16 shadow-lg fixed top-0 left-0 flex items-center justify-between px-6",
            h1 { class: "text-2xl font-semibold text-gray-800", "Dashboard" }
            div { class: "flex items-center space-x-4",
                span { class: "text-gray-600", "Welcome, User!" }
                a { class: "w-8 h-8 bg-gray-300 rounded-full" }  // Replace with your avatar component
            }
        }
    ))
}
