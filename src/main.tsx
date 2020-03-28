import app from 'apprun';
import('../pkg').catch(e => console.log(e));

const model = 'Hello world - AppRun !';

const view = (state) => <div>
  <h1>{state}</h1>
</div>;

const update = {
  hello: (_, state) => state
};

app.start(document.body, model, view, update);
