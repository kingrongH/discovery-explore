use std::{collections::{HashMap, HashSet}, sync::Arc};

use once_cell::sync::Lazy;
use parking_lot::{Mutex, RwLock};

use crate::ServiceInstance;


/// service registry
static SERVICE_REGISTRY: Lazy<RwLock<Registry>> = Lazy::new(|| {
    RwLock::new(Registry::new())
});


/// get all instances 
pub fn get_all_instances() -> Vec<Arc<ServiceInstance>> {
    let guard = SERVICE_REGISTRY.read();
    guard.get_all_instances()
}


/// get service instances by its name
pub fn get_instances(service_name: &str) -> Vec<Arc<ServiceInstance>> {
    let guard = SERVICE_REGISTRY.read();
    guard.get_instances(service_name)
}

/// update instances
pub fn update(instances: Vec<ServiceInstance>) {
    let mut guard = SERVICE_REGISTRY.write();
    guard.update(instances);
}



pub struct Registry {
    // {service_name -> {instance_id -> instance}}
    map: HashMap<String, HashMap<String, Arc<ServiceInstance>>>,
}

impl Registry {

    /// create a new service segistry
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    /// insert service instances
    pub fn update(&mut self, instances: Vec<ServiceInstance>) {
        for instance in instances {
            let instance = Arc::new(instance);
            let service_name = instance.get_service_name();
            let instances_opt = self.map.get_mut(service_name);
            match instances_opt {
                Some(v) => {
                    v.insert(instance.get_instance_id().to_string(), instance);
                },
                None => {
                    let mut id_map = HashMap::new();
                    id_map.insert(instance.get_instance_id().to_string(), instance.clone());
                    self.map.insert(service_name.to_string(), id_map);
                }
            }
        }
    }

    /// get all instances by service name
    pub fn get_instances(&self, service_name: &str) -> Vec<Arc<ServiceInstance>> {
        match self.map.get(service_name) {
            Some(id_map) => {
                id_map.values().map(|i| i.clone()).collect()
            },
            None => Vec::new()
        }
    }

    /// get all instances of this registry
    pub fn get_all_instances(&self) -> Vec<Arc<ServiceInstance>> {
        self.map.values().map(|v| {
            v.values()
        }).flatten().map(|v| {
            v.clone()
        }).collect()
    }


}






