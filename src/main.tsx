import app from 'apprun';

let mod;
import('../pkg')
  .then(m => mod = m)
  .catch(e => console.log(e));

const test_data = {
  tag: 'div'
}

const more_data = new Array(10).fill(test_data);

const vdom = [
  0,
  100,
  true,
  false,
  'a string',
  { t: 'an object', a: 100 },
  document.createElement('h1'),
  {
    tag: 'h2',
    children: ['title 2']
  },
  {
    tag: 'section',
  },
  {
    tag: 'div',
    props: { id: '2' },
    children: [{
      tag: 'button',
      props: { onclick: () => { } },
      children: ['button 1']
    }]
  },
];

const test_js = () => {
  vdom.forEach((e, i) => {
    console.log("js test case: ", i);
    app.render(document.getElementById('p'), [e as any])
  });
  app.render(document.getElementById('p'), vdom as any)
}

const test_wasm = () => {
  vdom.forEach((e, i) => {
    console.log("wasm test case: ", i);
    mod.render(document.getElementById('p'), [e])
  });
  mod.render(document.getElementById('p'), vdom)
}

const test_js_many = () => {
  const startTime = performance.now();
  app.render(document.getElementById('p'), more_data);
  const stop = performance.now();
  console.log("js_test took " + (stop - startTime));
}

const test_wasm_many = () => {
  const startTime = performance.now();
  mod.render(document.getElementById('p'), more_data);
  const stop = performance.now();
  console.log("js_wasm took " + (stop - startTime));
}

const model = 'Hello world - AppRun !';

const view = (state) => <div >
  <h1>{state}</h1>
  <button onclick={test_js}>test js</button>
  <button onclick={test_wasm}>test wasm</button>
  <button onclick={test_js_many}>JS test x10000 divs</button>
  <button onclick={test_wasm_many}>WASM test x1000 divs</button>
  <div id="p"></div>
</div>;

const update = {
  hello: (_, state) => state
};

app.start(document.body, model, view, update);
