import { Route, Switch } from 'wouter-preact';
import { Fragment } from 'preact';

import HomePage from './HomePage';
import StreamPage from './StreamPage';

const App = () => (
  <Fragment>
    <Switch>
      <Route path="/" component={HomePage} />
      <Route path="/:streamid">
        {(params) => <StreamPage id={params.streamid} />}
      </Route>
    </Switch>
  </Fragment>
);

export default App;
