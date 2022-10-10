use web_sys::MouseEvent;
use yew::{html, Callback, Component, Context, Html};

use crate::{NorComputer, services::{events::{OutboundMessage, OutboundMessageType}}};

pub enum ComponentEvent {
    ComputerReset,
}

pub struct Keyboard {
    state: NorComputer,
}

impl Component for Keyboard {
    type Message = ComponentEvent;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (state, _listener) = ctx
            .link()
            .context::<NorComputer>(Callback::noop() /*ctx.link().callback(ComputerEvent::ContextChanged)*/)
            .expect("context to be set");

        Self {
            state,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|evt: MouseEvent| {
            evt.prevent_default();

            ComponentEvent::ComputerReset
        });

        html! {
            <div class="bg-gray-800 flex w-screen">
                <div class="container mx-auto flex flex-col justify-center items-center	">
                    <form class="m-4 flex">
                        <button onclick={submit} class="px-8 rounded-lg bg-red-600 text-white font-bold p-4 uppercase border-red-600 border" >
                            {"RESET Computer!"}
                        </button>
                    </form>
                </div>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // // the web-app's context has changed
            // ComputerEvent::ContextChanged(state) => {
            //     log::info!("Received ComputerEvent::ContextChanged");

            //     self.state = state;
            //     true
            // }

            // handle the user issuing a computer reset
            ComponentEvent::ComputerReset => {
                let msg = OutboundMessage {
                    message_type: OutboundMessageType::ComputerReset,
                    data: Option::None
                };

                if let Err(e) = self.state.wss.tx.clone().try_send(msg) {
                    log::error!("Error sending OutboundWebSocketMessage to channel: {:?}", e);
                }

                false
            },
        }
    }
}
