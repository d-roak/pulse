use crate::node::{PeerId, Key};


pub enum Message {
    Ping,
    Store,
    FindNode(PeerId),
    FindValue(Key),
}

pub fn handle_message(message: Message) {
    match message {
        Message::Ping => {
            // Probes a node to see if it is online
        },
        Message::Store => {
            // Instructs to store a <Key, Value> pair for retrieval
        },
        Message::FindNode(_id) => {
            // Input 160 bit ID as an argument
            // Returns Peer for the k nodes it knows about closest to the target ID
            // It must return k nodes, unless it has less than k in all buckets
        },
        Message::FindValue(_id) => {
            // It behaves like FindNode
            // If it has the key stored, returns value
        },
    }
}

pub fn send_message() {
}
