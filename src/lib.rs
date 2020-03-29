
use std::cmp;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::Reflect;

#[wasm_bindgen]
extern "C" {
  // AppRun
  #[wasm_bindgen(js_namespace = app)]
  fn run(event: &str, p: &str);

  // DOM
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &JsValue);

  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_str(s: &str);

  #[wasm_bindgen(js_namespace = JSON, js_name = stringify)]
  fn to_json(s: &JsValue) -> JsValue;

  pub type Element;
  pub type SVGElement;
  pub type HTMLElement;

  #[wasm_bindgen(js_namespace = document, js_name = createDocumentFragment)]
  fn createDocumentFragment() -> Element;

  #[wasm_bindgen(js_namespace = document, js_name = createElement)]
  fn createElement(tag: &JsValue) -> Element;

  #[wasm_bindgen(js_namespace = document, js_name = createElementNS)]
  fn createElementNS(namespace: &JsValue, tag: &JsValue) -> Element;

  #[wasm_bindgen(js_namespace = document, js_name = createTextNode)]
  fn createTextNode(tag: &JsValue) -> Element;

  #[wasm_bindgen(method)]
  pub fn appendChild(this: &Element, child: &Element);

  #[wasm_bindgen(method, js_name = appendChild)]
  pub fn appendChild2(this: &Element, child: &JsValue);

  #[wasm_bindgen(method)]
  pub fn insertBefore(this: &Element, child: &JsValue, element: &JsValue);

  #[wasm_bindgen(method)]
  pub fn replaceChild(this: &Element, child: &JsValue, element: &JsValue);

  #[wasm_bindgen(method)]
  pub fn removeChild(this: &Element, child: &JsValue);

  #[wasm_bindgen(method, getter)]
  pub fn childNodes(this: &Element) -> Vec<JsValue>;

  #[wasm_bindgen(method, setter)]
  pub fn set_textContext(this: &Element, text: &JsValue);

  #[wasm_bindgen(method, getter)]
  pub fn nodeType(this: &Element) -> u32;

  #[wasm_bindgen(method, getter)]
  pub fn lastChild(this: &Element) -> JsValue;

  #[wasm_bindgen(method, getter)]
  pub fn parentNode(this: &Element) -> JsValue;

}

#[wasm_bindgen]
pub fn render(element: &Element, vdom: Vec<JsValue>, is_svg: &JsValue) {
  update_children(element, vdom, is_svg);
}

fn update_children (element: &Element, children: Vec<JsValue>, is_svg: &JsValue) {
  // log_str("vdom: update element children:");

  let el_nodes = element.childNodes();
  let old_len = el_nodes.len();
  let new_len = children.len();
  let len = cmp::min(old_len, new_len);

  for i in 0..len {
    let ch = &children[i];
    let el = &el_nodes[i];

    match ch {
      ch if ch.is_instance_of::<HTMLElement>() || ch.is_instance_of::<SVGElement>() => element.insertBefore(&ch, &el),
      // ch if ch.has_type::<string>() => {

      // },
      _ => {}
    }
  }


  let mut n = element.childNodes().len();
  while n > len {
    element.removeChild(&element.lastChild());
    n = n - 1;
  }

  if new_len > len {
    let doc_fragment = createDocumentFragment();
    for i in len..new_len {
      let vnode = &children[i];
      if vnode.is_instance_of::<HTMLElement>() || vnode.is_instance_of::<SVGElement>() {
        doc_fragment.appendChild2(&vnode);
      } else {
        let new_element = create_element(&vnode, is_svg);
        doc_fragment.appendChild(&new_element);
      }
    }
    element.appendChild(&doc_fragment);
  }
}

fn create_text(text: &JsValue) -> Element {
  // log_str("vdom: create text:");
  // Todo: create
  // if (node.indexOf('_html:') === 0) { // ?
  //   const div = document.createElement('div');
  //   div.insertAdjacentHTML('afterbegin', node.substring(6))
  //   return div;
  // }
  createTextNode(text)
}

fn create_element(vnode: &JsValue, is_svg: &JsValue) -> Element {
  // log_str("vdom: create element:");
  match Reflect::get(&vnode, &JsValue::from_str("tag")) {
    Ok(tag) => {
      if tag.is_undefined() {
        create_text(&to_json(&vnode))
      } else {
        let element;

        if !is_svg.is_falsy() || tag == JsValue::from_str("svg") {
          element = createElementNS(&JsValue::from_str("http://www.w3.org/2000/svg"), &tag);
        } else {
          element = createElement(&tag);
        }

        match Reflect::get(&vnode, &JsValue::from_str("props")) {
          Ok(props) => update_element_props(&element, &props, is_svg),
          Err(_) => {}
        };

        match Reflect::get(&vnode, &JsValue::from_str("children")) {
          Ok(children) => {
            if !children.is_falsy() {
              let arr = js_sys::Array::from(&children);
              arr.for_each(&mut |node, _, __| {
                let new_element = create_element(&node, is_svg);
                element.appendChild(&new_element);
              });
            }
          },
          Err(_) => {}
        };

        element
      }
    },
    _ => {
      if !vnode.is_falsy() {
        create_text(&to_json(&vnode))
      } else {
        create_text(&JsValue::from_str(""))
      }
    }
  }
}

fn update_element(element: &Element, vnode: &JsValue, is_svg: &JsValue) {
  // log_str("vdom: update element:");
  // isSvg = isSvg || node.tag == "svg";
  // //console.log('update', element, node);
  // if (!same(element, node) || node instanceof HTMLElement || node instanceof SVGElement) {
  //   element.parentNode.replaceChild(create(node, isSvg), element);
  //   return;
  // }

  match Reflect::get(&vnode, &JsValue::from_str("tag")) {
    Ok(tag) => {
      if !is_svg.is_falsy() || tag == JsValue::from_str("svg") {

        return;
      }
    },
    Err(_) => {}
  };

  match Reflect::get(&vnode, &JsValue::from_str("props")) {
    Ok(props) => update_element_props(&element, &props, is_svg),
    Err(_) => {}
  };

  match Reflect::get(&vnode, &JsValue::from_str("children")) {
    Ok(children) => {
      if !children.is_falsy() {
        // update_children(&element, &children, is_svg);
      }
    },
    Err(_) => {}
  };
}


fn update_element_props(element: &Element, vnode: &JsValue, is_svg: &JsValue) {
  // log_str("vdom: update element props:");
  // log(&vnode);
}
