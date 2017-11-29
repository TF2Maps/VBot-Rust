use datatypes::user;
use datatypes::source;
use datatypes::destination;
use datatypes::map;

//Class used to Group relevant Storage Functions together 

pub trait storage {
    fn store_map(&self,map_to_add: map, sender: source) -> storage_event_outcome;
    fn store_source(&self,entity: source) -> storage_event_outcome;
    fn get_by_regex_map(&self,regex: String) -> (storage_event_outcome, map);
    fn get_by_regex_source(&self,regex: String) -> (storage_event_outcome, source); 
}

pub enum storage_event_outcome {
    completed_successfully,
    integrity_error (String),
    io_error(String)
}