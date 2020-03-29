import app from 'apprun';

let mod;
import('../pkg')
  .then(m => mod = m)
  .catch(e => console.log(e));


const test_data = {
  tag: 'div',
  // props: { id: '1' },
  // children: [{
  //   tag: 'button',
  //   props: { onclick: () => { }}
  // }]
}

const more_data = new Array(10000).fill(test_data);

const test = () => mod.render(document.getElementById('p'), [
  'hi',
  document.createElement('h1'),
  {
    tag: 'section',
    props: { id: '1' },
    children: [{
      tag: 'p',
      props: { style: {color: 'red'} }

    }]
  },
  {
    tag: 'div',
    props: { id: '1' },
    children: [{
      tag: 'button',
      props: { onclick: () => { }}
    }]
  },
  100,
  { a: 100 }
])

const test_js = () => {
  const startTime = performance.now();
  app.render(document.getElementById('p'), more_data);
  const stop = performance.now();
  console.log("js_test took " + (stop - startTime));
}

const test_wasm = () => {
  const startTime = performance.now();
  mod.render(document.getElementById('p'), more_data);
  const stop = performance.now();
  console.log("js_wasm took " + (stop - startTime));
}

const model = 'Hello world - AppRun !';

const view = (state) => <div >
  <h1>{state}</h1>
  <button onclick={test}>simple test</button>
  <button onclick={test_js}>JS test x10000 divs</button>
  <button onclick={test_wasm}>WASM test x1000 divs</button>
  <div id="p"></div>
</div>;

const update = {
  hello: (_, state) => state
};

app.start(document.body, model, view, update);
