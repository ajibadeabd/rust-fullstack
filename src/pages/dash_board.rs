use dioxus::prelude::*;
// use super::component::side_bar;

use crate::component::{
    side_bar::Sidebar,
    nav_bar::Navbar
};


#[inline_props]
pub fn DashboardPage(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "min-h-screen bg-gray-100", "dash"
            Navbar{}
            Sidebar{}
            render!(rsx!(
                div { class: "ml-64 p-6 mt-20",  // Adjust margins and padding as needed
                    section { class: "mb-8",
                        h3 { class: "text-lg font-semibold mb-4", "Section 1" }
                        p { class: "text-gray-600", "This is the first section of the main content." }
                    }
                    section { class: "mb-8",
                        h3 { class: "text-lg font-semibold mb-4", "Section 2" }
                        p { class: "text-gray-600", "This is the second section of the main content." }
                    }
                    section { class: "mb-8",
                        h3 { class: "text-lg font-semibold mb-4", "Section 3" }
                        p { class: "text-gray-600", "This is the third section of the main content." }
                    }
                }
            ))
        }
    ))
}
