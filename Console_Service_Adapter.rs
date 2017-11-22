use map_datatypes::User;
use map_datatypes::Source;
use map_datatypes::Destination;
use map_datatypes::Map;

use Service_Adapter::Service;
use Service_Adapter::Tick_Outcome;
use Service_Adapter::Activity_Outcome;
 
use std::io;
use std::io::BufRead;

pub fn Get_Console_Service_Adapter() -> Service {

    let Console_Service_Adapter: Service = Service {
        Send_Private_Message: Send_Private_Message,
        Send_Public_Message: Send_Public_Message,
        Login: Login,
        On_Tick: On_Tick,    
    };
    return Console_Service_Adapter;
}

pub fn Send_Private_Message(message: String, desination_user_id: String )-> Activity_Outcome {
        println!("Sending: {0} to {1}", message, desination_user_id);
        return Activity_Outcome::success;
    }
pub fn Send_Public_Message(message: String, desination_chatroom_id: String ) -> Activity_Outcome {
        println!("Sending: {0} to {1}", message, desination_chatroom_id);
        return Activity_Outcome::success;
    }
pub fn Login () -> Activity_Outcome {
        println!("Logging in!");
        return Activity_Outcome::success;
    }
pub fn On_Tick() -> Tick_Outcome {
        
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