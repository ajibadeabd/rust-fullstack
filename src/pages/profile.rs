 use dioxus::prelude::*;
use crate::component::{
    side_bar::Sidebar,
    nav_bar::Navbar
};

#[inline_props]
pub fn ProfilePage(cx: Scope) -> Element {
    let user_data = use_state(cx, || {
        UserData {
            user_name: "John Doe".to_string(),
            user_role: "Admin".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            date_of_birth: "January 1, 1990".to_string(),
            email: "johndoe@example.com".to_string(),
            phone: "+1234567890".to_string(),
            address: "123 Main St, City".to_string(),
        }
    });

    render!(rsx!(
                            div { class: "min-h-screen bg-gradient-to-r from-blue-400 to-purple-500",
            Navbar{}
            Sidebar{}

                    div { class: "flex justify-end ",

                div {
                    class: "flex flex-col  justify-start p-6 py-20 w-full lg:w-[75%]",
                    
            h2 { class: "text-2xl font-semibold mb-4", "Profile" }
            div { class: "flex items-center mb-4",
                div { class: "w-16 h-16 bg-gray-300 rounded-full" }
                div { class: "ml-4",
                    h3 { class: "text-xl font-semibold", "{user_data.user_name}" }
                    p { class: "text-gray-600", "{user_data.user_role}" }
                }
            }
            section { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-4", "Personal Information" }
                div { class: "flex mb-2",
                    span { class: "w-1/4 font-semibold text-gray-800", "First Name:" }
                    span { class: "w-3/4 text-gray-600", "{user_data.first_name}" }
                }
                div { class: "flex mb-2",
                    span { class: "w-1/4 font-semibold text-gray-800", "Last Name:" }
                    span { class: "w-3/4 text-gray-600", "{user_data.last_name}" }
                }
                div { class: "flex mb-2",
                    span { class: "w-1/4 font-semibold text-gray-800", "Date of Birth:" }
                    span { class: "w-3/4 text-gray-600", "{user_data.date_of_birth }"}
                }
                // Add more personal information fields here
            }
            section { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-4", "Contact Information" }
                div { class: "flex mb-2",
                    span { class: "w-1/4 font-semibold text-gray-800", "Email:" }
                    span { class: "w-3/4 text-gray-600", "{user_data.email}" }
                }
                div { class: "flex mb-2",
                    span { class: "w-1/4 font-semibold text-gray-800", "Phone:" }
                    span { class: "w-3/4 text-gray-600", "{user_data.phone}" }
                }
                div { class: "flex mb-2",
                    span { class: "w-1/4 font-semibold text-gray-800", "Address:" }
                    span { class: "w-3/4 text-gray-600", "{user_data.address}" }
                }
                // Add more contact information fields here
            }
              section { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-4", "Education and Work" }
                // Add fields for education and work history
            }
            section { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-4", "Interests and Hobbies" }
                // Add fields for interests and hobbies
            }
            section { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-4", "Privacy Settings" }
                // Add privacy settings options
            }
            section { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-4", "Activity Feed" }
                // Display recent activities or updates
            }
            section { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-4", "Security Settings" }
                // Allow users to update passwords and security settings
            }
            section { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-4", "Notification Preferences" }
                // Allow users to customize notification preferences
            }
            section { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-4", "Connections" }
                // Display user's connections or network
            }
            section { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-4", "Badges and Achievements" }
                // Display badges, achievements, or awards
            }
            section { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-4", "Profile Customization" }
                // Allow users to customize profile appearance
            }
            section { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-4", "Feedback and Reviews" }
                // Display user feedback and reviews
            }
            section { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-4", "Support and Help" }
                // Provide support and help resources
            }
            // Add more sections as needed
        }
        }
        }
    ))
}

struct UserData {
    user_name: String,
    user_role: String,
    first_name: String,
    last_name: String,
    date_of_birth: String,
    email: String,
    phone: String,
    address: String,
}
