use yew::{classes, html, Component, Context, Html};
use yew_agent::{Bridge, Bridged};

use crate::services::{events::{InboundMessage, InboundMessageType, InboundMessageEvent}, event_bus::EventBus};

pub struct Display {
    value: [[u8;7];4],
    _producer: Box<dyn Bridge<EventBus>>
}

impl Component for Display {
    type Message = InboundMessageEvent;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            value: [
                [0,0,0,0,0,0,0],
                [0,0,0,0,0,0,0],
                [0,0,0,0,0,0,0],
                [0,0,0,0,0,0,0],
            ],
            _producer: EventBus::bridge(ctx.link().callback(InboundMessageEvent::InboundMessage)),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let value_1 = decode_value(self.value[0]);
        let value_2 = decode_value(self.value[1]);
        let value_3 = decode_value(self.value[2]);
        let value_4 = decode_value(self.value[3]);

        // log::warn!("DECODE: {:?}", std::str::from_utf8(&self.value[0]));

        let css_class_1 = format!("display-container display-size-12 display-no-{}", value_1);
        let css_class_2 = format!("display-container display-size-12 display-no-{}", value_2);
        let css_class_3 = format!("display-container display-size-12 display-no-{}", value_3);
        let css_class_4 = format!("display-container display-size-12 display-no-{}", value_4);

        html! {
            <div id="vertical-center">
	            <div id="clock-container">
		            <div id="display-1" class={classes!(css_class_1)}>
			            <div class="segment-x segment-a"><span class="segment-border"></span></div>
			            <div class="segment-y segment-b"><span class="segment-border"></span></div>
			            <div class="segment-y segment-c"><span class="segment-border"></span></div>
            			<div class="segment-x segment-d"><span class="segment-border"></span></div>
            			<div class="segment-y segment-e"><span class="segment-border"></span></div>
            			<div class="segment-y segment-f"><span class="segment-border"></span></div>
                        <div class="segment-x segment-g"><span class="segment-border"></span></div>
                    </div>
                    <div id="display-2" class={classes!(css_class_2)}>
			            <div class="segment-x segment-a"><span class="segment-border"></span></div>
			            <div class="segment-y segment-b"><span class="segment-border"></span></div>
			            <div class="segment-y segment-c"><span class="segment-border"></span></div>
                        <div class="segment-x segment-d"><span class="segment-border"></span></div>
			            <div class="segment-y segment-e"><span class="segment-border"></span></div>
			            <div class="segment-y segment-f"><span class="segment-border"></span></div>
			            <div class="segment-x segment-g"><span class="segment-border"></span></div>
                    </div>
		            <div id="display-3" class={classes!(css_class_3)}>
                        <div class="segment-x segment-a"><span class="segment-border"></span></div>
            			<div class="segment-y segment-b"><span class="segment-border"></span></div>
            			<div class="segment-y segment-c"><span class="segment-border"></span></div>
			            <div class="segment-x segment-d"><span class="segment-border"></span></div>
            			<div class="segment-y segment-e"><span class="segment-border"></span></div>
            			<div class="segment-y segment-f"><span class="segment-border"></span></div>
            			<div class="segment-x segment-g"><span class="segment-border"></span></div>
                    </div>
	            	<div id="display-4" class={classes!(css_class_4)}>
                        <div class="segment-x segment-a"><span class="segment-border"></span></div>
			            <div class="segment-y segment-b"><span class="segment-border"></span></div>
			            <div class="segment-y segment-c"><span class="segment-border"></span></div>
                        <div class="segment-x segment-d"><span class="segment-border"></span></div>
            			<div class="segment-y segment-e"><span class="segment-border"></span></div>
            			<div class="segment-y segment-f"><span class="segment-border"></span></div>
            			<div class="segment-x segment-g"><span class="segment-border"></span></div>
                    </div>
                </div>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // handle inbound messages from the websocket server
            InboundMessageEvent::InboundMessage(msg) => {
                let msg: InboundMessage = serde_json::from_str(&msg).unwrap();
                match msg.message_type {
                    InboundMessageType::OutputValue => {
                            let msg_data = msg.data.unwrap_or_default();

                            let msg_data: [[u8;7];4] = serde_json::from_str(&msg_data).unwrap();
                            log::info!("Received InboundMessageType::OutputValue: {:?}", &msg_data);

                            self.value = msg_data.clone();

                            return true;
                    },
                    _ => false
                }
            }
        }
    }
}

fn decode_value(bits: [u8;7]) -> String {
    let mut result = "x";

    if bits[0] == 1 && bits[1] == 1 && bits[2] == 1 && bits[3] == 1 && bits[4] == 1 && bits[5] == 1 && bits[6] == 0 {
        result = "0";
    } else if bits[0] == 0 && bits[1] == 1 && bits[2] == 1 && bits[3] == 0 && bits[4] == 0 && bits[5] == 0 && bits[6] == 0 {
        result = "1";
    } else if bits[0] == 1 && bits[1] == 1 && bits[2] == 0 && bits[3] == 1 && bits[4] == 1 && bits[5] == 0 && bits[6] == 1 {
        result = "2";
    } else if bits[0] == 1 && bits[1] == 1 && bits[2] == 1 && bits[3] == 1 && bits[4] == 0 && bits[5] == 0 && bits[6] == 1 {
        result = "3";
    } else if bits[0] == 0 && bits[1] == 1 && bits[2] == 1 && bits[3] == 0 && bits[4] == 0 && bits[5] == 1 && bits[6] == 1 {
        result = "4";
    } else if bits[0] == 1 && bits[1] == 0 && bits[2] == 1 && bits[3] == 1 && bits[4] == 0 && bits[5] == 1 && bits[6] == 1 {
        result = "5";
    } else if bits[0] == 1 && bits[1] == 0 && bits[2] == 1 && bits[3] == 1 && bits[4] == 1 && bits[5] == 1 && bits[6] == 1 {
        result = "6";
    } else if bits[0] == 1 && bits[1] == 1 && bits[2] == 1 && bits[3] == 0 && bits[4] == 0 && bits[5] == 0 && bits[6] == 0 {
        result = "7";
    } else if bits[0] == 1 && bits[1] == 1 && bits[2] == 1 && bits[3] == 1 && bits[4] == 1 && bits[5] == 1 && bits[6] == 1 {
        result = "8";
    } else if bits[0] == 1 && bits[1] == 1 && bits[2] == 1 && bits[3] == 1 && bits[4] == 0 && bits[5] == 1 && bits[6] == 1 {
        result = "9";
    }

    // log::debug!("Decoded {:?} into {}", &bits, &result);

    String::from(result)
}
