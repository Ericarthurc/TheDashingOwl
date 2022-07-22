import 'preact/debug';
import { Fragment, h, render } from 'preact';
import { useEffect, useState } from 'preact/hooks';

import LoginForm from './components/loginForm';

const App = () => {
  const [loaded, setLoaded] = useState(false);

  const loadedHandler = (state: boolean) => {
    setLoaded(state);
  };

  return (
    <Fragment>
      {!loaded && (
        <div>
          <LoginForm loadedHandler={loadedHandler} />
        </div>
      )}
      {loaded && (
        <div>
          <h1>Admin Page</h1>
        </div>
      )}
    </Fragment>
  );
};

render(<App />, document.getElementById('root'));
