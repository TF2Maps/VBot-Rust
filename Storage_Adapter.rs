use map_datatypes::User;
use map_datatypes::Source;
use map_datatypes::Destination;
use map_datatypes::Map;

//Class used to Group relevant Storage Functions together 
pub struct Storage_Adapter {
    pub Store_Map: fn(map_to_add: Map, sender: Source) -> Storage_outcome,
    pub Store_Source: fn(entity: Source) -> Storage_outcome,
    pub get_map_by_regex: fn(regex: String) -> (Storage_outcome, Map),
    pub get_source_by_regex: fn(regex: String) -> (Storage_outcome, Source), 
}

pub enum Storage_outcome {
    completed_successfully,
    integrity_error (String),
    io_error(String)
}