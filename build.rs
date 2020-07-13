use std::collections::BTreeSet;

use heck::CamelCase;
use heck::KebabCase;
use quote::format_ident;
use quote::quote;

fn path_from_icon_kind(kind: &str, big: bool) -> proc_macro2::TokenStream {
    let size = if big { 24 } else { 16 };
    let file_path = format!("octicons/icons/{}-{}.svg", kind.to_kebab_case(), size);
    println!("{}", file_path);
    let file_content = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        // Hopefully this doesn't loop forever
        Err(_) => return path_from_icon_kind(kind, !big),
    };
    scraper::Html::parse_fragment(&file_content)
        .select(&scraper::Selector::parse("path").unwrap())
        .next()
        .unwrap()
        .html()
        .parse()
        .unwrap()
}

fn main() {
    let icon_kinds_camel_case = std::fs::read_dir("octicons/icons")
        .unwrap()
        .map(|dir_entry| {
            dir_entry
                .unwrap()
                .file_name()
                .into_string()
                .unwrap()
                .rsplitn(2, '-')
                .nth(1)
                .unwrap()
                .to_camel_case()
        })
        .collect::<BTreeSet<String>>()
        .into_iter()
        .collect::<Vec<String>>();

    let icon_kind_enum_inner = icon_kinds_camel_case
        .iter()
        .map(|kind| format_ident!("{}", kind));

    let path_match_arms = icon_kinds_camel_case.iter().map(|kind| {
        let kind_ident = format_ident!("{}", kind);
        let path_small = path_from_icon_kind(kind, false);
        let path_big = path_from_icon_kind(kind, true);
        quote! {
            (IconKind::#kind_ident, false) => html!(#path_small),
            (IconKind::#kind_ident, true) => html!(#path_big),
        }
    });

    let code = quote! {
        use yew::html::Html;
        use yew::html;

        pub enum IconKind {
            #(#icon_kind_enum_inner),*
        }

        impl IconKind {
            pub(crate) fn path(self, big: bool) -> Html {
                match (self, big) {
                    #(#path_match_arms)*
                }
            }
        }
    };

    std::fs::write("src/generated.rs", code.to_string()).unwrap();
}
