extern crate proc_macro;

use proc_macro::{Delimiter, Group, TokenStream, TokenTree};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Data, DeriveInput, Ident};

#[proc_macro_derive(BaseUpdate)]
pub fn base_update_derive(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let struct_name = parsed_input.ident;

    match parsed_input.data {
        Data::Struct(_s) => {
            let tokens = quote! {
                use crate::model::SpriteType;
                use crate::sprites::{CollisionMargin, Size, SpriteCell};


                impl BaseUpdate for #struct_name {
                    fn add_behavior(&mut self, behavior: Box<dyn Behavior>) {
                        self.sprite.add_behavior(behavior);
                    }

                    fn get_ref_artist(&self) -> &dyn Draw {
                        self.sprite.get_ref_artist()
                    }

                    fn get_mut_artist(&mut self) -> &mut dyn Draw {
                        self.sprite.get_mut_artist()
                    }

                    fn get_mut_behaviors(&mut self) -> &mut Vec<Box<dyn Behavior>> {
                        self.sprite.get_mut_behaviors()
                    }

                    fn toggle(&mut self) {
                        self.sprite.toggle();
                    }

                    fn is_visible(&self) -> bool {
                        self.sprite.is_visible()
                    }

                    fn id(&self) -> String {
                        self.sprite.id()
                    }

                    fn name(&self) -> SpriteType {
                        self.sprite.name()
                    }

                    fn point_in_path(&self, mouse_pos: &Pos, context: &CanvasRenderingContext2d) -> bool {
                        self.sprite.point_in_path(mouse_pos, context)
                    }

                    fn update_outlines(&mut self) {
                        self.sprite.update_outlines();
                    }

                    fn update_loc(&mut self, loc: Loc) {
                        self.sprite.update_loc(loc);
                    }

                    fn get_order(&self) -> usize {
                        self.sprite.get_order()
                    }

                    fn get_rect(&self) -> SpriteCell {
                        self.sprite.get_rect()
                    }

                    fn get_pos(&self) -> Pos {
                        self.sprite.get_pos()
                    }

                    fn get_loc(&self) -> Loc {
                        self.sprite.get_loc()
                    }

                    fn get_collision_margin(&self) -> CollisionMargin {
                        self.sprite.get_collision_margin()
                    }

                    fn set_clicked(&mut self, clicked: bool) {
                        self.sprite.set_clicked(clicked);
                    }

                    fn is_clicked(&self) -> bool {
                        self.sprite.is_clicked()
                    }
                }
            };

            proc_macro::TokenStream::from(tokens)
        }
        other => panic!("BaseUpdate is not yet implemented for: {:?}", other),
    }
}

#[proc_macro_derive(Draw)]
pub fn draw_derive(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let struct_name = parsed_input.ident;

    match parsed_input.data {
        Data::Struct(_s) => {
            let tokens = quote! {
                use web_sys::CanvasRenderingContext2d;

                use crate::artists::{Draw, Stroke};

                impl Stroke for #struct_name {}

                impl Draw for #struct_name {
                    fn draw(&self, context: &CanvasRenderingContext2d) {
                        self.sprite.draw(context);
                    }

                    fn in_last_cell(&self) -> bool {
                        self.sprite.in_last_cell()
                    }

                    fn advance(&mut self) {
                        self.sprite.advance()
                    }
                }
            };

            proc_macro::TokenStream::from(tokens)
        }
        other => panic!("Draw is not yet implemented for: {:?}", other),
    }
}

#[proc_macro_derive(WithoutTimer)]
pub fn without_timer_derive(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let struct_name = parsed_input.ident;

    match parsed_input.data {
        Data::Struct(_s) => {
            let tokens = quote! {
                use super::BehaviorState;

                impl BehaviorState for #struct_name {
                    fn start(&mut self, _now: f64) {
                        self.running = true;
                    }

                    fn stop(&mut self, _now: f64) {
                        self.running = false;
                    }

                    fn is_running(&self) -> bool {
                        self.running
                    }
                }
            };

            proc_macro::TokenStream::from(tokens)
        }
        other => panic!("BehaviorState is not yet implemented for: {:?}", other),
    }
}

