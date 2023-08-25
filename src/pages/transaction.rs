use dioxus::prelude::*;

use crate::component::{side_bar::Sidebar, nav_bar::Navbar};

#[inline_props]
pub fn DepositPage(cx: Scope) -> Element {

    // // Define breadcrumb items based on current view
    // let breadcrumb_items = if "current_view" != "form" {
    //     vec![("Transaction", "/"), ("Deposit", "#")]
    // } else {
    //     vec![("Transaction", "/"), ("Transfer ", "#")]
    // };
    let actions = vec!["Deposit".to_owned(), "Transfer".to_owned(),"Withdraw".to_owned()];
    let first_action = use_state(cx, || 0 as i32);
let mut count = use_state(cx, || 0);

    cx.render(rsx!(
        div { class: "min-h-screen bg-gradient-to-r from-blue-400 to-purple-500",
            // Navigation Bar
            Navbar {}
            Sidebar {}

            div { class: "flex justify-end",
                div {
                    class: "flex flex-col  justify-start p-6 py-20 w-full lg:w-[75%]",
                    // Deposit Section

div { class: "p-4 bg-gray-200 text-gray-600",
                div { class: "container  ",
                  
                     div {
        class: "flex space-x-4",
        for (index, action) in actions.iter().enumerate() {
            span {
                onclick: move |_| {
                    println!("{}",index);
                    first_action.set(index as i32);
                    
                },
                 
                    class: if first_action.get() == &(index as i32) {
                "text-gray-800 font-semibold cursor-pointer" // Apply different style for the active tab
            } else {
                "text-gray-600 cursor-pointer"
            },
                "{action}"
                if index < actions.len() - 1 {
                    ("       >>>")
                },
                
            }
        }
    }
                }
            },

                    
                     
                    div { class: "  w-full md:w-[50%] lg:w-[50%] p-8 mt-16 bg-white rounded-lg shadow-md ",
                        h2 { class: "text-3xl font-semibold mb-4 text-gray-800", 
                         if first_action.get()  == &0 as &i32 {
        "Deposit into Wallet"
    } else if first_action.get()  == &1 as &i32  {
        "Transfer to a friend"
    } else  {
        "Withdraw"
    }
                        
                        
                        
                        
                         },
                        form { class: "space-y-4",
                            div { class: "flex flex-col",
                                label { class: "text-sm font-medium mb-1", "Amount" },
                                input { class: "px-3 py-2 border rounded-lg focus:outline-none focus:border-blue-500", "type": "number", "placeholder": "Enter amount to deposit" }
                            }
                            div { class: "flex flex-col",
                                label { class: "text-sm font-medium mb-1",
                                "Currency" },
                                select { class: "px-3 py-2 border rounded-lg focus:outline-none focus:border-blue-500",
                                    
                                    option { "NGN" }
                                }
                            }
                            if first_action.get()  == &2 as &i32 {
                           rsx!( div { class: "flex flex-col",
                                label { class: "text-sm font-medium mb-1",
                                "Account Number" },
                                input { class: "px-3 py-2 border rounded-lg focus:outline-none focus:border-blue-500", "type": "number", "placeholder": "Enter Account Number" }

                            })
                        }
                        if first_action.get()  == &1 as &i32 {
                           rsx!( div { class: "flex flex-col",
                                label { class: "text-sm font-medium mb-1",
                                "Email" },
                              input { class: "px-3 py-2 border rounded-lg focus:outline-none focus:border-blue-500", "type": "number", "placeholder": "Enter user email" }

                            })
                        }





                            button { class: "px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 focus:outline-none focus:bg-blue-600", "Deposit" }
                        }
                    }
                    // Transaction History Section
                    div { class: "    p-8 mt-8 bg-white rounded-lg shadow-md",
                        h2 { class: "text-3xl font-semibold mb-4 text-gray-800", 
                        
                        
                         if first_action.get()  == &0 as &i32 {
        "Deposit History"
    } else if first_action.get()  == &1 as &i32  {
        "Transfer History"
    } else  {
        "Withdrawal History"
    }
                        // "Transaction History" 
                    },
                        table { class: "w-full border-collapse",
                            thead { class: "bg-gray-100",
                                tr {
                                    th { class: "p-3 font-semibold text-left text-gray-800", "Date" }
                                    th { class: "p-3 font-semibold text-left text-gray-800", "Amount" }
                                    th { class: "p-3 font-semibold text-left text-gray-800", 
                                    
                                    
                                    
                                    
                                    
                                    "Payment Method" }
                                }
                            }
                            tbody { class: "divide-y divide-gray-200",
                                // Loop through user's transactions and display them
                                for i in 1..=10 {
                                    tr { class: "hover:bg-gray-100",
                                        td { class: "p-3 text-gray-600", "August 21, 2023" }
                                        td { class: "p-3 text-gray-800 font-semibold", "$100" }
                                        td { class: "p-3 text-gray-800 font-semibold", "Credit Card" }
                                    }
                                }
                            }
                        }
                    }
                    // Footer
                    div { class: "fixed bottom-0 left-0 right-0 bg-gray-900 text-white py-4",
                        div { class: "container mx-auto flex justify-between items-center",
                            p { class: "text-sm text-gray-500", "© 2023 PayNow. All rights reserved." },
                            div { class: "flex space-x-4",
                                a { class: "text-gray-500 hover:text-white", href: "/privacy", "Privacy Policy" },
                                a { class: "text-gray-500 hover:text-white", href: "/terms", "Terms of Service" },
                            }
                        }
                    }
                }
            }
        }
    ))
}
