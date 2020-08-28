use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_octicons::*;

/// Main app component
pub struct App {
    link: ComponentLink<Self>,
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

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { link, size: 16 }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Inc(n) => {
                self.size += n;
            }
        }
        true
    }

    fn view(&self) -> Html {
        // Create buttons for incrementing the icon size by certain amounts
        let buttons = (2..8)
            .map(|n| {
                let n = 2_usize.pow(n);
                html! {
                    <button onclick=self.link.callback(move |_| Msg::Inc(n))>
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
    yew::start_app::<App>()
}
