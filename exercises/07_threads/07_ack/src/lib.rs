use crate::{
    data::{Ticket, TicketDraft},
    store::{TicketId, TicketStore},
};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert {
        draft: TicketDraft,
        response_sender: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_sender: Sender<Option<Ticket>>,
    },
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_sender,
            }) => {
                let ticket_id = store.add_ticket(draft);
                response_sender.send(ticket_id);
            }
            Ok(Command::Get {
                id,
                response_sender,
            }) => {
                let ticket = match store.get(id) {
                    Some(ticket) => Some(ticket.clone()),
                    None => None,
                };
                // This one line replaces your entire 5-line match block
                // let ticket = store.get(id).cloned();
                response_sender.send(ticket);
            }

            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
