use std::{env, str::FromStr};

use chrono::{Duration, Utc};
use dotenv::dotenv;
use fuels::{
    accounts::wallet::WalletUnlocked,
    prelude::{abigen, Bech32ContractId, CallParameters, Provider, TxParameters},
    types::{ContractId, SizedAsciiString},
};
use tai64::Tai64;
abigen!(Contract(name = "DApp", abi = "out/debug/saprana-abi.json"));

const RPC: &str = "beta-3.fuel.network";
const CONTRACT_ADDRESS: &str = "0x248c0ee15d2d865de1c3ff767450c73646e17c1282f6025a0332b4c0193ac607";

//admin - owner of contract
//alice - event manager (creator of event)
//bob - participant

#[tokio::test]
async fn create_event_test() {
    let mut name: String = "Test event".into();
    let mut description: String = "hi guys go puff.".into();
    let max_guests = 100;
    let price = 0.01 * 10f64.powf(9.0); //0.01 ETH
    let in_days = Duration::days(30);
    let protocol_fee = (0.01 * 10f64.powf(9.0)) as u64; //0.01 eth
    dotenv().ok();

    let provider = Provider::connect(RPC).await.unwrap();

    let alice_pk = env::var("ALICE").unwrap().parse().unwrap();
    let alice = WalletUnlocked::new_from_private_key(alice_pk, Some(provider.clone()));

    let contract_id: Bech32ContractId = ContractId::from_str(CONTRACT_ADDRESS).unwrap().into();
    let event_maker_instance = DApp::new(contract_id, alice.clone());

    name.push_str(" ".repeat(50 - name.len()).as_str());
    let name = SizedAsciiString::<50>::new(name).unwrap();
    description.push_str(" ".repeat(50 - description.len()).as_str());
    let description = SizedAsciiString::<50>::new(description).unwrap();

    let deadline = Tai64::from_unix(Utc::now().checked_add_signed(in_days).unwrap().timestamp()).0;

    let res = event_maker_instance
        .methods()
        .create_event(name, description, max_guests, deadline, price as u64)
        .tx_params(TxParameters::default().set_gas_price(1))
        .call_params(CallParameters::default().set_amount(protocol_fee))
        .unwrap()
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    println!("Created event id = {:?}", res.value);

    let event = event_maker_instance
        .methods()
        .get_event(res.value)
        .simulate()
        .await
        .unwrap()
        .value;
    println!("Event info  = {:#?}", event);
}
