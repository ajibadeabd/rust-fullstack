use dioxus::prelude::*;
use crate::component::{
    side_bar::Sidebar,
    nav_bar::Navbar
};

#[inline_props]
pub fn ProfilePage(cx: Scope) -> Element {
    render!(rsx!(
        div { class: "ml-64 p-6",  // Adjust margins and padding as needed
        Navbar{}
        Sidebar{}
            h2 { class: "text-2xl font-semibold mb-4", "Profile" }
            div { class: "flex items-center mb-4",
                div { class: "w-16 h-16 bg-gray-300 rounded-full" }  // Replace with your avatar component
                div { class: "ml-4",
                    h3 { class: "text-xl font-semibold", "User Name" }  // Replace with user's name
                    p { class: "text-gray-600", "User Role" }  // Replace with user's role
                }
            }
            section { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-4", "Personal Information" }
                div { class: "flex mb-2",
                    span { class: "w-1/4 font-semibold text-gray-800", "First Name:" }
                    span { class: "w-3/4 text-gray-600", "Ajibade" }  // Replace with user's first name
                }
                div { class: "flex mb-2",
                    span { class: "w-1/4 font-semibold text-gray-800", "Last Name:" }
                    span { class: "w-3/4 text-gray-600", "Akorede" }  // Replace with user's last name
                }
                div { class: "flex mb-2",
                    span { class: "w-1/4 font-semibold text-gray-800", "Date of Birth:" }
                    span { class: "w-3/4 text-gray-600", "January 1, 2023" }  // Replace with user's DOB
                }
                // Add more personal information fields here
            }
            section { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-4", "Contact Information" }
                div { class: "flex mb-2",
                    span { class: "w-1/4 font-semibold text-gray-800", "Email:" }
                    span { class: "w-3/4 text-gray-600", "ajibadeabd@gmail.com" }  // Replace with user's email
                }
                div { class: "flex mb-2",
                    span { class: "w-1/4 font-semibold text-gray-800", "Phone:" }
                    span { class: "w-3/4 text-gray-600", "+2348090903620" }  // Replace with user's phone number
                }
            }
        }
    ))
}
