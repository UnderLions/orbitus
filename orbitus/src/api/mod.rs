use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct AtomicDB {
    hm : Arc<Mutex<Box<HashMap<String,String>>>>
}


pub trait CRUD {
    type Hw;
    fn new() -> Self;
    fn create(&self, username :String , passwd : String ) -> Option<String>;
}

pub trait UserAction {
    fn verify(&self, username:&str, passwd : &str) -> bool;
}

impl CRUD for AtomicDB {
    type Hw = Box<HashMap<String,String>>;
    fn new() -> Self {
        Self {
            hm : Arc::new(Mutex::new(Box::new(HashMap::new())))
        }
    }

    fn create(&self, username :String , passwd : String ) -> Option<String> {
        let binding = &self.hm;
        let mut map = binding.lock().unwrap();
        map.insert(username, passwd)
    }
}

impl UserAction for AtomicDB {
    fn verify(&self, username: &str, passwd : &str) -> bool {
        let bindings = &self.hm.clone();
        let map = bindings.lock().unwrap();
        match map.get(username) {
            Some(passwd1) => { passwd1 == passwd },
            None => { false }
        }
    }
}
