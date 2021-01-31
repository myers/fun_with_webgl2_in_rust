use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

pub struct RenderLoop<F>
where
    F: FnMut(f64),
{
    callback: F,
    frame_rate: u32,
    stop: bool,
}

impl<F> RenderLoop<F>
where
    F: FnMut(f64),
{
    pub fn new(f: F, frame_rate: u32) -> Self {
        Self {
            callback: f,
            frame_rate: frame_rate,
            stop: true,
        }
    }

    pub fn start(&mut self) {
        self.stop = false;
        let f = Rc::new(RefCell::new(None));
        let callback = Rc::new(RefCell::new(self.callback));
        let callback_clone = callback.clone();
        let g = f.clone();

        let performance = window()
            .performance()
            .expect("performance should be available");

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            console::log_1(&"on_render".into());

            let callback = callback.borrow();
            (callback)(performance.now());

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
