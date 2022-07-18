import { Fragment, h, hydrate } from 'preact';
import { useEffect, useState } from 'preact/hooks';

const App = () => {
  const [loaded, setLoaded] = useState(false);

  useEffect(() => {
    setTimeout(() => {
      setLoaded(true);
    }, 10000);
  }, []);

  return <Fragment>{loaded && <div>Loaded</div>}</Fragment>;
};

hydrate(<App />, document.getElementById('root'));
