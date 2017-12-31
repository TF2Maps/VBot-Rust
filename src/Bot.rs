use datatypes::user;
use datatypes::source;
use datatypes::destination;
use datatypes::map;
use service::tick_outcome;
use storage::storage_utility;

pub struct bot {
    pub commands: Vec<fn( msg: String, user: source, storage_system: storage_utility) -> Vec<(destination , String)> >,
    pub begin_loop: fn(Vec<fn( msg: String, user: source, storage_system: storage_utility) -> Vec<(destination , String)>>, Box<storage_utility>),    
    pub storage_adapter: Box<storage_utility>
}


