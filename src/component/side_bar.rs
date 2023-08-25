use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::{Route, context::SideBarOpen};

#[inline_props]
pub fn Sidebar(cx: Scope) -> Element {
    let is_side_nav_open_data = use_shared_state::<SideBarOpen>(cx).unwrap();
    let is_side_nav_open = match *is_side_nav_open_data.read(){
        SideBarOpen::True=>"true",
        SideBarOpen::False=>"false"
    }; 
    let current_theme = *is_side_nav_open_data.read();


     // Define route data as tuples (route, label)
                let routes = vec![
                    (Route::ProfilePage {}, "Profile"),
                    (Route::DashboardPage {}, "Dashboard"),
                    (Route::DepositPage {}, "Transactions"),
                    (Route::HomePage {}, "Log Out"),
                ];

    let toggle_cross = move |_| {
        *is_side_nav_open_data.write() = match current_theme {
        SideBarOpen::True=>SideBarOpen::False,
        SideBarOpen::False=>SideBarOpen::True,
    }
    };
                

    render!(rsx!(
        div {
            class: "hidden lg:block bg-gray-800 w-[25%] h-full fixed top-0 left-0 overflow-y-auto transition-transform duration-300 ease-in-out transform translate-x-0",
            class: if is_side_nav_open != "true" {
                "block bg-gray-800 w-[25%] h-full fixed top-0 left-0 overflow-y-auto transition-transform duration-300 ease-in-out transform translate-x-0"

            }else {
                "hidden lg:block bg-gray-800 w-[25%] h-full fixed top-0 left-0 overflow-y-auto transition-transform duration-300 ease-in-out transform translate-x-0"
            },
            div {
                class: "p-auto lg:p-6 ",
                div {
                    class:"block lg:hidden p-6",
                onclick: toggle_cross,
                        div { class: "w-6 h-0.5 bg-white transform rotate-45 origin-center" }
                    div { class: "w-6 h-0.5 bg-white transform -rotate-45 origin-center" }
                },
                h2 { class: "hidden lg:block text-2xl font-semibold text-white mb-4", "Rust-Fe" }

               
                // Use an unordered list with list items to display the links
                ul {
                    class: "list-none p-0 m-0",
                    // Use the map function to generate list items (li) for each route
                    for (route, label) in routes {
                        li {
                onclick: toggle_cross,
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