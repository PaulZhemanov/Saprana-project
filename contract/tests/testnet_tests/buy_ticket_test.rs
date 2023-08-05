use std::{env, str::FromStr};

use dotenv::dotenv;
use fuels::{
    accounts::wallet::WalletUnlocked,
    prelude::{abigen, Bech32ContractId, CallParameters, Provider, TxParameters},
    types::ContractId,
};
abigen!(Contract(name = "DApp", abi = "out/debug/saprana-abi.json"));

const RPC: &str = "beta-3.fuel.network";
const CONTRACT_ADDRESS: &str = "0x010e919ebe4c15517279e2d942cdff66ef28032f59b9d4b0001839ffa5201ffb";

//admin - owner of contract
//alice - event manager (creator of event)
//bob - participant

#[tokio::test]
async fn buy_ticket_test() {
    let event_id = 1;
    dotenv().ok();

    let provider = Provider::connect(RPC).await.unwrap();

    let bob_pk = env::var("BOB").unwrap().parse().unwrap();
    let bob = WalletUnlocked::new_from_private_key(bob_pk, Some(provider.clone()));

    let contract_id: Bech32ContractId = ContractId::from_str(CONTRACT_ADDRESS).unwrap().into();
    let buyer_instance = DApp::new(contract_id, bob.clone());

    let event = buyer_instance
        .methods()
        .get_event(event_id)
        .simulate()
        .await
        .unwrap()
        .value;
    println!("event = {:#?}", event);

    let res = buyer_instance
        .methods()
        .buy_ticket(event_id)
        .tx_params(TxParameters::default().set_gas_price(1))
        .call_params(CallParameters::default().set_amount(event.ticket_price))
        .unwrap()
        .call()
        .await
        .unwrap();

    println!("Ticket id = {:?}", res.value);
}
