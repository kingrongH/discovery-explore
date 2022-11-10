mod watch;
mod registry;
mod resolver;
mod client;
mod simple_poll_discovery;

use std::{collections::{HashMap, HashSet}, sync::Arc, borrow::Cow, net::SocketAddr};

use watch::Receiver;


/// a service instance
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceInstance {
    // a unique id that represent an only instance
    instance_id: String, 
    service_name: String,
    address: SocketAddr,
    metadata: HashMap<String, String>,
}



impl ServiceInstance {

    /// get the unique id of this instance
    pub fn get_instance_id(&self) -> &str {
        &self.instance_id
    }

    /// get service name of this instance
    pub fn get_service_name(&self) -> &str {
        &self.service_name
    }

    // get address of this instance
    pub fn get_address(&self) -> &SocketAddr {
        &self.address
    }

}



/// the change of a service with name: `service_name`
#[derive(Debug, Clone)]
pub struct Change {
    pub service_name: String,
    pub all: Vec<Arc<ServiceInstance>>,
    pub added: Vec<Arc<ServiceInstance>>,
    pub removed: Vec<Arc<ServiceInstance>>,
    pub updated: Vec<Arc<ServiceInstance>>,
}

impl Change {


}


/// create change from existing instances and new instances 
pub fn create_change(service_name: &str, existing: &HashMap<String, Arc<ServiceInstance>>, new: &HashMap<String, Arc<ServiceInstance>>) -> Change {
    // existing instance id list
    let mut left_ids: HashSet<&str> = existing.keys().map(|v| v.as_str()).collect();
    
    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut updated = Vec::new();

    for instance in new.values() {
        // if existing dont has that instance
        if !existing.contains_key(instance.get_instance_id()) {
            added.push(instance.clone());
            continue;
        }

        // existing instance
        let existing_instance = existing.get(instance.get_instance_id()).unwrap();
        if existing_instance != instance {
            updated.push(instance.clone())
        }
        // remove from left ids
        left_ids.remove(instance.get_instance_id());
    }

    // what's left if left_ids is not in new, so it's removed
    for left_id in left_ids {
        // here unwrap is ok
        let removed_instance = existing.get(left_id).unwrap();
        removed.push(removed_instance.clone());
    }

    let all: Vec<Arc<ServiceInstance>> = new.values().map(|v| v.clone()).collect();
    Change {
        service_name: service_name.to_string(),
        all,
        added,
        removed,
        updated,
    }
}



trait DiscoveryClient {

    // get all service instances of service_name
    fn get_service_instances(&self, service_name: &str) -> Vec<Arc<ServiceInstance>>;

    // watch change by clone a receiver
    fn watch(&self) -> Option<Receiver>;

}

