use dioxus::prelude::*;
use crate::component::{
    side_bar::Sidebar,
    nav_bar::Navbar, card::Card
};

#[inline_props]
pub fn DashboardPage(cx: Scope) -> Element {
    // Sample card data
    let card_data = vec![
        ("Total ", "$1000"),
        ("Revenue", "$5000"),
        ("Profit", "$4000"),
        ("Profit", "$4000"),
        // Add more card data here
    ];

    cx.render(rsx!(
        div { class: "min-h-screen bg-gray-100", "dash"
        Navbar{}
            
            div {
                class: "flex  justify-end",
                    Sidebar{}
                div { class: " p-6 mt-20  w-full lg:w-[75%]",  // Adjust margins and padding as needed
                 div {
                    class:"flex   space-x-4   flex-row",
                    for (title, value) in card_data {
                        div {
                            class:"sm:w-1/2 md:w-1/3 lg:w-1/4 bg-white shadow-md p-4 rounded-lg w-1/4  ",
                            h2 {
                                class:"text-lg font-semibold mb-2",
                                title
                            }
                            p {
                                class:"text-xl",
                                value
                            }
                        }
                    }
                 }
                 Card { }
                }
        }
        }
    ))
}
