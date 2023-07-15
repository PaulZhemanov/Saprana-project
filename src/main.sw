contract;

use std::constants::ZERO_B256;
use std::storage::storage_vec::*;
use std::call_frames::msg_asset_id; // get attached asset id func
use std::context::msg_amount; // get attached asset amount func
use std::token::transfer_to_address;
use std::block::timestamp;
use nft::{mint, owner_of};

//todo add description and image
struct Event {
    id: u64,
    owner: Address,
    name: str[50],
    // description: str[1000],
    // image: str[500],
    max_participantes: u64,
    deadline: u64, //tai64 format
    ticket_price: u64,
    tickets_sold: u64,
    balance: u64,
    //guests: 
}

configurable {
    ADMIN: Address = Address::from(ZERO_B256),
    PROTOCOL_OWNER_FEE: u64 = 10_000_000, //0.01 ETH
    PROTOCOL_BUYER_FEE: u64 = 5_000_000, //0.005 ETH
}

storage {
    events: StorageMap<u64, Event> = StorageMap {},
    //events: StorageMap<u64, StorageMap<u64, Event>> = StorageMap {},
    total_events_count: u64 = 0
}

abi NFTTicketingContract {
    #[storage(read)]
    fn verify(event_id: u64, nft_id: u64) -> bool;

    #[storage(read, write)]
    fn create_event(name: str[50], /*description: str[1000], image: str[500],*/ max_participantes: u64, deadline: u64, ticket_price: u64);
    
    #[storage(read, write)]
    fn buy_ticket(event_id: u64);
    
    #[storage(read, write)]
    fn claim(event_id: u64);
}
//configurable block
//https://fuellabs.github.io/sway/v0.42.0/book/basics/constants.html
//https://rust.fuel.network/v0.42.0/contracts/configurable-constants.html
//logs
//https://fuellabs.github.io/sway/v0.42.0/book/basics/comments_and_logging.html
//https://rust.fuel.network/v0.42.0/calling-contracts/logs.html
//identity
//https://fuellabs.github.io/sway/v0.42.0/book/basics/blockchain_types.html
//match
//https://fuellabs.github.io/sway/v0.42.0/book/basics/control_flow.html?highlight=match#match-expressions

impl NFTTicketingContract for Contract {

    #[storage(read, write)]
    fn create_event(name: str[50], max_participantes: u64, deadline: u64, ticket_price: u64){
        
        assert(ADMIN != Address::from(ZERO_B256)); // не давать создавать заказ если админ равен Address::from(ZERO_B256)
        // owner - чувак, который создает ивент
        let owner: Identity = msg_sender().unwrap();
        let owner: Address = match owner {
            Identity::Address(identity) => identity,
            _ => revert(0),
        };
        
        // Берем комиссию 0.01 ETH у owner
        let protocol_fee_asset_id: b256 = msg_asset_id().into();
        let protocol_fee_amount = msg_amount();
        assert(protocol_fee_asset_id == ZERO_B256 && protocol_fee_amount >= PROTOCOL_OWNER_FEE);
        
        // отправляем комиссию на адрес ADMIN
        transfer_to_address(protocol_fee_amount, AssetId::from(protocol_fee_asset_id), ADMIN);

        // создаем экземпляр ивента
        let id = storage.total_events_count.read();
        let new_event_instance = Event {
            id,
            owner,
            name,
            max_participantes,
            deadline,
            ticket_price,
            tickets_sold: 0,
            balance: 0
        };

        // добавляем его в сторадж
        storage.events.insert(id, new_event_instance);
        storage.total_events_count.write(id + 1);
    }
    
    #[storage(read, write)]
    fn buy_ticket(id: u64){
        //достаем ивент из storage.events по id
        let mut event = storage.events.get(id).read();

        //проверяем что денег достаточно для покупки билета и валюта ETH
        let buyer: Identity = msg_sender().unwrap();
        let buyer: Address = match buyer {
            Identity::Address(identity) => identity,
            _ => revert(0),
        };
        
        let peyment_asset_id: b256 = msg_asset_id().into();
        let payment_amount = msg_amount();
        assert(peyment_asset_id == ZERO_B256 && payment_amount == event.ticket_price);

        //проверяем что deadline > timestamp()
        assert(event.deadline >= timestamp());

        //проверяем что tickets_sold < max_participantes 
        assert(event.tickets_sold < event.max_participantes);

        //todo
        //минтим билет на адрес того кто вызвал функцию buy_ticket
        //mint();
        //transfer_to_address(1, nft_asset_id, buyer);


        //увеличиваем баланс владельца ивента
        event.tickets_sold += 1;
        event.balance += event.ticket_price;
        storage.events.insert(id, event);
    }

    #[storage(read, write)]
    fn claim(id: u64){
        let mut event = storage.events.get(id).read();
        let caller: Identity = msg_sender().unwrap();
        assert(caller == Identity::Address(event.owner));//todo use require to show an error message
        assert(event.balance > 0);
        transfer_to_address(event.balance, AssetId::from(ZERO_B256), event.owner);
        event.balance = 0;
        storage.events.insert(id, event);
    }
    
    #[storage(read)]
    fn verify(event_id: u64, nft_id: u64) -> bool{
        false
    }
  
}