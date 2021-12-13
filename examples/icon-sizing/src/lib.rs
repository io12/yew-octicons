use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_octicons::*;

/// Main app component
pub struct App {
    /// Size of icons
    size: usize,
}

/// App message
pub enum Msg {
    /// Increment icons by amount
    Inc(usize),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App { size: 16 }
    }

    fn changed(&mut self, _: &Context<Self>) -> bool {
        false
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Inc(n) => {
                self.size += n;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Create buttons for incrementing the icon size by certain amounts
        let buttons = (2..8)
            .map(|n| {
                let n = 2_usize.pow(n);
                html! {
                    <button onclick={ ctx.link().callback(move |_| Msg::Inc(n)) }>
                        { n }
                    </button>
                }
            })
            .collect::<Vec<Html>>();

        html! {
            <div>
                <pre style="display: inline"> { "size += " } </pre>
                { buttons }
                <br />
                { Icon::new_sized(IconKind::Rocket, self.size) }
                { Icon::new_sized(IconKind::Alert, self.size) }
                { Icon::new_sized(IconKind::FileBinary, self.size) }
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<App>();
}
