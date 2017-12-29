use datatypes::user;
use datatypes::source;
use datatypes::destination;
use datatypes::map;
use service::tick_outcome;
use storage::storage_utility;

pub struct bot {
    pub commands: Vec<fn( msg: String, user: source, storage_system: storage_utility) -> Vec<(destination , String)> >,
    pub tick_functions: Vec<fn() -> (tick_outcome)>,    
    pub storage_adapter: Box<storage_utility>
}


