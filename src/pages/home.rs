use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::Route;

#[inline_props]
pub fn HomePage(cx: Scope) -> Element {
    render!(rsx!(
        div { class: "min-h-screen flex items-center justify-center bg-gradient-to-r from-blue-400 to-purple-500",
            div { class: "w-full max-w-md bg-white p-6 rounded-lg shadow-md",
                h2 { class: "text-3xl font-semibold mb-4 text-center text-gray-800", "Welcome to the Test App" },
                p { class: "mb-4 text-gray-600", "This is a simple test app built with Rust and Dioxus." },
                p { class: "mb-4 text-gray-600", "Feel free to explore and experiment with the app." },
                p { class: "mb-2 text-gray-600", "Please note that this is for demonstration purposes only." },
                p { class: "text-sm text-gray-500", "Created by kord::rs/ts" }
                span { "You can click on the links below to navigate:" }
                    div { class: "mt-2",
                    ul { class: "space-y-2",
                   
                    li { rsx!{
                        Link {
                            to: Route::DashboardPage {  },
                            "Dashboard!",
                            class: "ttext-gray hover:text-blue-500 cursor-pointer",
                        }
                    }}
                    li { rsx!{
                        Link {
                            to: Route::LoginPage  {},
                            "Login",
                            class: "text-gray hover:text-blue-500 cursor-pointer",
                        }
                    }}
                    li { rsx!{
                        Link {
                            to: Route::ProfilePage{},
                            "Profile",
                            class: "text-gray hover:text-blue-500 cursor-pointer",
                        }
                    }}
                    }
                    }
                }
        }
    ))
}
