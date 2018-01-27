use datatypes::user;
use datatypes::source;
use datatypes::destination;
use datatypes::map;
use std::collections::HashMap;
use std::vec;
use regex::Regex;

//Class used to Group relevant Storage Functions together 

pub struct storable_object {
    pub storage_location: String,
    pub primary_keys: Vec<(String, String)>,
    pub values: Vec<(String, String)>,
    pub foreign_keys: Vec<(String , storable_object)>
}

pub trait storable {
        fn convert_to_storable(self) -> storable_object;        
} 
pub enum SaveErrorReason {
    KeyAlreadyExists, IOError, FileNotFound
}

pub trait storage_utility {
    fn store_object(&self, object: &storable_object) ->  &str;
    fn get_stored_data(&self, storage_location: String, primary_keys: HashMap<String, Regex>) -> Vec<HashMap<String, String>>;
    fn delete_stored_data(&self, storage_location: String, primary_keys: Vec<(String, String)>) -> Result<String,String>; 
    fn get_by_regex_map(&self,regex: String) -> (storage_event_outcome, map);
    fn get_by_regex_source(&self,regex: String) -> (storage_event_outcome, source); 
}

pub enum storage_event_outcome {
    completed_successfully,
    integrity_error (String),
    io_error(String)
}