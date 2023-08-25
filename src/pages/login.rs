use dioxus::prelude::*;
use dioxus_router::prelude::{use_navigator, Link};

use crate::Route;

#[inline_props]
pub fn LoginPage(cx: Scope) -> Element {
    let nav = use_navigator(cx);
    render!(rsx!(
        div { class: "min-h-screen flex items-center justify-center bg-gradient-to-r from-blue-400 to-purple-500",
            div { class: "w-full max-w-md bg-white p-6 rounded-lg shadow-md",
                h2 { class: "text-2xl font-semibold mb-4", "Login" },
                form { class: "space-y-4",
                    div { class: "flex flex-col",
                        label { class: "text-sm font-medium mb-1", "Username" },
                        input { class: "px-3 py-2 border rounded-lg focus:outline-none focus:border-blue-500", "type": "text", "placeholder": "Enter your username" }
                    }
                    div { class: "flex flex-col",
                        label { class: "text-sm font-medium mb-1", "Password" },
                        input { class: "px-3 py-2 border rounded-lg focus:outline-none focus:border-blue-500", "type": "password", "placeholder": "Enter your password" }
                    }
                    button { 
                         class: "px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 focus:outline-none focus:bg-blue-600",  
                        onclick: move |event| {
                                        nav.push(Route::DashboardPage {  });
                        },
                            "Login",
            }
                     
                }
            }
        }
    ))
}


// button { 
//                         onclick: move |event| {
//                                         nav.push(Route::PageNotFound {  })

//             },