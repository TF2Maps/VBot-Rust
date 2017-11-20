use map_datatypes;
use map_datatypes::Map;
use map_datatypes::Source;

pub enum storage_outcome {
    completed_successfully,
    integrity_error (String),
    io_error(String)
}

pub trait Storage_Adapter {
    fn Store_Map(&self, map_to_add: Map, sender: Source) -> storage_outcome;
    fn Store_Source(&self, entity: Source) -> storage_outcome;
    fn get_map_by_regex(&self, regex: String) -> (storage_outcome, Map); 
    fn get_source_by_regex(&self, regex: String) -> (storage_outcome, Source); 
}