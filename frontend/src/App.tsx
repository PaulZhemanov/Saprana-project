import "./App.css";
import { useFuel } from "./hooks/useFuel";
import { useIsConnected } from "./hooks/useIsConnected"; 
import { Wallet } from "fuels";

function App() {
  const [fuel, notDetected] = useFuel();
  const [isConnected] = useIsConnected();

  const BuyTicket = () => {
    // Ваша логика покупки билета...
    console.log('Билет куплен!');
  };

  return (
    <div className="App">
      <header>
        <h1>saprana</h1>
      </header>

      {fuel && (
        <div>
          <div>fuel is detected</div>
          {isConnected ? (
            <div>
              <div>you are connected!</div>
              <button onClick={() => fuel.connect()}>Connect wallet</button>
              <button onClick={BuyTicket}>Buy ticket</button>
            </div>
          ) : (
            <button onClick={() => fuel.connect()}>Connect wallet</button>
          )}
        </div>
      )}

      {!fuel && <div>fuel not detected</div>}
    </div>
  );
}

export default App;






