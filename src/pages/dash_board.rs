use dioxus::prelude::*;
use crate::component::{
    side_bar::Sidebar,
    nav_bar::Navbar, card::Card, transaction_card::Transactions
};


use charming::{
    component::Legend,
    element::{ItemStyle, AxisType},
    series::{Pie, PieRoseType, Line},
    Chart,  WasmRenderer
};
use charming::component::Axis;

#[inline_props]
pub fn DashboardPage(cx: Scope) -> Element {
    // Sample card data
    let card_data: Vec<(&str, &str)> = vec![
        ("Total ", "$1000"),
        ("Revenue", "$5000"),
        ("Profit", "$4000"),
        ("Profit", "$4000"),
        // Add more card data here
    ];
let renderer: WasmRenderer = WasmRenderer::new(600, 400);
    use_future!(cx, || async move {
      let chart = Chart::new()
      .x_axis(
          Axis::new()
              .type_(AxisType::Category)
              .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
      )
      .y_axis(Axis::new().type_(AxisType::Value))
      .series(Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]));
    // let sss = renderer.render("chart",&chart).unwrap();
    // let mut renderer = ImageRenderer::new(1000, 800);
    // renderer.save(&chart, "/tmp/nightingale.svg");

    });

let chart = Chart::new()
    .x_axis(Axis::new().data(vec!["Jan", "Feb", "Mar", "Apr"]))
    .y_axis(Axis::new());
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
                 div{
                    class:"flex flex-wrap justify-around",
                    Card { }
                    Transactions{ }


                    div {
                        // renderer.render("chart",&chart).unwrap()
                    }


                    //  div{
                    //     // class: "w-full lg:-m-2",
                    //     style: "width: calc(100% - 2px);",
                    //     Card { }
                    // }
                    //  div{
                    //     // class: "w-full lg:-m-2",
                    //     style: "width: calc(100% - 2px);",
                    //     // Transactions{}
                    //     Card { }

                    // }
                 

                //  Card { }
                 }

                //  Transactions{}
                }
        }
        }
    ))
}
