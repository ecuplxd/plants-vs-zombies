use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;

use crate::game::Game;
use crate::loader::Loader;
use crate::model::{Event, LevelData};
use crate::sprites::{SpriteCell, SpriteData};
use crate::util::request_animation_frame;
use crate::{asset_image, asset_json};

pub struct Engine {
    game: Rc<RefCell<Game>>,
}

pub enum EngineError {
    IO(std::io::Error),
    Js(JsValue),
}

impl From<JsValue> for EngineError {
    fn from(e: JsValue) -> Self {
        EngineError::Js(e)
    }
}

impl From<EngineError> for JsValue {
    fn from(e: EngineError) -> Self {
        match e {
            EngineError::Js(e) => e,
            EngineError::IO(e) => JsValue::from_str(&e.to_string()),
        }
    }
}

impl Engine {
    pub fn launch() {
        spawn_local(async move {
            let loader = Loader::new(
                vec![
                    asset_json!("interface-cell"),
                    asset_json!("card-cell"),
                    asset_json!("plant-cell"),
                    asset_json!("zombie-cell"),
                ],
                vec![
                    asset_json!("interface-data"),
                    asset_json!("card-data"),
                    asset_json!("plant-data"),
                    asset_json!("zombie-data"),
                ],
                vec![asset_json!("level-data")],
                vec![
                    asset_image!("interface"),
                    asset_image!("card"),
                    asset_image!("plant"),
                    asset_image!("zombie"),
                ],
            );

            let engine = Engine {
                game: Rc::new(RefCell::new(Game::new())),
            };
            let game = Rc::clone(&engine.game);

            {
                let mut game = game.borrow_mut();

                loader
                    .load_jsons(&loader.cells)
                    .await
                    .iter()
                    .map(|json| SpriteCell::from_json(json))
                    .for_each(|cell| game.resource.cells.extend(cell));

                loader
                    .load_jsons(&loader.data)
                    .await
                    .iter()
                    .map(|json| SpriteData::from_json(json))
                    .for_each(|data| game.resource.data.extend(data));

                loader
                    .load_jsons(&loader.level)
                    .await
                    .iter()
                    .map(|json| LevelData::new_from_json(json))
                    .for_each(|level| game.leval_data.extend(level));

                game.select_level(0);
                game.resource.sheets.extend(loader.load_images().await);

                game.init();
            }

            engine.listen_event(Event::Mouseenter);
            engine.listen_event(Event::Mouseleave);
            engine.listen_event(Event::Mousedown);
            engine.listen_event(Event::Mouseup);
            engine.listen_event(Event::Mousemove);

            engine.start_loop();
        });
    }

    fn listen_event(&self, name: Event) {
        let game = Rc::clone(&self.game);
        let game2 = Rc::clone(&self.game);
        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            game.borrow_mut().dispatch_event(name, event);
        }) as Box<dyn FnMut(_)>);

        game2
            .borrow()
            .canvas
            .add_event_listener_with_callback(
                &name.to_string().to_lowercase(),
                closure.as_ref().unchecked_ref(),
            )
            .unwrap();

        closure.forget();
    }

    fn start_loop(&self) {
        let f = Rc::new(RefCell::new(None));
        let g = Rc::clone(&f);
        let game = Rc::clone(&self.game);

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let mut game = game.borrow_mut();

            game.run();

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}
