library;

abi NFTTicketingContract {
    #[storage(read, write)]
    fn create_event(name: str[50], /*description: str[1000], image: str[500],*/ max_participantes: u64, deadline: u64, ticket_price: u64);
    
    #[storage(read, write)]
    fn buy_ticket(event_id: u64);
    
    #[storage(read, write)]
    fn claim(event_id: u64);
    
    // #[storage(read)]
    // fn verify(event_id: u64, nft_id: u64) -> bool;
    #[storage(read)]
    fn log_values();
}