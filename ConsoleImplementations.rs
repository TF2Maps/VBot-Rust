use map_datatypes;

use StorageAdapter;
use StorageAdapter::Storage_Adapter;
use StorageAdapter::storage_outcome;

use map_datatypes::User;
use map_datatypes::Source;
use map_datatypes::Destination;
use map_datatypes::Map;

use UserHandler::UserHandler;
use UserHandler::Message_Handle_Responses;

use service::Service;
use service::Activity_Outcome;
use service::Tick_Outcome;

use std::io;
use std::io::BufRead;

pub struct Console_Bot;


impl Storage_Adapter for Console_Bot {
    fn Store_Map(&self, map_to_add: Map, sender: Source) -> storage_outcome{
        println!("Storing Map");
        return storage_outcome::completed_successfully;
    }

    fn Store_Source(&self, entity: Source) -> storage_outcome {
        println!("Storing Source");
        storage_outcome::completed_successfully
    }

    fn get_map_by_regex(&self, regex: String) -> (storage_outcome, Map){
        
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

        let tuple: (storage_outcome, Map)  = (storage_outcome::completed_successfully, testmap);
        return tuple;
    }
    fn get_source_by_regex(&self, regex: String) -> (storage_outcome, Source) {
        
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
        let tuple: (storage_outcome, Source)  = (storage_outcome::completed_successfully, TestSource);
        return tuple;
    } 

}

impl Service for Console_Bot 
{
    fn SendPrivateMessage(&self, message: String, desination_user_id: String )-> Activity_Outcome {
        println!("Sending: {0} to {1}", message, desination_user_id);
        return Activity_Outcome::success;
    }
    fn SendPublicMessage(&self, message: String, desination_chatroom_id: String ) -> Activity_Outcome {
        println!("Sending: {0} to {1}", message, desination_chatroom_id);
        return Activity_Outcome::success;
    }
    fn Login (&self) -> Activity_Outcome {
        println!("Logging in!");
        return Activity_Outcome::success;
    }
    fn OnTick(&self) -> Tick_Outcome {
        
        let TestUser: User = User {
            id: "00".to_string(),  
            application: "Console".to_string(), 
            display_name: "Console_Dummy".to_string() 
        };

        let stdin = io::stdin();
        let mut Message: String = "".to_string();
        
        for line in stdin.lock().lines() {
            Message.push_str(&line.unwrap());
            break;
        }
        
        return Tick_Outcome::Received_Private_Message(TestUser, Message);
    }
}

impl UserHandler for Console_Bot 
{
    fn Handle_Message(&self, msg: String, sender: Source) -> (Message_Handle_Responses) {
        
        let TestUser: User = User {
            id: "01".to_string(),  
            application: "Discord".to_string(), 
            display_name: "Test User".to_string() 
        };

        let TestDestination: Destination = Destination {
            application : "Discord".to_string(),
            chatroom: "Console #1".to_string()
        };
        
        let mut Public_Responses_Group: Vec<(Destination,String)> = Vec::new();
        let mut Private_Responses_Group: Vec<(User,String)> = Vec::new();
        
        Public_Responses_Group.push( (TestDestination, "Hello World".to_string()) );
        Private_Responses_Group.push( (TestUser, "Hello User".to_string()) ); 

        let Responses: Message_Handle_Responses = Message_Handle_Responses {
            Public_Responses: Public_Responses_Group,
            Private_Responses: Private_Responses_Group
        };
        return Responses;
    }
}