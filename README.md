[![](https://img.shields.io/crates/v/yew-octicons)](https://crates.io/crates/yew-octicons)
[![](https://docs.rs/yew-octicons/badge.svg)](https://docs.rs/yew-octicons)

# `yew-octicons`

An easy interface for using [Octicons](https://primer.style/octicons/) in [yew](https://yew.rs) projects.

## Example

```rust
use yew::html;
use yew_octicons::Icon;
use yew_octicons::IconKind;

let code = html! {
    <span>
        { Icon::new(IconKind::Alert) }
    </span>
};
```

For a more complex example, see [examples/icon-sizing](examples/icon-sizing).
