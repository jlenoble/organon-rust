use gloo::console::log;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let msg = format!("Hello from Rust!");
    log!(&msg);

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_text_content(Some(&msg));

    body.append_child(&val)?;

    Ok(())
}