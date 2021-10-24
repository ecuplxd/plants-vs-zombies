use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub fn impl_life_derive(parsed_input: DeriveInput) -> TokenStream {
    let struct_name = parsed_input.ident;

    match parsed_input.data {
        Data::Struct(_s) => {
            let tokens = quote! {
                use crate::sprites::{Attack, Life};

                impl Life for #struct_name {
                    fn get_life(&self) -> f64 {
                        self.life
                    }

                    fn set_life(&mut self, life: f64) {
                        self.life = life;
                    }
                }

                impl Attack for #struct_name {
                    fn get_hurt(&self) -> f64 {
                        self.hurt
                    }
                }
            };

            proc_macro::TokenStream::from(tokens)
        }
        other => panic!("Life is not yet implemented for: {:?}", other),
    }
}
