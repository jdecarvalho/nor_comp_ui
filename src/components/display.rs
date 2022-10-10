use yew::{html, Component, Context, Html};

pub enum ComponentEvent {
    OutputValue([[i8;7];4]),
}

pub struct Display {
    // state: NorComputer,
}

impl Component for Display {
    type Message = ComponentEvent;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // let (state, _listener) = ctx
        //     .link()
        //     .context::<NorComputer>(Callback::noop() /*ctx.link().callback(ComputerEvent::ContextChanged)*/)
        //     .expect("context to be set");

        Self {
            // state,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let baseClass = "display-container display-size-12 display-no-8";

        html! {
            <div id="vertical-center">
	            <div id="clock-container">
		            <div id="display-1" class="display-container display-size-12 display-no-0 ${baseClass}">
			            <div class="segment-x segment-a"><span class="segment-border"></span></div>
			            <div class="segment-y segment-b"><span class="segment-border"></span></div>
			            <div class="segment-y segment-c"><span class="segment-border"></span></div>
            			<div class="segment-x segment-d"><span class="segment-border"></span></div>
            			<div class="segment-y segment-e"><span class="segment-border"></span></div>
            			<div class="segment-y segment-f"><span class="segment-border"></span></div>
                        <div class="segment-x segment-g"><span class="segment-border"></span></div>
                    </div>
                    <div id="display-2" class="display-container display-size-12 display-no-1">
			            <div class="segment-x segment-a"><span class="segment-border"></span></div>
			            <div class="segment-y segment-b"><span class="segment-border"></span></div>
			            <div class="segment-y segment-c"><span class="segment-border"></span></div>
                        <div class="segment-x segment-d"><span class="segment-border"></span></div>
			            <div class="segment-y segment-e"><span class="segment-border"></span></div>
			            <div class="segment-y segment-f"><span class="segment-border"></span></div>
			            <div class="segment-x segment-g"><span class="segment-border"></span></div>
                    </div>
		            <div id="display-3" class="display-container display-size-12 display-no-2">
                        <div class="segment-x segment-a"><span class="segment-border"></span></div>
            			<div class="segment-y segment-b"><span class="segment-border"></span></div>
            			<div class="segment-y segment-c"><span class="segment-border"></span></div>
			            <div class="segment-x segment-d"><span class="segment-border"></span></div>
            			<div class="segment-y segment-e"><span class="segment-border"></span></div>
            			<div class="segment-y segment-f"><span class="segment-border"></span></div>
            			<div class="segment-x segment-g"><span class="segment-border"></span></div>
                    </div>
	            	<div id="display-4" class="display-container display-size-12 display-no-3">
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
            ComponentEvent::OutputValue(values) => {
                log::info!("OUTPUTTING!! ");

                false
            }
        }
    }
}
