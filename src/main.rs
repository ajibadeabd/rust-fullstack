#![allow(non_snake_case, unused)]

use rust_fullstack::app;
use dioxus::prelude::*;

// fn main() {
//     LaunchBuilder::new(app).launch();
// }



fn main() {
    // launch the web app
    dioxus_web::launch(app);
}

 



// fn app(cx: Scope) -> Element {
//     let mut count = use_state(cx, || 0);

//     cx.render(rsx! {
//         h1 { "High-Five counter: {count}" }
//         button { onclick: move |_| count += 1, "Up high!" }
//         button { onclick: move |_| count -= 1, "Down low!" }
//     })
// }

