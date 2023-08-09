contract;

use std::constants::ZERO_B256;
// use std::storage::storage_vec::*;
use std::auth::msg_sender;
use std::call_frames::msg_asset_id; // get attached asset id func
use std::context::msg_amount; // get attached asset amount func
use std::token::transfer_to_address;
use std::block::timestamp;
use std::logging::log;
use nft::{mint, transfer, owner_of, tokens_minted};
use nft::extensions::token_metadata::*;

abi NFTTicketingContract {
    #[storage(read, write), payable]
    fn create_event(name: str[50], /*description: str[1000], image: str[500],*/ max_participantes: u64, deadline: u64, ticket_price: u64) -> u64;
    
    #[storage(read, write), payable]
    fn buy_ticket(event_id: u64) -> u64;
    
    #[storage(read, write)]
    fn claim(event_id: u64) -> u64;
    
    #[storage(read)]
    fn verify(owner: Address, event_id: u64, nft_id: u64) -> bool;

    #[storage(read)]
    fn get_event(id: u64) -> Event;
}
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
}

struct TicketMetadata{
    event_id: u64
}

enum Error {
    AddressAlreadyMint: (),
    MintIsClosed: (),
    NotOwner: (),
    SorryButSoldOut: ()
}

configurable {
    // ADMIN: Address = Address::from(ZERO_B256),
    ADMIN: Address = Address::from(0x0000000000000000000000000000000000000000000000000000000000000000),
    PROTOCOL_OWNER_FEE: u64 = 10_000_000, //0.01 ETH
}

storage {
    events: StorageMap<u64, Event> = StorageMap {},
    total_events_count: u64 = 0
}

struct CreateEventLog {
    event: Event,
    timestamp: u64,
    payment: u64
}

struct BuyTicketLog {
    event: Event,
    timestamp: u64,
    buyer: Identity,
    ticket_id: u64,
    ticket_number: u64
}

struct ClaimLog{
    event: Event,
    timestamp: u64,
    amount: u64
}


impl NFTTicketingContract for Contract {

    #[storage(read, write), payable]
    fn create_event(name: str[50], max_participantes: u64, deadline: u64, ticket_price: u64)-> u64 {
        
        assert(ADMIN != Address::from(ZERO_B256)); // не давать создавать заказ если админ равен Address::from(ZERO_B256)
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
        // let id = storage.total_events_count.try_read().unwrap_or(0);
        let id = storage.total_events_count;
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
        // storage.total_events_count.write(id + 1);
        storage.total_events_count = id + 1;
        log(CreateEventLog{
            event: new_event_instance,
            timestamp: timestamp(),
            payment: protocol_fee_amount,
        });
        return id;
    }
    
    #[storage(read, write), payable]
    fn buy_ticket(id: u64) -> u64 {
        //достаем ивент из storage.events по id
        // let mut event = storage.events.get(id).read();
        let mut event = storage.events.get(id).unwrap();

        let buyer: Identity = msg_sender().unwrap();
        //проверяем что денег достаточно для покупки билета и валюта ETH
        let payment_asset_id: b256 = msg_asset_id().into();
        let payment_amount = msg_amount();
        assert(payment_asset_id == ZERO_B256 && payment_amount == event.ticket_price);

        //проверяем что deadline > timestamp()
        require(event.deadline >= timestamp(), Error::MintIsClosed);

        //проверяем что tickets_sold < max_participantes 
        require(event.tickets_sold < event.max_participantes, Error::SorryButSoldOut);

        //минтим билет на адрес того кто вызвал функцию buy_ticket
        let ticket_id = tokens_minted();
        mint(1, buyer);
        let metadata = TicketMetadata{event_id: id};
        set_token_metadata(Option::Some(metadata), ticket_id);

        //увеличиваем баланс владельца ивента
        event.tickets_sold += 1;
        event.balance += event.ticket_price;
        storage.events.insert(id, event);
        log(BuyTicketLog{
            event,
            buyer,
            timestamp: timestamp(),
            ticket_id,
            ticket_number: event.tickets_sold 
        });
        ticket_id
    }

    #[storage(read, write)]
    fn claim(id: u64) -> u64 {
        // let mut event = storage.events.get(id).read();
        let mut event = storage.events.get(id).unwrap();
        let caller: Identity = msg_sender().unwrap();
        require(caller == Identity::Address(event.owner), Error::NotOwner);
        if event.balance == 0 {
            return 0;
        }
        transfer_to_address(event.balance, AssetId::from(ZERO_B256), event.owner);
        let claim_log = ClaimLog{
            event,
            timestamp: timestamp(),
            amount: event.balance
        };
        log(claim_log);
        event.balance = 0;
        storage.events.insert(id, event);
        claim_log.amount
    }
    
    #[storage(read)]
    fn verify(ticket_owner: Address, event_id: u64, token_id: u64) -> bool {
        owner_of(token_id).unwrap() == Identity::Address(ticket_owner) && token_metadata::<TicketMetadata>(token_id).unwrap().event_id == event_id
    }
  
    #[storage(read)]
    fn get_event(id: u64) -> Event {
        // storage.events.get(id).read()
        storage.events.get(id).unwrap()
    }
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