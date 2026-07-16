// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender, SyncSender, TrySendError};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    capacity: usize,
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, ()> {
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(self.capacity);
        let result = self.sender.try_send(Command::Insert {
            draft,
            response_channel: response_sender,
        });
        match result {
            Ok(()) => Ok(response_receiver.recv().unwrap()),
            Err(err) => panic!("{}", err),
        }
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, ()> {
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(self.capacity);
        let result = self.sender.try_send(Command::Get {
            id,
            response_channel: response_sender,
        });
        match result {
            Ok(()) => Ok(response_receiver.recv().unwrap()),
            Err(err) => panic!("{}", err),
        }
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { capacity, sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                response_channel.try_send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id).cloned();
                response_channel.try_send(ticket);
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
