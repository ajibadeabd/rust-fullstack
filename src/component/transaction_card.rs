use dioxus::prelude::*;


#[inline_props]
#[warn(non_snake_case)]
pub fn Transactions(cx:Scope) -> Element {
    let transactions = use_state(cx, || {
        [
            Transaction {
            description: "Deposit from my Card".to_string(),
            created_at: format_date("28 January 2021").to_string(),
            status: "success".to_string(),
            receiver_id: None,
            amount: 100.0,
        },
        Transaction {
            description: "Withdraw to Bank".to_string(),
            created_at: "30 January 2021".to_string(),
            status: "pending".to_string(),
            receiver_id: Some("123456789".to_string()),
            amount: 50.0,
        },
        Transaction {
            description: "Transfer to Bank".to_string(),
            created_at: "30 January 2021".to_string(),
            status: "failed".to_string(),
            receiver_id: Some("1v9f56789".to_string()),
            amount: 50.0,
        },
        Transaction {
            description: "Transfer to Bank".to_string(),
            created_at: "30 January 2021".to_string(),
            status: "failed".to_string(),
            receiver_id: Some("1v9f56789".to_string()),
            amount: 50.0,
        },
        Transaction {
            description: "Transfer to Bank".to_string(),
            created_at: "30 January 2021".to_string(),
            status: "failed".to_string(),
            receiver_id: Some("1v9f56789".to_string()),
            amount: 50.0,
        },
        Transaction {
            description: "Transfer to Bank".to_string(),
            created_at: "30 January 2021".to_string(),
            status: "failed".to_string(),
            receiver_id: Some("1v9f56789".to_string()),
            amount: 50.0,
        },
        Transaction {
            description: "Transfer to Bank".to_string(),
            created_at: "30 January 2021".to_string(),
            status: "failed".to_string(),
            receiver_id: Some("1v9f56789".to_string()),
            amount: 50.0,
        },
        Transaction {
            description: "Transfer to Bank".to_string(),
            created_at: "30 January 2021".to_string(),
            status: "failed".to_string(),
            receiver_id: Some("1v9f56789".to_string()),
            amount: 50.0,
        },
        Transaction {
            description: "Transfer to Bank".to_string(),
            created_at: "30 January 2021".to_string(),
            status: "failed".to_string(),
            receiver_id: Some("1v9f56789".to_string()),
            amount: 50.0,
        },
        ]
    });
    render!(
        div {
            class:"mt-4 pb-4 w-[100%] md:w-[47%] lg:w-[47%] max-h-[50vh] bg-white shadow-md rounded-[25px] overflow-y-auto",

            for transaction  in transactions.iter() {
                div {
                    class:  " mt-4 ml-4 mr-4 bg-gray-100 rounded-[25px] flex md:flex-1 flex-row  items-start justify-between   p-4   w-[cal(100%-8px)]",
                    div {
                        class: "flex flex-col gap-[7px] items-start justify-start ",
                        div {
                            class: "text-base text-bluegray-600",
                            "{transaction.description}"
                        }
                        div {
                            class: "text-[15px] text-bluegray-400",
                             "{transaction.created_at}",
                        }
                    }
                    div {
                        class: "flex flex-col",
                        div {
                            class:  if transaction.status == "success" {
                                    "text-green-600"
                                } else if transaction.receiver_id == None {
                                    "text-red-500"
                                } else {
                                    "text-yellow-500"
                                }
                            ,
                            // size: "txtInterMedium16",
                              if transaction.status == "success" {
                                "Success"
                            } else {
                                "Pending"
                            },
                        }
                        div {
                            class: "ml-[13px] text-base text-red-700",
                             "{transaction.amount}",
                        }
                    }
                }
            }
        }
    )
}

#[derive(Clone)]
struct Transaction {
    description: String,
    created_at: String,
    status: String,
    receiver_id: Option<String>,
    amount: f64,
}

fn format_date(date: &'static str) -> &'static str {
    // Implement your date formatting logic here
    date
}
