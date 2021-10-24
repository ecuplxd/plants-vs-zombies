use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};

#[derive(Debug)]
pub struct MacroInput {
    pub kind: String,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let kind = input.parse::<syn::LitStr>()?;

        Ok(MacroInput { kind: kind.value() })
    }
}

impl MacroInput {
    pub fn get_fields(kind: String) -> Vec<TokenStream> {
        let mut field_map: HashMap<String, Vec<TokenStream>> = HashMap::new();

        field_map.insert(
            String::from("default"),
            vec![
                quote!(running: bool,).into(),
                quote!(sprite: SpritePointer,).into(),
                quote!(callbacks: Vec<ErasedFnPointer<SpritePointer>>,).into(),
            ],
        );
        field_map.insert(
            String::from("without_callback"),
            vec![
                quote!(running: bool,).into(),
                quote!(sprite: SpritePointer,).into(),
            ],
        );
        field_map.insert(
            String::from("with_callback"),
            vec![
                quote!(sprite: SpritePointer,).into(),
                quote!(callbacks: Vec<ErasedFnPointer<SpritePointer>>,).into(),
            ],
        );

        field_map.remove(&kind).unwrap()
    }
}
