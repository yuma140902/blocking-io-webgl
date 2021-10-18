use wasm_bindgen::{prelude::*, JsCast};
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    body.style().set_property("margin", "0").unwrap();
    body.style().set_property("padding", "0").unwrap();

    let canvas = document
        .create_element("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    body.append_child(&canvas).unwrap();

    let canvas_w: u32 = 1024;
    let canvas_h: u32 = 640;

    canvas.set_width(canvas_w);
    canvas.set_height(canvas_h);
    canvas.style().set_property("border", "solid 1px").unwrap();

    Ok(())
}
