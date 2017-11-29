use datatypes::user;
use datatypes::source;
use datatypes::destination;
use datatypes::map;
use service::tick_outcome;
use storage::storage;

pub struct bot {
    pub commands: Vec<fn( msg: String, user: source) -> Vec<(destination,String)> >,
    pub tick_functions: Vec<fn() -> (tick_outcome)>,    
    pub storage_adapter: Box<storage>
}


