use std::{env, str::FromStr};

use dotenv::dotenv;
use fuels::{
    accounts::wallet::WalletUnlocked,
    prelude::{abigen, Bech32ContractId, CallParameters, Provider, TxParameters},
    types::ContractId,
};
abigen!(Contract(name = "DApp", abi = "out/debug/saprana-abi.json"));

const RPC: &str = "beta-3.fuel.network";
const CONTRACT_ADDRESS: &str = "0x248c0ee15d2d865de1c3ff767450c73646e17c1282f6025a0332b4c0193ac607";

//admin - owner of contract
//alice - event manager (creator of event)
//bob - participant

#[tokio::test]
async fn buy_ticket_test() {
    let event_id = 2;
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
    println!("Pre-purchase event = {:#?}", event);

    let res = buyer_instance
        .methods()
        .buy_ticket(event_id)
        .tx_params(TxParameters::default().set_gas_price(1))
        .call_params(CallParameters::default().set_amount(event.ticket_price))
        .unwrap()
        .call()
        .await
        .unwrap();

    println!("Ticket purchased. Ticket id = {:?}", res.value);

    let event = buyer_instance
        .methods()
        .get_event(event_id)
        .simulate()
        .await
        .unwrap()
        .value;
    println!("Post-purchase event = {:#?}", event);

    println!("Ticket purchased. Ticket id = {:?}", res.value);

}
