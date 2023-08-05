import "./App.css";
import { useFuel } from "./hooks/useFuel";
import { useIsConnected } from "./hooks/useIsConnected"; 
import { Wallet } from "fuels";
import { SapranaAbi__factory } from "./contracts";

const CONTRACT_ID = "0x010e919ebe4c15517279e2d942cdff66ef28032f59b9d4b0001839ffa5201ffb";



function App() {
  const [fuel, notDetected] = useFuel();
  const contract = SapranaAbi__factory.connect(CONTRACT_ID, fuel);
  const [isConnected] = useIsConnected();

  const BuyTicket= () => {
  };

const CreateEvent = () => {
};

  return (
    <div className="App">
      <header>
        <h1>saprana</h1>
      </header>

      {fuel && (
        <div> 
          <div>Wallet is detected</div>
          {isConnected ? (
            <div>
              <div>You are connected</div>
              <button onClick={() => fuel.connect()}>Connect wallet</button>
              <button onClick={BuyTicket}>Buy ticket</button>
              <button onClick={CreateEvent}>Сreate Event</button>{" "}
            </div>
          ) : (
            <button onClick={() => fuel.connect()}>Connect wallet</button>
          )}
        </div>
      )}

      {!fuel && <div>Wallet not detected</div>}
    </div>
  );
}

export default App;






