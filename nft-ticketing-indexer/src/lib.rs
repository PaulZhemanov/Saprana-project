extern crate alloc;
use fuel_indexer_utils::prelude::*;

#[indexer(manifest = "nft_ticketing_indexer.manifest.yaml")]
pub mod nft_ticketing_indexer_index_mod {

    fn handle_event_creation(data: CreateEventLog) {
        let entry = EventEntry {
            id: data.event.id,
            owner: data.event.owner,
            // name: data.event.name,
            // description: data.event.description,
            // image: data.event.image,
            max_participantes: data.event.max_participantes,
            deadline: data.event.deadline,
            ticket_price: data.event.ticket_price,
            tickets_sold: data.event.tickets_sold,
            balance: data.event.balance,
        };
        entry.save();
    }


    fn handle_buy_ticket(data: BuyTicketLog) {
        let entry = BuyTicketEntry {
            id: data.timestamp,
            event_id: data.event.id,
            timestamp: data.timestamp,
            buyer: data.buyer,
            ticket_id: data.ticket_id,
            ticket_number: data.ticket_number,
        };
        entry.save();
    }


    fn handle_claim(data: ClaimLog) {
        let entry = ClaimEntry {
            id: data.timestamp,
            event_id: data.event.id,
            timestamp: data.timestamp,
            amount: data.amount,
        };
        entry.save();
    }
}
