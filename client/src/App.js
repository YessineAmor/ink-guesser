import React from 'react';
import logo from './logo.svg';
import './App.css';
import { ApiPromise, WsProvider } from '@polkadot/api';

function App() {
  let contractBytes;
  fetch('guesserv3.wasm').then(async (res) => {
    const buffer = await res.arrayBuffer();
    console.log(buffer);
    contractBytes = buffer;
  });
  const wsProvider = new WsProvider('ws://127.0.0.1:9944');
  ApiPromise.create({ provider: wsProvider }).then((api) => {
    console.log('Connected to substrate node ' + api.genesisHash.toHex());
    console.log(
      'Putting contract code ' + api.tx.contracts.putCode(contractBytes)
    );
  });

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
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

export default App;
