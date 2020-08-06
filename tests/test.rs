use wasm_bindgen_test::*;
use wasm_bindgen::{JsCast, Clamped};
use game_lib::renderer::Renderer;
use game_lib::memory::Memory;
use game_lib::memory::colour::Colour;
use game_lib::memory::position::Position;

#[cfg(test)]
mod test {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
}

#[wasm_bindgen_test]
fn init_renderer() {

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas: web_sys::HtmlCanvasElement = document.create_element("canvas")
        .unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    canvas.set_attribute("height", "200").expect("Could not add the height");
    canvas.set_attribute("width", "200").expect("Could not add the width");
    document.body().unwrap().append_child(&canvas).expect("Could not attach canvas to the body");

    let width = canvas.width();
    let height = canvas.height();

    let mut memory = Memory::new(width as usize, height as usize);
    memory.set(Position { x: 10, y: 10 }, Colour{ red: 255, green: 0, blue: 0});

    let renderer = Renderer::new(&canvas, &memory);
    renderer.render();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let image_data = context.get_image_data(10.0, 10.0, 1.0, 1.0).unwrap().data();
    console_log!("{:?}",image_data);

    assert_eq!(Clamped{ 0: vec![255,0,0,255] } == image_data, true);
}
