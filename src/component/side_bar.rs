use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::Route;

#[inline_props]
pub fn Sidebar(cx: Scope) -> Element {
     // Define route data as tuples (route, label)
                let routes = vec![
                    (Route::ProfilePage {}, "Profile"),
                    (Route::DashboardPage {}, "Dashboard"),
                    (Route::DepositPage {}, "Transactions"),
                    (Route::HomePage {}, "Log Out"),
                ];

    render!(rsx!(
        div {
            class: "hidden lg:block bg-gray-800 w-[25%] h-full fixed top-0 left-0 overflow-y-auto transition-transform duration-300 ease-in-out transform translate-x-0",
            div {
                class: "p-6",
                h2 { class: "text-2xl font-semibold text-white mb-4", "Rust-Fe" }

               
                // Use an unordered list with list items to display the links
                ul {
                    class: "list-none p-0 m-0",
                    // Use the map function to generate list items (li) for each route
                    for (route, label) in routes {
                        li {
                            class: "common-pointer flex flex-col items-start justify-start p-[10px] w-full",
                            rsx! {
                                Link {
                                    to: route,
                                    label,
                                    class: "flex text-white flex-row gap-5 items-center justify-start md:ml-[0] ml-[23px] w-[65%] md:w-full",
                                }
                            }
                        }
                    }
                }
            }
        }
    ))
} 