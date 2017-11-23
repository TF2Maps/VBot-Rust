use map_datatypes::User;
use map_datatypes::Source;
use map_datatypes::Destination;
use map_datatypes::Map;

//Class used to Group relevant Storage Functions together 

pub trait Storage_Adapter {
    fn Store_Map(&self,map_to_add: Map, sender: Source) -> Storage_outcome;
    fn Store_Source(&self,entity: Source) -> Storage_outcome;
    fn get_map_by_regex(&self,regex: String) -> (Storage_outcome, Map);
    fn get_source_by_regex(&self,regex: String) -> (Storage_outcome, Source); 
}

pub enum Storage_outcome {
    completed_successfully,
    integrity_error (String),
    io_error(String)
}