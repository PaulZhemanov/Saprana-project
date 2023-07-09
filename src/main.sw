contract;

use std::constants::ZERO_B256;
use std::storage::storage_vec::*;


struct Event {
    name: str[50],
    description: str[1000],
    image: str[500],
    max_participantes: u64,
    event_date: u64,
    ticket_price: u64,
    tickets_sold: u64,
    balance: u64,
    owner: Address
}


// class Address{
//     constructor(address: b256){
//         ...
//     }
// }

// const storage = {
//     contract_admin: Address = new Address(ZERO_B256),
//     events: Array<Event> = []
// }

storage {
     contract_admin: Address = Address::from(ZERO_B256),
     events: StorageVec<Event> = StorageVec {},
}

abi NFTTicketingContract {

    #[storage(read)]
    fn verify(event_id: u64, nft_id: u64) -> bool;

    #[storage(read, write)]
    fn create_event(name: str[50], description: str[1000], image: str[500], max_participantes: u64, event_date: u64, ticket_price: u64);
    fn buy_ticket(event_id: u64);
    fn claim(event_id: u64);
}
