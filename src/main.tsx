import app from 'apprun';

let mod;
import('../pkg')
  .then(m => mod = m)
  .catch(e => console.log(e));

const test = () => mod.render(document.getElementById('p'), [{
  tag: 'div',
  props: { id: '1' },
  children: []
}])


const model = 'Hello world - AppRun !';

const view = (state) => <div id="p" onclick={test}>
  <h1>{state}</h1>
</div>;

const update = {
  hello: (_, state) => state
};

app.start(document.body, model, view, update);
