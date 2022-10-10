use yew::{function_component, html};

use crate::components::display::Display;
use crate::components::keyboard::Keyboard;
use crate::components::register::Register;

#[function_component(Panel)]
pub fn main_component() -> Html {
    html! {
        <table class="table-auto">
            <tbody>
                <tr>
                    <Register name="A" />
                </tr>
                <tr>
                    <Register name="B" />
                </tr>
                <tr>
                    <Display />
                </tr>
                <tr>
                    <Keyboard />
                </tr>
            </tbody>
        </table>
    }
}
