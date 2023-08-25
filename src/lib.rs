use dioxus_router::prelude::*;
use dioxus::prelude::*;

mod pages;
mod component;
mod context;


use context::{
    context
};
use pages::{
    home::HomePage,
    login::LoginPage,
    not_found::NotFound,
    dash_board::DashboardPage,
    profile::ProfilePage,
    transaction::DepositPage
};
 
#[derive(Routable, PartialEq, Debug, Clone)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    HomePage{},
    #[route("/login")]
    LoginPage{},
    #[route("/dashboard")]
    DashboardPage{},
    #[route("/profile")]
    ProfilePage{},
    #[route("/transactions")]
    DepositPage{},
    #[route("/:..route")]
	NotFound { route: Vec<String> }
}



// Provider
fn Parent<'a>(cx: Scope<'a>) -> Element<'a> {
    //initialize shared data
    context(cx);
    render! {
        Router::<Route> { }
    }
}

pub fn app(cx: Scope) -> Element {
    return Parent(cx);
}
