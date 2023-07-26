use chrono::{Days, Utc};
use fuels::{prelude::*, types::SizedAsciiString};
use tai64::Tai64;

// Load abi from json
abigen!(Contract(name = "DApp", abi = "out/debug/saprana-abi.json"));

#[tokio::test]
async fn main_test() {
    let config = WalletsConfig::new(Some(7), Some(1), Some(1_000_000_000));
    let wallets: Vec<WalletUnlocked> =
        launch_custom_provider_and_get_wallets(config, None, None).await;
    let admin = &wallets[0];
    let event_maker = &wallets[1];
    // let buyer0 = &wallets[2];
    // let buyer1 = &wallets[3];
    // let buyer2 = &wallets[4];
    // let buyer3 = &wallets[6];
    // let buyer4 = &wallets[6];

    let protocol_fee = (0.01 * 10f64.powf(9.0)) as u64; //0.01 eth
    let configurables = DAppConfigurables::default()
        .set_ADMIN(admin.address().into())
        .set_PROTOCOL_OWNER_FEE(protocol_fee);
    let config = LoadConfiguration::default().set_configurables(configurables);
    let id = Contract::load_from("./out/debug/saprana.bin", config)
        .unwrap()
        .deploy(admin, TxParameters::default())
        .await
        .unwrap();

    let admin_instance = DApp::new(id.clone(), admin.clone());

    let mut name: String = "Test event".into();
    name.push_str(" ".repeat(50 - name.len()).as_str());
    let name = SizedAsciiString::<50>::new(name).unwrap();
    let price = 0.01 * 10f64.powf(9.0); //0.01 ETH
    let price1: u64 = 10_000_000;

    // println!("id = {:?}", id);
    // println!("contract address = {:?}", contract_instance.contract_id());
    let event_maker_instance = admin_instance.with_account(event_maker.clone()).unwrap();
    let days = Days::new(1);
    let timestamp = Utc::now().checked_add_days(days).unwrap().timestamp();
    let tai64_timestamp = Tai64::from_unix(timestamp).0;

    let res = event_maker_instance
        .methods()
        .create_event(name, 5, tai64_timestamp, price as u64)
        .tx_params(TxParameters::default().set_gas_price(1))
        .call_params(CallParameters::default().set_amount(protocol_fee))
        .unwrap()
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    let event_id = res.value;
    println!("event id = {event_id}");

    let res = event_maker_instance
        .methods()
        .get_event(event_id)
        .simulate()
        .await
        .unwrap();
    // println!("event = {:#?}", res.value);

    let res = event_maker_instance
        .methods()
        .buy_ticket(event_id)
        .tx_params(TxParameters::default().set_gas_price(1))
        .call_params(CallParameters::default().set_amount(price1))
        .unwrap()
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    // println!("buy = {:#?}", res);

    // let res = methods.log_values().call().await.unwrap();
    // println!("logs = {:?}", res.decode_logs());
    // println!("u64 logs = {:?}", res.decode_logs_with_type::<u64>());

    // instance.create_event
    //
    //
}

// let str_var = "Test event";
// let var0 = str_var.to_owned();
// let var1 = str_var.to_string();
// let string_var = String::from("Test event");
// let var2 = string_var.as_str();
