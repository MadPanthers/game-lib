use crate::memory::Memory;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Renderer<'a> {
    canvas: &'a web_sys::HtmlCanvasElement,
    memory: &'a Memory,
}

impl<'a> Renderer<'a> {
    pub fn new(canvas: &'a web_sys::HtmlCanvasElement, memory: &'a Memory) -> Renderer<'a> {
        Renderer {
            canvas,
            memory
        }
    }

    pub fn render(&self) {
        let context = self.canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();


        for (index, colour) in self.memory.board.iter().enumerate() {
            context.set_fill_style(
                &wasm_bindgen::JsValue::from(format!("#{:02x}{:02x}{:02x}", colour.red, colour.green, colour.blue))
            );
            context.fill_rect((index % self.memory.width) as f64, (index / self.memory.width) as f64, 1.0, 1.0);
        }
    }
}