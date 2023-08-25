 
use dioxus::prelude::*;

use crate::context::SideBarOpen;

#[inline_props]
pub fn Navbar(cx: Scope) -> Element {

    let is_side_nav_open_data = use_shared_state::<SideBarOpen>(cx).unwrap();
    
    let current_theme = *is_side_nav_open_data.read();
     
    let show_cross = use_state(cx, || false);

    let toggle_cross = move |_| {
        *is_side_nav_open_data.write() = match current_theme {
        SideBarOpen::True=>SideBarOpen::False,
        SideBarOpen::False=>SideBarOpen::True,
    };
        // show_cross.set(!show_cross.get());
    };

    cx.render(rsx!(
        header { class: "bg-white w-full h-16 shadow-lg fixed top-0 left-0 flex items-center justify-between px-6",
                if *show_cross.get() == false {
            rsx!(div { class: "space-y-2",
                onclick: toggle_cross,
                div { class: "w-8 h-0.5 bg-gray-600" },
                div { class: "w-8 h-0.5 bg-gray-600" },
                div { class: "w-8 h-0.5 bg-gray-600" },
            })
            }else {
                rsx!(
                    div {
                onclick: toggle_cross,
                        div { class: "w-6 h-0.5 bg-gray-600 transform rotate-45 origin-center" }
                    div { class: "w-6 h-0.5 bg-gray-600 transform -rotate-45 origin-center" }
                }
                )
            }
            
            div { class: "flex items-center space-x-4",
                span { class: "text-gray-600", "Welcome, User!" }
                a { class: "w-8 h-8 bg-gray-300 rounded-full" }  // Replace with your avatar component
            }
        }
    ))
}
