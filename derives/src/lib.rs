extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Update)]
pub fn update_derive(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let struct_name = parsed_input.ident;

    match parsed_input.data {
        Data::Struct(_s) => {
            let tokens = quote! {
              impl Update for #struct_name {
                fn update(&self, now: f64, last_animation_frame_time: f64, pos: &Pos) {
                  self.sprite.update(now, last_animation_frame_time, pos);
                }

                fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
                  self.sprite.draw(context);
                }
              }
            };

            proc_macro::TokenStream::from(tokens)
        }
        other => panic!("Update is not yet implemented for: {:?}", other),
    }
}

#[proc_macro_derive(Resource)]
pub fn resource_derive(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let struct_name = parsed_input.ident;

    match parsed_input.data {
        Data::Struct(_s) => {
            let tokens = quote! {
              impl Resource for #struct_name {
                fn get_current_cell(&self) -> Option<&SpriteCell> {
                  return self.artist.get_current_cell();
                }

                fn in_last_cell(&self) -> bool {
                  return self.artist.in_last_cell();
                }

                fn advance(&mut self) {
                  self.artist.advance();
                }

                fn goto(&mut self, index: usize) {
                  self.artist.goto(index);
                }
              }
            };

            proc_macro::TokenStream::from(tokens)
        }
        other => panic!("Update is not yet implemented for: {:?}", other),
    }
}
