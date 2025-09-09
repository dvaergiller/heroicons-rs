use std::{ffi::OsStr, fs::read_dir, path::{Path, PathBuf}};

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn icons_in_path(path_tokens: TokenStream) -> TokenStream {
    let path = parse_macro_input!(path_tokens as LitStr).value();
    let svg_files = svg_files_in_path(&path);

    let icon_tokens = svg_files.map(|path| {
        let icon_name = path_to_icon_name(&path);
        let icon_name_ident = format_ident!("{icon_name}");
        let path_str = path.as_os_str().to_str();
        quote! {
            (Icon{ name: #icon_name_ident, variant: Outline }, #path_str)
        }
    });

    quote! {
        &[
            #(#icon_tokens),*
        ]
    }.into()
}

#[proc_macro]
pub fn for_each_icon(tokens: TokenStream) -> TokenStream {
    let mut tokens_iter = tokens.into_iter();
    let path_tokens = tokens_iter.next().unwrap().into();
    let _ = tokens_iter.next();
    let callback_tokens = tokens_iter.collect();

    let path = parse_macro_input!(path_tokens as syn::LitStr).value();
    let callback = parse_macro_input!(callback_tokens as syn::ExprClosure);
    let svg_files = svg_files_in_path(&path);

    let icon_tokens = svg_files.map(|path| {
        let icon_name = path_to_icon_name(&path);
        let icon_name_ident = format_ident!("{icon_name}");
        let path_str = path.as_os_str().to_str();

        quote! {
            let callback = #callback;
            callback(#icon_name_ident, #path_str);
        }
    });

    quote! {
        #(#icon_tokens)*
    }.into()
}

fn svg_files_in_path(path: impl Into<PathBuf>) -> impl Iterator<Item = PathBuf> {
    read_dir(&path.into())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension() == Some(OsStr::new("svg")))
}

fn path_to_icon_name(path: &Path) -> String {
    let stem = path.file_stem().unwrap().to_string_lossy().into_owned();

    // Convert to PascalCase
    stem.split('-')
        .map(|word| {
            let (first, rest) = word.split_at(1);
            let mut fixed = first.to_ascii_uppercase();
            fixed.push_str(rest);
            fixed
        })
        .collect::<String>()
}
