// #![no_std]
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &JsValue);

  #[wasm_bindgen(js_namespace = app)]
  fn run(event: &str, p: &str);

  // pub type VNode;

  // #[wasm_bindgen(method, getter)]
  // fn tag(this: &VNode) -> String;

  // // fn props(this: &VNode) -> Map;

  // // fn children(this: &VNode) -> vec!<VDOM>;

}

#[wasm_bindgen]
pub fn render(element: HtmlElement, vdom: Vec<JsValue>) -> Result<(), JsValue> {

  log(&element);
  for item in vdom {
    let tag = js_sys::Reflect::get(&item, &JsValue::from_str("tag"))?;
    log(&tag);
  }

  Ok(())
}