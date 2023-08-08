import React, {useState} from 'react';
import {Button, Carousel, Layout, notification, Result, theme, Typography} from 'antd';
import "./App.css";
import {useFuel} from "./hooks/useFuel";
import {useIsConnected} from "./hooks/useIsConnected";
import {SapranaAbi__factory} from "./contracts";
import {BN} from "fuels";

const CONTRACT_ID = "0x010e919ebe4c15517279e2d942cdff66ef28032f59b9d4b0001839ffa5201ffb";

const {Header, Content, Footer} = Layout;


const contentStyle: React.CSSProperties = {
  margin: 0,
  height: '160px',
  color: '#fff',
  lineHeight: '160px',
  textAlign: 'center',
  background: '#364d79',
};

const App: React.FC = () => {
  const [ticketNumber, setTicketNumber] = useState<BN | null>(null);
  const [loading, setLoading] = useState(false);
  const {token: {colorBgContainer}} = theme.useToken();
  const [fuel, error, fuelLoading] = useFuel();
  const [isConnected] = useIsConnected();
  const ticketPrice = 0.01 * 1e9;
  const buyTicket = () => {
    setLoading(true);
    fuel.currentAccount()
        .then(account => fuel.getWallet(account))
        .then(wallet => SapranaAbi__factory.connect(CONTRACT_ID, wallet))
        .then(contract => contract.functions.buy_ticket(1)
            .txParams({gasPrice: 1})
            .callParams({forward: {amount: ticketPrice}})
            .call()
        ).then((res) => setTicketNumber(res.value))
        .catch(e => {
          console.log(e)
          notification.error({message: e.toString()})
        }).finally(() => setLoading(false))
  }

  if (ticketNumber != null) {
    return <Result
        status="success"
        title="Successfully Purchased Saprana test event ticket!"
        subTitle={`Ticket number: ${ticketNumber.toString()}. You can go to check contract code now`}
        extra={[
          <Button onClick={() => window.open("https://github.com/PaulZhemanov/Saprana-project")} type="primary">
            Go check GitHub
          </Button>
        ]}
    />
  }

  return (
      <Layout style={{height: '100vh'}}>
        <Header
            style={{
              position: 'sticky',
              top: 0,
              zIndex: 1,
              width: '100%',
              display: 'flex',
              alignItems: 'center',
              justifyContent: "space-between"
            }}
        >
          <Typography.Title level={3} style={{color: 'white', margin: 0}}>🥳 SAPRANA</Typography.Title>
          <div>
            {isConnected ? (
                <div style={{color: 'white'}}>You are connected</div>
            ) : (
                <Button disabled={fuel == null || fuelLoading} onClick={() => fuel.connect()}>Connect wallet</Button>
            )}
          </div>
        </Header>
        <Content className="site-layout" style={{padding: '0 50px'}}>
          <div style={{height: 32}}>{error}</div>
          <div style={{padding: 24, minHeight: 380, background: colorBgContainer}}>
            <Carousel>
              <div><h3 style={contentStyle}>Saprana test event</h3></div>
              <div><h3 style={contentStyle}>Saprana test event pic1</h3></div>
              <div><h3 style={contentStyle}>Saprana test event pic2</h3></div>
              <div><h3 style={contentStyle}>Saprana test event pic3</h3></div>
            </Carousel>
            <Typography.Title level={4}>August 31th</Typography.Title>
            <Typography.Title>Saprana test event</Typography.Title>
            <Typography>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
              incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation
              ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit
              in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
              cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est
              laborum.</Typography>
            <div style={{height: 32}}/>
            <Button type="primary" size="large" disabled={loading || !isConnected}
                    onClick={buyTicket}>{loading ? "Loading..." : "Buy ticket for 0.01 ETH"}</Button>
          </div>
        </Content>
        <Footer style={{textAlign: 'center'}}>Saprana nft ticketing platform ©2023 <a
            href="https://github.com/PaulZhemanov/Saprana-project">GitHub</a></Footer>
      </Layout>
  );
};

export default App;

