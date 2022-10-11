use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::{EventBus};
use crate::services::events::{InboundMessage, InboundMessageType, InboundMessageEvent};

// pub enum ComponentEvent {
//     InboundMessage(String),
// }

#[derive(Properties, PartialEq)]
pub struct RegisterProps {
    pub name: AttrValue,
}

pub struct Register {
    register_a: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>
}
impl Component for Register {
    type Message = InboundMessageEvent;
    type Properties = RegisterProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            register_a: NodeRef::default(),
            _producer: EventBus::bridge(ctx.link().callback(InboundMessageEvent::InboundMessage)),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let RegisterProps {
            name,
        } = &ctx.props();

        // let details = Callback::from(|evt: MouseEvent| {
        //     log::warn!("OVER!!");
        // });

        html! {
            <div class="bg-gray-800 flex w-screen">
                <div class="container mx-auto flex flex-col justify-center items-center	">
                    <form class="m-4 flex">
                        <input ref={self.register_a.clone()} /*onmouseover={details}*/ type="text" name="register_a" placeholder={name.clone()} readonly={true} class="rounded-lg p-4 border-t mr-0 border-b border-l text-gray-800 border-gray-200 bg-white" />
                    </form>
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // handle inbound messages from the websocket server
            InboundMessageEvent::InboundMessage(msg) => {
                let msg: InboundMessage = serde_json::from_str(&msg).unwrap();
                match msg.message_type {
                    InboundMessageType::RegisterModified{ register_name } => {
                        if ctx.props().name.eq_ignore_ascii_case(&register_name) {
                            let msg_data = msg.data.unwrap_or_default();

                            log::info!("Received InboundMessageType::RegisterModified[{}]: {}", register_name, &msg_data);

                            let input = self.register_a.cast::<HtmlInputElement>();
                            if let Some(input) = input {
                                input.set_value(&msg_data);
                            }

                            return true;
                        } else {
                            return false;
                        }
                    },
                    _ => false
                }
            },
        }
    }
}
