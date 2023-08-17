use std::{env, str::FromStr};

use dotenv::dotenv;
use fuels::{
    accounts::wallet::WalletUnlocked,
    prelude::{abigen, Bech32ContractId, Provider, TxParameters},
    types::ContractId,
};
abigen!(Contract(name = "DApp", abi = "out/debug/saprana-abi.json"));

const RPC: &str = "beta-3.fuel.network";
const CONTRACT_ADDRESS: &str = "0x248c0ee15d2d865de1c3ff767450c73646e17c1282f6025a0332b4c0193ac607";

//admin - owner of contract
//alice - event manager (creator of event)
//bob - participant

#[tokio::test]
async fn claim_test() {
    let event_id = 0;
    dotenv().ok();

    let provider = Provider::connect(RPC).await.unwrap();

    let alice_pk = env::var("ALICE").unwrap().parse().unwrap();
    let alice = WalletUnlocked::new_from_private_key(alice_pk, Some(provider.clone()));

    let contract_id: Bech32ContractId = ContractId::from_str(CONTRACT_ADDRESS).unwrap().into();
    let event_maker_instance = DApp::new(contract_id, alice.clone());
    let res = event_maker_instance
        .methods()
        .claim(event_id)
        .tx_params(TxParameters::default().set_gas_price(1))
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    println!("Claimed = {:?} ETH", res.value as f64 / 10f64.powf(9f64));
}
