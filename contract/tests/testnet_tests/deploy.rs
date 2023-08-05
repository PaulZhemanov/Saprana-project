use std::env;

use dotenv::dotenv;
use fuels::{
    accounts::{
        fuel_crypto::rand::{rngs::StdRng, Rng, SeedableRng},
        wallet::WalletUnlocked,
    },
    prelude::{abigen, Contract, LoadConfiguration, Provider, TxParameters},
};
abigen!(Contract(name = "DApp", abi = "out/debug/saprana-abi.json"));

const RPC: &str = "beta-3.fuel.network";

//admin - owner of contract
//alice - event manager (creator of event)
//bob - participant

#[tokio::test]
async fn deploy() {
    dotenv().ok();

    let provider = Provider::connect(RPC).await.unwrap();

    let admin_pk = env::var("ADMIN").unwrap().parse().unwrap();
    let admin = WalletUnlocked::new_from_private_key(admin_pk, Some(provider.clone()));

    let protocol_fee = (0.01 * 10f64.powf(9.0)) as u64; //0.01 eth

    let configurables = DAppConfigurables::default()
        .set_ADMIN(admin.address().into())
        .set_PROTOCOL_OWNER_FEE(protocol_fee);

    let config = LoadConfiguration::default().set_configurables(configurables);
    let rng = &mut StdRng::seed_from_u64(tai64::Tai64::now().0);
    let salt: [u8; 32] = rng.gen();
    let id = Contract::load_from("./out/debug/saprana.bin", config)
        .unwrap()
        .with_salt(salt.into())
        .deploy(&admin, TxParameters::default().set_gas_price(1))
        .await
        .unwrap();

    println!("Contract deployed on beta-3");
    println!("0x..   = {:?}", id.hash().to_string());
    println!("fuel.. = {:?}", id.to_string());
}
