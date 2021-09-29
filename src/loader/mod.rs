use std::collections::HashMap;
use std::rc::Rc;

use futures::future::join_all;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlImageElement, Request, RequestInit, RequestMode, Response};

use self::image::ImageFuture;
use crate::engine::EngineError;

mod image;

pub struct Loader<'a> {
    pub cells: Vec<&'a str>,
    pub data: Vec<&'a str>,
    pub level: Vec<&'a str>,
    images: Vec<(&'a str, &'a str)>,
}

impl<'a> Loader<'a> {
    pub fn new(
        cells: Vec<&'a str>,
        data: Vec<&'a str>,
        level: Vec<&'a str>,
        images: Vec<(&'a str, &'a str)>,
    ) -> Loader<'a> {
        Loader {
            cells,
            data,
            level,
            images,
        }
    }

    pub async fn load_jsons(&self, paths: &[&'a str]) -> Vec<JsValue> {
        let mut jsons: Vec<JsValue> = vec![];

        for json in paths.iter() {
            let result = self.load_json(json).await;

            if let Ok(value) = result {
                jsons.push(value);
            }
        }

        jsons
    }

    pub async fn load_json(&self, url: &str) -> Result<JsValue, EngineError> {
        // https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
        let mut opts = RequestInit::new();

        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(url, &opts)?;

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().unwrap();

        let json = JsFuture::from(resp.json()?).await?;

        Ok(json)
    }

    pub async fn load_images(&self) -> HashMap<String, Rc<HtmlImageElement>> {
        let image_futures: Vec<ImageFuture> = self
            .images
            .iter()
            .map(|path| ImageFuture::new(&(path.0.to_string() + path.1)))
            .collect();

        let future: Vec<Result<HtmlImageElement, ()>> = join_all(image_futures).await;

        let images: HashMap<String, Rc<HtmlImageElement>> = self
            .images
            .iter()
            .zip(future.into_iter())
            .filter(|(_key, value)| (*value).is_ok())
            .map(|(key, value)| (key.0.to_string(), Rc::new(value.unwrap())))
            .collect();

        images
    }
}
