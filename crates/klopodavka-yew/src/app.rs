use klopodavka_lib::{ai, game};
use yew::prelude::*;

pub struct App {
    clicked: bool,
    onclick: Callback<ClickEvent>,
}
pub enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            clicked: false,
            onclick: link.callback(|_| Msg::Click),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = true;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.clicked {
            "Clicked!"
        } else {
            "Click me!"
        };

        html! {
            <div>
                <h1>{ "Hello Klopodavka" }</h1>
                <button onclick=&self.onclick>{ button_text }</button>
            </div>
        }
    }
}
