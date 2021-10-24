use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident};

pub fn impl_without_timer_derive(parsed_input: DeriveInput) -> TokenStream {
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

pub fn impl_with_timer_derive(parsed_input: DeriveInput) -> TokenStream {
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

pub fn impl_with_callback_derive(parsed_input: DeriveInput) -> TokenStream {
    let struct_name = parsed_input.ident;
    let sprite_name = match parsed_input.attrs.get(0) {
        // TODO：解析 ident
        Some(_attr) => quote!(cycle.sprite),
        None => quote!(sprite),
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

                    fn add_callback(&mut self, callback: ErasedFnPointer<SpritePointer>) {
                        let exit = self
                            .callbacks
                            .iter()
                            .find(|item| item.struct_pointer == callback.struct_pointer && item.fp == callback.fp);

                        if exit.is_none() {
                            self.callbacks.push(callback);
                        }
                    }

                    fn execute_callback(&self) {
                        self.callbacks.iter().for_each(|callback| callback.call(self.#sprite_name));
                    }
                }
            };

            proc_macro::TokenStream::from(tokens)
        }
        other => panic!("BehaviorCallback is not yet implemented for: {:?}", other),
    }
}

pub fn impl_without_callback_derive(parsed_input: DeriveInput) -> TokenStream {
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
