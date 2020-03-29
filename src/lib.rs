
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use web_sys::SvgElement;
use js_sys::Reflect;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &JsValue);

  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_str(s: &str);

  #[wasm_bindgen(js_namespace = JSON, js_name = stringify)]
  fn to_json(s: &JsValue) -> JsValue;

  #[wasm_bindgen(js_namespace = app)]
  fn run(event: &str, p: &str);
}

macro_rules! console_log {
  ($($t:tt)*) => (log_str(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn render(element: HtmlElement, vdom: Vec<JsValue>) -> Result<(), JsValue> {
  log(&element);
  for vnode in vdom {
    match vnode {
      vnode if vnode.is_string() => create_text(&element, &vnode),
      vnode if vnode.is_instance_of::<HtmlElement>()
            || vnode.is_instance_of::<SvgElement>() => insert_element(&element, &vnode),
      _ => try_create_element(&element, &vnode)
    }
  }
  Ok(())
}

fn create_text(element: &HtmlElement, text: &JsValue) {
  console_log!("vdom: create text:");
  log(&text);
}

fn insert_element(element: &HtmlElement, vnode: &JsValue) {
  console_log!("vdom: insert element:");
  log(&vnode);
}

fn try_create_element(element: &HtmlElement, vnode: &JsValue) {
  console_log!("vdom: create element:");
  log(&vnode);
  match Reflect::get(&vnode, &JsValue::from_str("tag")) {
    Ok(tag) => {
      if tag.is_undefined() {
        create_text(&element, &to_json(&vnode));
      } else {
        create_element(&element, &tag, &vnode);
      }
    },
    _ => {
      if !vnode.is_falsy() {
        create_text(&element, &to_json(&vnode));
      }
    }
  }
}

fn create_element(element: &HtmlElement, tag: &JsValue, vnode: &JsValue) {
  console_log!("vdom: create element:");
  log(&tag);
  log(&vnode);
}

fn update_element(element: &HtmlElement, vnode: &JsValue, is_svg: &bool) {
  console_log!("vdom: update element:");
  log(&vnode);
}

fn update_element_props(element: &HtmlElement, vnode: &JsValue, is_svg: &bool) {
  console_log!("vdom: update element props:");
  log(&vnode);
}