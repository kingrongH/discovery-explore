use std::{net::{SocketAddr, ToSocketAddrs}, collections::HashMap};

use once_cell::sync::Lazy;
use parking_lot::RwLock;

use crate::Change;



/// global transport manager
static TRANSPORT_MANAGER: Lazy<RwLock<TransportManager>> = Lazy::new(|| {
    RwLock::new(TransportManager::new())
});


/// resolve from change
pub async fn resolve(change: &Change) {
    let mut guard = TRANSPORT_MANAGER.write();

    // removed 
    for instance in &change.removed {
        guard.remove(instance.get_service_name(), instance.get_instance_id());
    }

    // added
    for instance in &change.added {
        let address = instance.address;
        let mut transport = Transport::new(address);
        transport.connect().await;
        // insert into manager
        guard.insert(instance.get_service_name(), instance.get_instance_id(), transport);
    }

    // TODO updated

}



/// a transport represent
#[derive(Clone)]
pub struct Transport {
    address: SocketAddr,
    state: State
}

impl Transport {

    pub fn new(address: SocketAddr) -> Self {
        Self {
            address,
            state: State::Init
        }
    }

    /// start a connect
    pub async fn connect(&mut self) {
        // but actually we do nothing here
        println!("address: {} connected", self.address);
        self.state = State::Connected;
    }

}

#[derive(Debug, Clone)]
pub enum State {
    Init,
    Connected
}



pub struct TransportManager {
    // {service_name -> {instance_id -> Transport}}
    map: HashMap<String, HashMap<String, Transport>>
}

impl TransportManager {

    /// create a new TransportManager
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    /// get transports by service name
    pub fn get_transports(&self, service_name: &str) -> Option<&HashMap<String, Transport>> {
        self.map.get(service_name)
    }

    /// remove
    pub fn remove(&mut self, service_name: &str, instance_id: &str) {
        let opt = self.map.get_mut(service_name);
        match opt {
            Some(v) => {
                v.remove(instance_id);
            },
            None => ()
        }
    } 

    /// insert
    pub fn insert(&mut self, service_name: &str, instance_id: &str, transport: Transport) {
        let opt = self.map.get_mut(service_name);
        match opt {
            Some(v) => {
                v.insert(instance_id.to_string(), transport);
            },
            None => {
                let mut instance_transport = HashMap::new();
                instance_transport.insert(instance_id.to_string(), transport);
                self.map.insert(service_name.to_string(), instance_transport);
            }
        }
    }


}


