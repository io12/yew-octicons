mod generated;

pub use generated::IconKind;

use yew::html;
use yew::html::Html;

pub struct Icon {
    kind: IconKind,
    size: usize,
}

impl Icon {
    pub fn new(kind: IconKind) -> Self {
        Self::new_sized(kind, 16)
    }

    pub fn new_big(kind: IconKind) -> Self {
        Self::new_sized(kind, 24)
    }

    pub fn new_sized(kind: IconKind, size: usize) -> Self {
        Self { kind, size }
    }
}

impl From<Icon> for Html {
    fn from(icon: Icon) -> Self {
        let big = icon.size >= 24;
        let path = icon.kind.path(big);
        html! {
            <svg
                width=icon.size
                height=icon.size
                viewBox=format!("0 0 {0} {0}", icon.size)
                fill="currentColor">

                <path fill-rule="evenodd" d=path />
            </svg>
        }
    }
}
