use dioxus::prelude::*;
use dioxus_router::prelude::{Link, use_navigator};

use crate::Route; 

#[inline_props]
pub fn HomePage(cx: Scope) -> Element {
    let nav = use_navigator(cx);

    cx.render(rsx!(
        div { class: "min-h-screen bg-gradient-to-r from-blue-400 to-purple-500",
            // div { class: "flex justify-between items-center p-8 bg-white shadow-md",
             div { class: "fixed top-0 left-0 right-0 flex justify-between items-center p-8 bg-white shadow-md",
                img { class: "w-16 h-8", src: "https://assets.paystack.com/assets/img/logos/merchants/_300xAUTO_crop_center-center_none/FilmHouse-Stack-Blue_200304_153641.svg", alt: "PayNow Logo" },
                div { class: "space-x-4",
                    a { class: "text-gray-800 hover:underline", href: "/about", "About Us" },
                    a { class: "text-gray-800 hover:underline", href: "/services", "Services" },
                    a { class: "text-gray-800 hover:underline", href: "/contact", "Contact Us" },
                }
            }
            div { class: "flex flex-col justify-center items-center py-10   sm:py-20 mt-32",
                h1 { class: "text-4xl font-semibold mb-4 text-center text-white", "Welcome to PayNow" },
                p { class: "text-lg text-center text-white", "Empowering Secure and Convenient Payments" }

                button {
                         class: "px-6 py-3 mt-6 bg-blue-600 rounded-lg hover:bg-blue-700 focus:outline-none focus:bg-blue-700", 
                         
                             onclick: move |event| {
                                        nav.push(Route::LoginPage {});
                        },
                        "Sign Up Now"
                 }
            }
            div { class: "bg-gray-100 py-16",
                div { class: "container mx-auto p-8",
                    h2 { class: "text-3xl font-semibold mb-8 text-gray-800", "What We Offer" },
                    div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3   gap-8",
                         div { class: "bg-white p-6 rounded-lg shadow-md",
                            h3 { class: "text-xl font-semibold mb-4 text-gray-800", "Secure Transactions" },
                            p { class: "text-gray-600", "Your payments are safe with our advanced encryption technology." }
                        }
                        div { class: "bg-white p-6 rounded-lg shadow-md",
                            h3 { class: "text-xl font-semibold mb-4 text-gray-800", "Convenient Wallet" },
                            p { class: "text-gray-600", "Access your funds anytime, anywhere with our user-friendly wallet." }
                        }
                        div { class: "bg-white p-6 rounded-lg shadow-md",
                            h3 { class: "text-xl font-semibold mb-4 text-gray-800", "Business Solutions" },
                            p { class: "text-gray-600", "Streamline your business operations with our payment solutions." }
                        }
                        div { class: "bg-white p-6 rounded-lg shadow-md",
                            h3 { class: "text-xl font-semibold mb-4 text-gray-800", "Global Reach" },
                            p { class: "text-gray-600", "Send and receive payments globally with ease." }
                        }
                        div { class: "bg-white p-6 rounded-lg shadow-md",
                            h3 { class: "text-xl font-semibold mb-4 text-gray-800", "Additional Service 1" },
                            p { class: "text-gray-600", "Description of the additional service 1." }
                        }
                        div { class: "bg-white p-6 rounded-lg shadow-md",
                            h3 { class: "text-xl font-semibold mb-4 text-gray-800", "Additional Service 2" },
                            p { class: "text-gray-600", "Description of the additional service 2." }
                        }
                    }
                }
            }
            div { class: "p-16",
                div { class: "container mx-auto",
                    h2 { class: "text-3xl font-semibold mb-8 text-gray-800", "Why Choose PayNow?" },
                    div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3   gap-8",
                        div { class: "bg-white p-6 rounded-lg shadow-md",
                            h3 { class: "text-xl font-semibold mb-4 text-gray-800", "Fast Transactions" },
                            p { class: "text-gray-600", "Experience lightning-fast payment processing." }
                        }
                        div { class: "bg-white p-6 rounded-lg shadow-md",
                            h3 { class: "text-xl font-semibold mb-4 text-gray-800", "24/7 Support" },
                            p { class: "text-gray-600", "Our support team is available around the clock to assist you." }
                        }
                        div { class: "bg-white p-6 rounded-lg shadow-md",
                            h3 { class: "text-xl font-semibold mb-4 text-gray-800", "Easy Integration" },
                            p { class: "text-gray-600", "Seamlessly integrate our payment solutions into your app or website." }
                        }
                    }
                }
            }
            div { class: "bg-blue-800 text-white py-12",
                div { class: "container mx-auto flex flex-col items-center",
                    h2 { class: "text-3xl font-semibold mb-4 text-white", "Ready to Get Started?" },
                    p { class: "text-lg text-white text-center", "Join thousands of businesses and individuals who trust PayNow for their payments." },
                    button {
                         class: "px-6 py-3 mt-6 bg-blue-600 rounded-lg hover:bg-blue-700 focus:outline-none focus:bg-blue-700", 
                         
                             onclick: move |event| {
                                        nav.push(Route::LoginPage {});
                        },
                        "Sign Up Now"
                 }
                 
                }
            }
            div { class: "bg-gray-900 text-white py-8",
                div { class: "container mx-auto flex justify-between items-center",
                    p { class: "text-sm text-gray-500", "© 2023 PayNow. All rights reserved." },
                    div { class: "flex space-x-4",
                        a { class: "text-gray-500 hover:text-white", href: "/privacy", "Privacy Policy" },
                        a { class: "text-gray-500 hover:text-white", href: "/terms", "Terms of Service" },
                    }
                }
            }
        }
    ))
}
