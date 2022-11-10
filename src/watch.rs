
use crate::Change;


// since we only care about the latest change
const CAPACITY: usize = 1usize;

/// create channel for listen & send `Change`
pub fn create() -> (Sender, Receiver) {
    let (mut sender, receiver) = async_broadcast::broadcast(CAPACITY);
    // we only need the lastest change in the queue
    sender.set_overflow(false);
    let sender = Sender {
        sender
    };
    let receiver = Receiver {
        receiver
    };
    (sender, receiver)
}



/// a wrap for broadcast channel sender of change
#[derive(Debug, Clone)]
pub struct Sender {
    sender: async_broadcast::Sender<Change>
}


impl Sender {

    /// send a change to the receivers
    pub fn send(&self, change: Change) {
        // TODO handle the result, but since we dont overflow, and we dont expose the close method, so this should not fail actually
        let result = self.sender.try_broadcast(change);
        match result {
            Err(_) => {

            },
            _ => {
            
            }
        }
    }

}


/// a wrap for broadcast channel receiver of change
#[derive(Debug, Clone)]
pub struct Receiver {
    receiver: async_broadcast::Receiver<Change>
}

impl Receiver {

    /// try receiver change from channel
    pub fn recv(&mut self) -> Option<Change> {
        let result = self.receiver.try_recv();
        // we dont have closed error,so it's ok to return None, if its not Ok
        match result {
            Ok(v) => Some(v),
            _ => None,
        }
    }

}

