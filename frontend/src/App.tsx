import "./App.css";
import { useFuel } from "./hooks/useFuel";
import { useIsConnected } from "./hooks/useIsConnected";
import { SapranaAbi__factory } from "./contracts";

const CONTRACT_ID =
  "0x010e919ebe4c15517279e2d942cdff66ef28032f59b9d4b0001839ffa5201ffb";

function App() {
  const [fuel] = useFuel();
  const [isConnected] = useIsConnected();
  const ticketPrice = 0.01 * 1e9;
  const buyTicket = () =>
    fuel
      .currentAccount()
      .then((account) => fuel.getWallet(account))
      .then((wallet) => SapranaAbi__factory.connect(CONTRACT_ID, wallet))
      .then((contract) =>
        contract.functions
          .buy_ticket(1)
          .txParams({ gasPrice: 1 })
          .callParams({ forward: { amount: ticketPrice } })
          .call()
      );

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
              <button onClick={buyTicket}>Buy ticket</button>
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
