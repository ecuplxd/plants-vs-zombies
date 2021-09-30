mod behavior;
mod behavior_field;
mod draw;
mod life;
mod update;

extern crate proc_macro;

use behavior_field::MacroInput;
use proc_macro::{Delimiter, Group, Ident, TokenStream, TokenTree};
use syn::{parse_macro_input, DeriveInput};

use crate::behavior::{
    impl_with_callback_derive, impl_with_timer_derive, impl_without_callback_derive,
    impl_without_timer_derive,
};
use crate::draw::impl_draw_derive;
use crate::life::impl_life_derive;
use crate::update::impl_base_update_derive;

#[proc_macro_derive(BaseUpdate)]
pub fn base_update_derive(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);

    impl_base_update_derive(parsed_input)
}

#[proc_macro_derive(Draw)]
pub fn draw_derive(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);

    impl_draw_derive(parsed_input)
}

#[proc_macro_derive(WithoutTimer)]
pub fn without_timer_derive(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);

    impl_without_timer_derive(parsed_input)
}

#[proc_macro_derive(WithTimer, attributes(behavior))]
pub fn with_timer_derive(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);

    impl_with_timer_derive(parsed_input)
}

#[proc_macro_derive(WithCallback)]
pub fn with_callback_derive(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);

    impl_with_callback_derive(parsed_input)
}

#[proc_macro_derive(WithoutCallback)]
pub fn without_callback_derive(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);

    impl_without_callback_derive(parsed_input)
}

#[proc_macro_derive(Life)]
pub fn life_derive(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);

    impl_life_derive(parsed_input)
}

#[proc_macro_attribute]
pub fn derive_behavior(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut found_struct = false;
    let input = syn::parse_macro_input!(attr as MacroInput);

    item.into_iter()
        .map(|r| match &r {
            &TokenTree::Ident(ref ident) if is_struct(ident) => {
                found_struct = true;
                r
            }
            &TokenTree::Group(ref group) if is_brace(group.delimiter()) && found_struct => {
                let mut stream = TokenStream::new();
                let fields: Vec<TokenStream> = MacroInput::get_fields(input.kind.to_string());

                stream.extend(fields.into_iter());
                stream.extend(group.stream());

                TokenTree::Group(Group::new(Delimiter::Brace, stream))
            }
            _ => r,
        })
        .collect()
}

fn is_struct(ident: &Ident) -> bool {
    ident.to_string() == "struct"
}

fn is_brace(delimiter: Delimiter) -> bool {
    matches!(delimiter, Delimiter::Brace)
}
