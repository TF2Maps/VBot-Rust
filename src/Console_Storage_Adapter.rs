use map_datatypes;
use Bot;

use Storage_Adapter::Storage_outcome;
use Storage_Adapter::Storage_Adapter;

use Message_Handle_Responses;

use map_datatypes::User;
use map_datatypes::Source;
use map_datatypes::Destination;
use map_datatypes::Map;


use std::io;
use std::io::BufRead;

#[derive(Debug,Copy, Clone)]
pub struct Console_Storage_Adapter 
{
}

impl Storage_Adapter for Console_Storage_Adapter
{
    fn Store_Map(&self, map_to_add: Map, sender: Source) -> Storage_outcome {
        println!("Storing Map");
        return Storage_outcome::completed_successfully;
    }

    fn Store_Source(&self, entity: Source) -> Storage_outcome {
        println!("Storing Source");
        Storage_outcome::completed_successfully
    }

    fn get_map_by_regex(&self, regex: String) -> (Storage_outcome, Map) {
            
        println!("Getting Map by Regex");

        let TestUser: User = User {
            id: "01".to_string(),  
            application: "Discord".to_string(), 
            display_name: "Test User".to_string() 
        };

        let testmap: Map = Map {
            name: "name".to_string(),
            url: "url".to_string(), 
            notes: "notes".to_string(),  
            uploaded: true, 
            owner: TestUser
        };

        let tuple: (Storage_outcome, Map)  = (Storage_outcome::completed_successfully, testmap);
        return tuple;
        }
        
    fn get_source_by_regex(&self, regex: String) -> (Storage_outcome, Source) {
            
        println!("Getting Source by Regex");
        let TestUser: User = User {
            id: "01".to_string(),  
            application: "Discord".to_string(), 
            display_name: "Test User".to_string() 
        };
        let TestSource: Source = Source {
            sender: TestUser,
            chatroom: "01".to_string(),
            elevated_perms: true
        };
        let tuple: (Storage_outcome, Source)  = (Storage_outcome::completed_successfully, TestSource);
        return tuple;
    } 
}