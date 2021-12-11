use std::collections::BTreeSet;
use std::path::Path;

use heck::ToKebabCase;
use heck::ToPascalCase;
use quote::format_ident;
use quote::quote;

/// Get the size of an icon in pixels
///
/// ## Parameters:
///   * `big` - whether the icon is the big variant
fn icon_size(big: bool) -> usize {
    if big {
        24
    } else {
        16
    }
}

/// Given an icon kind and whether or not to use the big version, get a string
/// with the `d` attribute on the `<path>` tag of the icon's SVG. If an icon
/// does not exist for the requested size, fall back to the other size. The
/// returned `bool` indicates whether the big version was used.
fn path_from_icon(kind: &str, big: bool) -> (String, bool) {
    let file_path = format!(
        "octicons/icons/{}-{}.svg",
        kind.to_kebab_case(),
        icon_size(big)
    );
    println!("{}", file_path);
    let file_content = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        // The icon doesn't exist for this size, so fall back to the other size.
        // Hopefully this doesn't loop forever.
        Err(_) => return path_from_icon(kind, !big),
    };
    let path = scraper::Html::parse_fragment(&file_content)
        .select(&scraper::Selector::parse("path").unwrap())
        .map(|path| path.value().attr("d").unwrap().to_string())
        .collect::<Vec<String>>()
        .join(" ");
    (path, big)
}

fn main() {
    // Sorted vector of strings containing CamelCased Octicon names
    let icon_kinds_camel_case = std::fs::read_dir("octicons/icons")
        .unwrap()
        .map(|dir_entry| {
            // Convert each directory entry to CamelCased Octicon name. Each
            // file name should have the format `icon-name-12.svg`, so splitting
            // at the last hyphen is enough to get the icon name by itself.
            dir_entry
                .unwrap()
                .file_name()
                .into_string()
                .unwrap()
                .rsplitn(2, '-')
                .nth(1)
                .unwrap()
                .to_pascal_case()
        })
        // Converting to `BTreeSet`, then `Vec`, automatically sorts and removes
        // duplicates.
        .collect::<BTreeSet<String>>()
        .into_iter()
        .collect::<Vec<String>>();

    // Iterator over the variants of `IconKind`
    let icon_kind_enum_inner = icon_kinds_camel_case.iter().map(|kind| {
        // Create doc comment with both SVG sizes
        let path_small = path_from_icon(kind, false);
        let path_big = path_from_icon(kind, true);
        let svg_html = format!(
            concat!(
                "<svg width='16' height='16' viewBox='0 0 {0} {0}' fill='currentColor'>",
                "    <path fill-rule='evenodd' d='{1}'/>",
                "</svg>",
                "",
                "<svg width='24' height='24' viewBox='0 0 {2} {2}' fill='currentColor'>",
                "    <path fill-rule='evenodd' d='{3}'/>",
                "</svg>",
            ),
            icon_size(path_small.1),
            path_small.0,
            icon_size(path_big.1),
            path_big.0,
        );
        let kind = format_ident!("{}", kind);
        quote! {
            #[doc = #svg_html]
            #kind,
        }
    });

    // Iterator over match arms in `IconKind::path()`
    let path_match_arms = icon_kinds_camel_case.iter().map(|kind| {
        let kind_ident = format_ident!("{}", kind);
        let (path_small, small_is_big) = path_from_icon(kind, false);
        let (path_big, big_is_big) = path_from_icon(kind, true);
        quote! {
            (IconKind::#kind_ident, false) => (#path_small, #small_is_big),
            (IconKind::#kind_ident, true) => (#path_big, #big_is_big),
        }
    });

    let code = quote! {
        /// Enum storing all the Octicons
        #[derive(PartialEq, PartialOrd, Clone, Copy, Hash, Debug)]
        pub enum IconKind {
            #(#icon_kind_enum_inner)*
        }

        impl IconKind {
            /// Given whether or not to use the big version of this icon, get a
            /// string with the `d` attribute on the `<path>` tag of the icon's
            /// SVG. If an icon does not exist for the requested size, fall back
            /// to the other size. The returned `bool` indicates whether the big
            /// version was used.
            pub(crate) fn path(self, big: bool) -> (&'static str, bool) {
                match (self, big) {
                    #(#path_match_arms)*
                }
            }
        }
    };

    std::fs::write(
        Path::new(&std::env::var("OUT_DIR").unwrap()).join("generated.rs"),
        code.to_string(),
    )
    .unwrap();
}
