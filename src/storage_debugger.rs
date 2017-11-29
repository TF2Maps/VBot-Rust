
use storage::storage_event_outcome;
use storage::storage;


use datatypes::user;
use datatypes::source;
use datatypes::map;



#[derive(Debug,Copy, Clone)]
pub struct storage_debugger 
{
}

impl storage for storage_debugger
{
    fn store_map(&self, map_to_add: map, sender: source) -> storage_event_outcome {
        println!("Storing map");
        return storage_event_outcome::completed_successfully;
    }

    fn store_source(&self, entity: source) -> storage_event_outcome {
        println!("Storing source");
        storage_event_outcome::completed_successfully
    }

    fn get_by_regex_map(&self, regex: String) -> (storage_event_outcome, map) {
            
        println!("Getting map by Regex");

        let test_user: user = user {
            id: "01".to_string(),  
            application: "Discord".to_string(), 
            display_name: "test_ user".to_string() 
        };

        let test_map: map = map {
            name: "name".to_string(),
            url: "url".to_string(), 
            notes: "notes".to_string(),  
            uploaded: true, 
            owner: test_user
        };

        let tuple: (storage_event_outcome, map)  = (storage_event_outcome::completed_successfully, test_map);
        return tuple;
        }
        
    fn get_by_regex_source(&self, regex: String) -> (storage_event_outcome, source) {
            
        println!("Getting source by Regex");
        let test_user: user = user {
            id: "01".to_string(),  
            application: "Discord".to_string(), 
            display_name: "test_ user".to_string() 
        };
        let test_source: source = source {
            sender: test_user,
            chatroom: "01".to_string(),
            elevated_perms: true
        };
        let tuple: (storage_event_outcome, source)  = (storage_event_outcome::completed_successfully, test_source);
        return tuple;
    } 
}