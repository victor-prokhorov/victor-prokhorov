// import React from 'react';
// import logo from './logo.svg';
// import './App.css';
//
import React from 'react'
import logo from 'logo';
import './App.css';
import Card from './cmd';


const a = 1;
// const a = 2;
//

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

console.log({a});

export default App;