#[proc_macro_derive(WithTimer, attributes(behavior))]
pub fn with_timer_derive(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let timer_name = match parsed_input.attrs.get(0) {
        // TODO：解析 ident
        Some(_attr) => Ident::new("cycle", parsed_input.ident.span()),
        None => Ident::new("timer", parsed_input.ident.span()),
    };
    let struct_name = parsed_input.ident;

    match parsed_input.data {
        Data::Struct(_s) => {
            let tokens = quote! {
                use super::BehaviorState;

                impl BehaviorState for #struct_name {
                    fn start(&mut self, now: f64) {
                        self.#timer_name.start(now);
                    }

                    fn stop(&mut self, now: f64) {
                        self.#timer_name.stop(now);
                    }

                    fn is_running(&self) -> bool {
                        self.#timer_name.is_running()
                    }
                }
            };

            proc_macro::TokenStream::from(tokens)
        }
        other => panic!("BehaviorState is not yet implemented for: {:?}", other),
    }
}

#[proc_macro_derive(WithCallback)]
pub fn with_callback_derive(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let struct_name = parsed_input.ident;
    let sprite_name = match parsed_input.attrs.get(0) {
        // TODO：解析 ident
        Some(_attr) => quote! { cycle.sprite },
        None => quote! { sprite },
    };

    match parsed_input.data {
        Data::Struct(_s) => {
            let tokens = quote! {
                use std::ptr::NonNull;
                use super::BehaviorCallback;

                impl BehaviorCallback for #struct_name {
                    fn set_sprite(&mut self, sprite: *mut dyn Update) {
                        self.#sprite_name = NonNull::new(sprite);
                    }

                    fn set_cb(&mut self, cb: ErasedFnPointer<SpritePointer>) {
                        self.cb = Some(cb);
                    }

                    fn execute_callback(&self) {
                        match self.cb {
                            Some(cb) => cb.call(self.#sprite_name),
                            _ => (),
                        }
                    }
                }
            };

            proc_macro::TokenStream::from(tokens)
        }
        other => panic!("BehaviorCallback is not yet implemented for: {:?}", other),
    }
}

#[proc_macro_derive(WithoutCallback)]
pub fn without_callback_derive(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let struct_name = parsed_input.ident;

    match parsed_input.data {
        Data::Struct(_s) => {
            let tokens = quote! {
                use std::ptr::NonNull;
                use super::BehaviorCallback;

                impl BehaviorCallback for #struct_name {
                    fn set_sprite(&mut self, sprite: *mut dyn Update) {
                        self.sprite = NonNull::new(sprite);
                    }
                }
            };

            TokenStream::from(tokens)
        }
        other => panic!("BehaviorCallback is not yet implemented for: {:?}", other),
    }
}

#[derive(Debug)]
struct MacroInput {
    default: bool,
    no_callback: bool,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let kind = input.parse::<syn::LitStr>()?;

        Ok(MacroInput {
            default: kind.value() == "default",
            no_callback: kind.value() == "no_callback",
        })
    }
}

#[proc_macro_attribute]
pub fn derive_behavior(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(attr as MacroInput);
    let mut found_struct = false;

    item.into_iter()
        .map(|r| match &r {
            &TokenTree::Ident(ref ident) if ident.to_string() == "struct" => {
                found_struct = true;
                r
            }
            &TokenTree::Group(ref group)
                if group.delimiter() == Delimiter::Brace && found_struct == true =>
            {
                let mut stream = TokenStream::new();
                let fields: Vec<TokenStream> = match (input.default, input.no_callback) {
                    (true, _) => vec![
                        // quote!(name: BehaviorType,).into(),
                        quote!(running: bool,).into(),
                        quote!(sprite: SpritePointer,).into(),
                        quote!(cb: Option<ErasedFnPointer<SpritePointer>>,).into(),
                    ],
                    (false, true) => vec![
                        quote!(running: bool,).into(),
                        quote!(sprite: SpritePointer,).into(),
                    ],
                    _ => vec![],
                };

                stream.extend(fields.into_iter());
                stream.extend(group.stream());

                TokenTree::Group(Group::new(Delimiter::Brace, stream))
            }
            _ => r,
        })
        .collect()
}
