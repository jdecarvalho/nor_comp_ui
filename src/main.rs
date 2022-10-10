#![recursion_limit = "512"]

mod components;
mod services;
use components::panel::Panel;

use services::websocket::WebsocketService;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Panel,

    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(selected_route: &Route) -> Html {
    match selected_route {
        Route::Panel => html! {<Panel />},
        Route::NotFound => html! {<h1>{"404 baby"}</h1>},
    }
}

#[function_component(Main)]
fn main_component() -> Html {
    let ctx = use_state(|| {
        Rc::new(NorComputerInner {
            register_a: RefCell::new("register-a".into()),
            wss: WebsocketService::new(),
        })
    });

    html! {
        <ContextProvider<NorComputer> context={(*ctx).clone()}>
            <BrowserRouter>
                <div class="flex w-screen h-screen">
                    <Switch<Route> render={Switch::render(switch)}/>
                </div>
            </BrowserRouter>
        </ContextProvider<NorComputer>>
    }
}

pub type NorComputer = Rc<NorComputerInner>;

#[derive(PartialEq)]
pub struct NorComputerInner {
    pub register_a: RefCell<String>,
    pub wss: WebsocketService,
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
}
