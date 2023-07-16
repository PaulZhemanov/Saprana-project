use fuels::{prelude::*, types::ContractId};

// Load abi from json
abigen!(Contract(
    name = "MyContract",
    abi = "out/debug/saprana-abi.json"
));


#[tokio::test]
async fn main_test() {
    let config = WalletsConfig::new(Some(7), Some(1), Some(1_000_000_000));
    let mut wallets: Vec<WalletUnlocked> =
        launch_custom_provider_and_get_wallets(config, None, None).await;
    let admin = &wallets[0];
    let event_maker = &wallets[1];
    let buyer0 = &wallets[2];
    let buyer1 = &wallets[3];
    let buyer2 = &wallets[4];
    let buyer3 = &wallets[6];
    let buyer4 = &wallets[6];

    let id = Contract::load_from("./out/debug/test123.bin", LoadConfiguration::default())
        .unwrap()
        .deploy(admin, TxParameters::default())
        .await
        .unwrap();

    let instance = MyContract::new(id.clone(), admin.clone());
    // instance.create_event
    //
    //
}

