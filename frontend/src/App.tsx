import './App.css';
import { useFuel } from './hooks/useFuel';
import { useIsConnected } from './hooks/useIsConnected';


function App() {
  const [fuel, notDetected] = useFuel();
  const [isConnected] = useIsConnected();
  return (
    <div className="App">
      <header className="App-header">
        <h1>nft ticketing</h1>
      </header>


      {fuel && (
        <div>
          {isConnected ? (
            <div>
              <button onClick={newPlayer}>New Player</button>
              <button onClick={levelUp}>Level Up</button>
            </div>
          ) : (
            <button onClick={() => fuel.connect()}>Connect Wallet</button>
          )}
        </div>
      )}

      {notDetected && <div>fuel NOT detected.</div>}
    </div>
  );
}

export default App;
