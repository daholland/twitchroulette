import 'tailwindcss/tailwind.css';

import { render } from 'preact';
import App from './App';

let el = document.getElementById('root');

if (el) {
  render(<App />, el);
}
