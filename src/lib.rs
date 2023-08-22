use dioxus_router::prelude::*;
use dioxus::prelude::*;

mod pages;
mod component;


use pages::{
    home::HomePage,
    login::LoginPage,
    not_found::NotFound,
    dash_board::DashboardPage,
    profile::ProfilePage
};
 
#[derive(Routable, PartialEq, Debug, Clone)]
enum Route {
    #[route("/")]
    HomePage{},
    #[route("/login")]
    LoginPage{},
    #[route("/dashboard")]
    DashboardPage{},
    #[route("/profile")]
    ProfilePage{},
    #[route("/:..segments")]
	NotFound { segments: Vec<String> }
}


pub fn app(cx: Scope) -> Element {
        render! {
            Router::<Route> { }
        }
    
}
