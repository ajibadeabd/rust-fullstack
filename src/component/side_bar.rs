use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::Route;

#[inline_props]
pub fn Sidebar(cx: Scope) -> Element {
    render!(rsx!(
        aside { class: "bg-gray-800 w-64 h-full fixed top-0 left-0 overflow-y-auto transition-transform duration-300 ease-in-out transform translate-x-0",
            div { class: "p-6",
                h2 { class: "text-2xl font-semibold text-white mb-4", "Sidebar" }
                ul { class: "space-y-2",
                li { rsx!{
                    Link {
                        to: Route::HomePage {},
                        "Go home!",
                        class: "text-white hover:text-blue-500 cursor-pointer",
                    }
                }}
                li { rsx!{
                    Link {
                        to: Route::DashboardPage {  },
                        "Dashboard!",
                        class: "text-white hover:text-blue-500 cursor-pointer",
                    }
                }}
                li { rsx!{
                    Link {
                        to: Route::LoginPage  {},
                        "Login",
                        class: "text-white hover:text-blue-500 cursor-pointer",
                    }
                }}
                li { rsx!{
                    Link {
                        to: Route::ProfilePage{},
                        "Profile",
                        class: "text-white hover:text-blue-500 cursor-pointer",
                    }
                }}
                }
            }
        }
    ))
}
