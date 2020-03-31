
use std::cmp;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::Reflect;
use js_sys::Array;
use js_sys::JSON;
use web_sys::HtmlElement;
use web_sys::SvgElement;
use web_sys::Element;
use web_sys::Node;

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
}

fn arr_get(target: &JsValue, idx: &u32) -> JsValue {
  Reflect::get(target, &JsValue::from_f64(*idx as f64)).unwrap()
}

#[wasm_bindgen]
pub fn render(element: &HtmlElement, vdom: Array, is_svg: &JsValue) {
  update_children(element, vdom, is_svg);
}

fn window() -> web_sys::Window {
  web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
  window().document().expect("should have a document on window")
}

fn update_children (element: &Node, children: Array, is_svg: &JsValue) {
  // log_str("vdom: update element children:");

  let el_nodes = element.child_nodes();
  let old_len = el_nodes.length();
  let new_len = children.length();
  let len = cmp::min(old_len, new_len);

  // update
  for i in 0..len {
    let old_el = arr_get(&el_nodes, &i);
    let new_el = arr_get(&children, &i);

    if new_el.is_instance_of::<HtmlElement>() || new_el.is_instance_of::<SvgElement>() {
      let el = &Node::from(old_el);
      let new_node = &new_el.dyn_ref::<Node>().unwrap();
      element.insert_before(new_node, Some(el));
      break;
    }

    //Todo
    // let key = get(&ch, "key");
    // if !key.is_falsy() {
    //   break;
    // }

    let el = &Element::from(old_el);
    update_element(el, new_el, is_svg);
  }

  let mut n = element.child_nodes().length();
  while n > len {
    element.remove_child(&element.last_child().unwrap());
    n = n - 1;
  }

  if new_len > len {
    let doc_fragment = document().create_document_fragment();
    for i in len..new_len {
      let vnode = arr_get(&children, &i);
      let new_element = create_element(vnode, is_svg);
      doc_fragment.append_child(&new_element);

    }
    element.append_child(&doc_fragment);
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
  let tt = text.as_string().unwrap_or_default();
  let txt = document().create_text_node(tt.as_str());
  Element::from(JsValue::from(txt))
}

fn create_element(vnode: JsValue, is_svg: &JsValue) -> Element {
  // log_str("vdom: create element:");

  if vnode.is_string() {
    return create_text(&vnode)
  } else if vnode.is_instance_of::<HtmlElement>() || vnode.is_instance_of::<SvgElement>() {
    return Element::from(vnode)
  }
  match Reflect::get(&vnode, &JsValue::from_str("tag")) {
    Ok(tag) if !&tag.is_falsy() => {
      let element;
      let tag_str = tag.as_string().unwrap_or_default();
      if !is_svg.is_falsy() || tag == JsValue::from_str("svg") {
        element = document().create_element_ns(Some("http://www.w3.org/2000/svg"), tag_str.as_str()).unwrap();
      } else {
        element = document().create_element(tag_str.as_str()).unwrap();
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
    _ => create_text(&JSON::stringify(&vnode).unwrap())
  }
}

fn update_element(element: &Element, vnode: JsValue, is_svg: &JsValue) {
  // log_str("vdom: update element:");

  if vnode.is_string() {
    let old_text = element.text_content().unwrap_or_default();
    let new_text = vnode.as_string().unwrap_or_default();
    if old_text != new_text {
      if element.node_type() == 3 {
        element.set_text_content(Some(new_text.as_str()));
      } else {
        let text = create_text(&vnode);
        element.parent_node().unwrap().replace_child(&text, element);
      }
    }
  } else if vnode.is_instance_of::<HtmlElement>() || vnode.is_instance_of::<SvgElement>() {
    let new_node = vnode.dyn_ref().unwrap();
    element.parent_node().unwrap().replace_child(new_node, element);
  } else {

    match Reflect::get(&vnode, &JsValue::from_str("tag")) {
      Ok(tag) if !&tag.is_falsy() => {

        match Reflect::get(element, &JsValue::from_str("tagName")) {
          Ok(tag_name) if tag_name.is_undefined() |
            (tag_name.as_string().unwrap_or_default() !=
            tag.as_string().unwrap_or_default().to_ascii_uppercase()) => {
            let node = &create_element(vnode, is_svg);
              element.parent_node().unwrap().replace_child(node, element);
              return;
          },
          _ => {}
        }

        //let isSvg = is_svg || tag == "svg";

        match Reflect::get(&vnode, &JsValue::from_str("props")) {
          Ok(props) => update_element_props(&element, &props, is_svg),
          _ => {}
        }
        match Reflect::get(&vnode, &JsValue::from_str("children")) {
          Ok(children) if !children.is_falsy() => update_children(&element, Array::from(&children), is_svg),
          _ => { update_children(&element, Array::new(), is_svg) }
        }
      },
      _ => {
        let text = &create_text(&JSON::stringify(&vnode).unwrap());
        element.parent_node().unwrap().replace_child(text, element);
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
