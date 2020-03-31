
use std::cmp;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::Reflect;
use js_sys::Array;
use js_sys::Object;

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
  pub fn childNodes(this: &Element) -> Array;

  #[wasm_bindgen(method, getter)]
  pub fn nodeType(this: &Element) -> u32;

  #[wasm_bindgen(method, getter)]
  pub fn lastChild(this: &Element) -> Element;

  #[wasm_bindgen(method, getter)]
  pub fn parentNode(this: &Element) -> Element;

  #[wasm_bindgen(method, getter)]
  pub fn textContent(this: &Element) -> JsValue;

  #[wasm_bindgen(method, setter)]
  pub fn set_textContent(this: &Element, text: &JsValue);

}

fn arr_get(target: &JsValue, idx: &u32) -> JsValue {
  Reflect::get(target, &JsValue::from_f64(*idx as f64)).unwrap()
}

#[wasm_bindgen]
pub fn render(element: &Element, vdom: Array, is_svg: &JsValue) {
  update_children(element, vdom, is_svg);
}

fn update_children (element: &Element, children: Array, is_svg: &JsValue) {
  // log_str("vdom: update element children:");

  let el_nodes = element.childNodes();
  let old_len = el_nodes.length();
  let new_len = children.length();
  let len = cmp::min(old_len, new_len);

  // update
  for i in 0..len {
    let old_el = arr_get(&el_nodes, &i);
    let new_el = arr_get(&children, &i);

    if new_el.is_instance_of::<HTMLElement>() || new_el.is_instance_of::<SVGElement>() {
      element.insertBefore(&new_el, &old_el);
      break;
    }

    //Todo
    // let key = get(&ch, "key");
    // if !key.is_falsy() {
    //   break;
    // }

    let elem: Result<Element, JsValue> = old_el.dyn_into();
    update_element(&elem.unwrap(), &new_el, is_svg);
  }

  let mut n = element.childNodes().length();
  while n > len {
    element.removeChild(&element.lastChild());
    n = n - 1;
  }


  if new_len > len {
    let doc_fragment = createDocumentFragment();
    for i in len..new_len {
      let vnode = arr_get(&children, &i);
      if vnode.is_instance_of::<HTMLElement>() || vnode.is_instance_of::<SVGElement>() {
        doc_fragment.appendChild2(&vnode);
      } else {
        let new_element = create_element(vnode, is_svg);
        doc_fragment.appendChild(&new_element);
      }
    }
    element.appendChild(&doc_fragment);
  }
}

fn create_text(text: &JsValue) -> Element {
  // log_str("vdom: create text:");
  // Todo:
  // if (node.indexOf('_html:') === 0) { // ?
  //   const div = document.createElement('div');
  //   div.insertAdjacentHTML('afterbegin', node.substring(6))
  //   return div;
  // }
  createTextNode(text)
}

fn create_element(vnode: JsValue, is_svg: &JsValue) -> Element {
  // log_str("vdom: create element:");

  if vnode.is_string() {
    return create_text(&to_json(&vnode))
  } else if vnode.is_instance_of::<HTMLElement>() || vnode.is_instance_of::<SVGElement>() {
    let element: Result<Element, JsValue> = vnode.dyn_into();
    return element.unwrap();
  }
  match Reflect::get(&vnode, &JsValue::from_str("tag")) {
    Ok(tag) if !&tag.is_falsy() => {
      let element;
      if !is_svg.is_falsy() || tag == JsValue::from_str("svg") {
        element = createElementNS(&JsValue::from_str("http://www.w3.org/2000/svg"), &tag);
      } else {
        element = createElement(&tag);
      }
      match Reflect::get(&vnode, &JsValue::from_str("props")) {
        Ok(props) => update_element_props(&element, &props, is_svg),
        _ => {}
      }
      match Reflect::get(&vnode, &JsValue::from_str("children")) {
        Ok(children) if !children.is_falsy() => update_children(&element, Array::from(&children), is_svg),
        _ => {}
      }
      element
    },
    _ => create_text(&to_json(&vnode))
  }
}

fn update_element(element: &Element, vnode: &JsValue, is_svg: &JsValue) {
  // log_str("vdom: update element:");

  if vnode.is_string() {
    if Object::is(&vnode, &element.textContent()) {
      if element.nodeType() == 3 {
        element.set_textContent(&vnode)
      } else {
        let text = create_text(&vnode);
        element.parentNode().replaceChild(&text, element);
      }
    }
  } else if vnode.is_instance_of::<HTMLElement>() || vnode.is_instance_of::<SVGElement>() {
    element.parentNode().replaceChild(&vnode, element)
  // } else if !same(element, node) {
  //   element.parentNode.replaceChild(create(node, isSvg), element);
  } else {

    match Reflect::get(&vnode, &JsValue::from_str("tag")) {
      Ok(tag) if !&tag.is_falsy() => {

        //let isSvg = is_svg || tag == "svg";

        match Reflect::get(&vnode, &JsValue::from_str("props")) {
          Ok(props) => update_element_props(&element, &props, is_svg),
          _ => {}
        }
        match Reflect::get(&vnode, &JsValue::from_str("children")) {
          Ok(children) if !children.is_falsy() => update_children(&element, Array::from(&children), is_svg),
          _ => {}
        }
      },
      _ => {
        let text = create_text(&to_json(&vnode));
        element.parentNode().replaceChild(&text, element);
      }
    }
  }
}


fn update_element_props(element: &Element, vnode: &JsValue, is_svg: &JsValue) {
  // let props = get(&vnode, "props");
  // if !props.is_falsy() {
  //   update_props(&element, &props, is_svg);
  // }


}
