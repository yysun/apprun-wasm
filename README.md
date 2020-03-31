## AppRun Virtual DOM Rust

This is the Virtual DOM of [AppRun](https://github.com/yysun/apprun) ported to WebAssembly using Rust.

> To use this template, you need the [Rust](https://www.rust-lang.org/tools/install) tool chain, and [wasm-pack](https://rustwasm.github.io/wasm-pack/) installed first.

### Install

```
npm i apprun-wasm
```

### Use

In JavaScript/TypeScript

```javascript
let mod;
import('../pkg').then(m => mod = m).catch(e => console.log(e));

const vdom = [...];
mod.render(document.getElementById('p'), vdom);
```

### References

#### Virtual DOM

The virtual DOM is an array that contains number, string, object, html element, svg element, and virtual node.

The virtual node is an object that has three properties: tag, props and children.

```javascript
[
  'hi',
  100,
  { a: 100 }
  document.createElement('h1'),
  {
    tag: 'div',
    props: { id: '1' },
    children: [{
      tag: 'button',
      props: { onclick: () => { }}
    }]
  },
])
```

#### Virtual DOM Rendering

The _render_ function renders the virtual DOM to an element or document.body.

In Rust

```rust
#[wasm_bindgen]
pub fn render(element: &Element, vdom: Vec<JsValue>, is_svg: &JsValue) {
  ......
}
```

Have fun!