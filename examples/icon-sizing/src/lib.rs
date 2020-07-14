use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_octicons::*;

pub struct App {
    link: ComponentLink<Self>,
    size: usize,
}

pub enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { link, size: 16 }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.size += 4;
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::Click)>
                    { "size += 4" }
                </button>

                <br />

                { Icon::new_sized(IconKind::Rocket, self.size) }
            </div>
        }
    }
}

#[wasm_bindgen]
pub fn run_app() {
    yew::start_app::<App>()
}
