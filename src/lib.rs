#![warn(missing_docs)]

//! `yew_octicons` is an easy interface for using
//! [Octicons](https://primer.style/octicons/) in [yew](https://yew.rs)
//! projects.
//!
//! ## Example
//!
//! ```
//! use yew::html;
//! use yew_octicons::Icon;
//! use yew_octicons::IconKind;
//!
//! let code = html! {
//!     <span>
//!         { Icon::new(IconKind::Alert) }
//!     </span>
//! };
//! ```

mod generated;

pub use generated::IconKind;

use yew::html;
use yew::html::Html;

/// An Octicon with size information. This can be used in yew's `html!` macro.
///
/// ```
/// use yew::html;
/// use yew_octicons::Icon;
/// use yew_octicons::IconKind;
///
/// let code = html! {
///     <span>
///         { Icon::new(IconKind::Alert) }
///     </span>
/// };
/// ```
#[derive(PartialEq, PartialOrd, Clone, Copy, Hash, Debug)]
pub struct Icon {
    kind: IconKind,
    size: usize,
}

impl Icon {
    /// Create a default-sized 16px icon
    ///
    /// ```
    /// use yew::html;
    /// use yew_octicons::Icon;
    /// use yew_octicons::IconKind;
    ///
    /// let code = html! {
    ///     <span>
    ///         { Icon::new(IconKind::Alert) }
    ///     </span>
    /// };
    /// ```
    pub fn new(kind: IconKind) -> Self {
        Self::new_sized(kind, 16)
    }

    /// Create a big 24px icon
    ///
    /// ```
    /// use yew::html;
    /// use yew_octicons::Icon;
    /// use yew_octicons::IconKind;
    ///
    /// let code = html! {
    ///     <span>
    ///         { Icon::new_big(IconKind::Alert) }
    ///     </span>
    /// };
    /// ```
    pub fn new_big(kind: IconKind) -> Self {
        Self::new_sized(kind, 24)
    }

    /// Create an icon of an arbitrary size. The `size` is the side length of
    /// the icon in pixels.
    ///
    /// ```
    /// use yew::html;
    /// use yew_octicons::Icon;
    /// use yew_octicons::IconKind;
    ///
    /// let code = html! {
    ///     <span>
    ///         { Icon::new_sized(IconKind::Alert, 50) }
    ///     </span>
    /// };
    /// ```
    pub fn new_sized(kind: IconKind, size: usize) -> Self {
        Self { kind, size }
    }
}

impl From<Icon> for Html {
    /// Convert icon to yew HTML. This allows icons to be used in yew's `html!`
    /// macro.
    fn from(icon: Icon) -> Self {
        let big = icon.size >= 24;
        let (path, big) = icon.kind.path(big);
        // The `viewBox` size should be the actual size of the icon, not the
        // requested size.
        let viewbox_size = if big { 24 } else { 16 };
        html! {
            <svg
                width=icon.size.to_string()
                height=icon.size.to_string()
                viewBox=format!("0 0 {0} {0}", viewbox_size)
                fill="currentColor">

                <path fill-rule="evenodd" d=path />
            </svg>
        }
    }
}
