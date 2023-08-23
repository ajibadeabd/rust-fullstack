use std::clone;

use dioxus::prelude::*;
use dioxus_router::prelude::Link;

// #[inline_props]
#[warn(non_snake_case)]
pub fn Card(cx: Scope) -> Element {

#[derive(Clone, Debug)]
struct AccountType{
    balance:f64,
    id:String
}

    let accounts = vec![
      AccountType{
        balance:100.00,
        id: "3025985702".to_owned(),
    }
    ];
    render!(
        div {

            class:"w-[45%] lg:w-[30%] mt-4",
            // for  (_i , account) in accounts.iter().enumerate() {
                div {
                    class:  "w-[25%] bg-indigo-500 flex md:flex-1 flex-col gap-[33px] items-center justify-end pt-6 rounded-[25px] w-[48%] md:w-full",
                    div {
                        class: "flex flex-col gap-[29px] items-start justify-start w-[87%] md:w-full",
                        div {
                            class: "flex flex-row items-start justify-between md:px-5 w-full",
                            div {
                                class: "flex flex-col items-start justify-start",
                                div {
                                    class: "text-white-A700 text-xs",
                                     "Balance"
                                }
                                div {
                                    class: "mt-1 text-white-A00 text-xl",
                                                  "₦ 100.00" 
                                }
                                div {
                        class:"  text-left text-gray-200  text-lg ",
                        "Abdullah Ajibade",
                           }
                            } 
                        }
                    }
                    
                    div {
                        class: " bg-white  leading-[normal] md:text-xl p-0 placeholder:text-white-A700 sm:pl-5 sm:text-lg text-[22px] text-left text-white-A700 bg-gradient  flex pl-6 py-[21px] rounded-bl-[25px] rounded-br-[25px] w-full",
                        "3025985702",
                    }
            }
        }
    )
}
