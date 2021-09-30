use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub fn impl_base_update_derive(parsed_input: DeriveInput) -> TokenStream {
    let struct_name = parsed_input.ident;

    match parsed_input.data {
        Data::Struct(_s) => {
            let tokens = quote! {
                use std::any::Any;

                use crate::model::SpriteType;
                use crate::sprites::{BaseUpdate, CollisionMargin, Size, SpriteCell};

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

                    fn show(&mut self) {
                        self.sprite.show();
                    }

                    fn hide(&mut self) {
                        self.sprite.hide();
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

                    fn as_any(&mut self) -> &mut dyn Any {
                        self
                    }
                }
            };

            proc_macro::TokenStream::from(tokens)
        }
        other => panic!("BaseUpdate is not yet implemented for: {:?}", other),
    }
}
