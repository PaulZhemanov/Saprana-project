extern crate alloc;
use fuel_indexer_utils::prelude::*;

#[indexer(manifest = "indexer.manifest.yaml")]
pub mod indexer_index_mod {

    fn saprana_indexer_handler(block: BlockData) {
        let height = block.height;
        info!("Height: {height}");
    }

    fn handle_event_creation(data: CreateEventLog) {
        let entry = EventEntry {
            id: data.event.id,
            owner: data.event.owner,
            name: data.event.name.to_string(),
            // description: data.event.description,
            // image: data.event.image,
            max_participantes: data.event.max_participantes,
            deadline: data.event.deadline,
            ticket_price: data.event.ticket_price,
            tickets_sold: data.event.tickets_sold,
            balance: data.event.balance,
        };
        info!("CreateEventLog {:#?}", entry);
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
        info!("BuyTicketLog {:#?}", entry);
        entry.save();
    }

    fn handle_claim(data: ClaimLog) {
        let entry = ClaimEntry {
            id: data.timestamp,
            event_id: data.event.id,
            timestamp: data.timestamp,
            amount: data.amount,
        };
        info!("ClaimLog {:#?}", entry);
        entry.save();
    }
}
