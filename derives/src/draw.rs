use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub fn impl_draw_derive(parsed_input: DeriveInput) -> TokenStream {
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
