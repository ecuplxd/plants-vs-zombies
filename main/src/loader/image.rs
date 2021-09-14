use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

use futures::task::{Context, Poll};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;

/// A future for loading a [HtmlImageElement](https://docs.rs/web-sys/0.3.39/web_sys/struct.HtmlImageElement.html)
/// that will resolve when the image has fully loaded.
///
/// Example:
/// ```rust
/// let image = ImageFuture::new("assets/sprite_sheet.png").await;
/// ```
///
/// It more or less replicates the promise in these lines of JS
/// ```javascript
/// const loadImage = src => new Promise((resolve, reject) => {
///  const img = new Image();
///  img.onload = resolve;
///  img.onerror = reject;
///  img.src = src;
/// })
/// ```
pub struct ImageFuture {
    image: Option<HtmlImageElement>,
    load_failed: Rc<Cell<bool>>,
}

impl ImageFuture {
    pub fn new(path: &str) -> Self {
        let image = HtmlImageElement::new().unwrap();
        image.set_src(path);
        ImageFuture {
            image: Some(image),
            load_failed: Rc::new(Cell::new(false)),
        }
    }
}

impl Future for ImageFuture {
    type Output = Result<HtmlImageElement, ()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match &self.image {
            Some(image) if image.complete() => {
                let failed = self.load_failed.get();
                if failed {
                    Poll::Ready(Err(()))
                } else {
                    let image = self.image.take().unwrap();
                    Poll::Ready(Ok(image))
                }
            }
            Some(image) => {
                let waker = cx.waker().clone();
                let on_load_closure = Closure::wrap(Box::new(move || {
                    waker.wake_by_ref();
                }) as Box<dyn FnMut()>);
                image.set_onload(Some(on_load_closure.as_ref().unchecked_ref()));
                on_load_closure.forget();

                let waker = cx.waker().clone();
                let failed_flag = self.load_failed.clone();
                let on_error_closure = Closure::wrap(Box::new(move || {
                    failed_flag.set(true);
                    waker.wake_by_ref();
                }) as Box<dyn FnMut()>);
                image.set_onerror(Some(on_error_closure.as_ref().unchecked_ref()));
                on_error_closure.forget();

                Poll::Pending
            }
            _ => Poll::Ready(Err(())),
        }
    }
}

pub async fn _load_image(path: &str) -> HtmlImageElement {
    let image = ImageFuture::new(path).await;

    return image.unwrap();
}
