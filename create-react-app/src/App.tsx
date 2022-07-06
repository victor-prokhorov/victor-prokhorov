import React from 'react';
import './App.css';
import Card from './Card';
import som from './index.tsx';

const a = {};

function ok() {}
// this is a comment
function App() {
  <Card />;
  return (
    <div className="App">
      <header className="App-header">
        <p>
          Edit
          {' '}
          <code>src/App.tsx</code>
          {' '}
          and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
          onClick={ok}
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

console.log({ a });

export default App;
