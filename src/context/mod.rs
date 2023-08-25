use dioxus::prelude::{use_shared_state_provider, Scope};

#[derive(Clone, Copy)]
pub enum SideBarOpen {
    True,
    False,
}

pub fn context (cx:Scope){
    return use_shared_state_provider(cx, || SideBarOpen::True );

}