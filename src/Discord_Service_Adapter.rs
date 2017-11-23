use map_datatypes::User;
use map_datatypes::Source;
use map_datatypes::Destination;
use map_datatypes::Map;

use Service_Adapter::Service;
use Service_Adapter::Tick_Outcome;
use Service_Adapter::Activity_Outcome;
 
use std::io;
use std::io::BufRead;


extern crate reqwest;

use std::io::Read;

pub struct Discord_Service;

impl Service for Discord_Service 
{
    fn Send_Private_Message(&self, message: String, desination_user_id: String )-> Activity_Outcome {
        println!("Sending: {0} to {1}", message, desination_user_id);
        return Activity_Outcome::success;
    }
    
    fn Send_Public_Message(&self, message: String, desination_chatroom_id: String ) -> Activity_Outcome {
        let params = [("content", "Hello World")];
        
        let client = reqwest::Client::new();
        let res = client.post("URL GOES HERE")
        .form(&params)
        .send().expect("Failed to Post Message");


        println!("Sending: {0} to {1}", message, desination_chatroom_id);
        return Activity_Outcome::success;
    }
    fn Login (&self) -> Activity_Outcome {
        println!("Logging in!");
        return Activity_Outcome::success;
    }
  
    fn On_Tick(&self) -> Tick_Outcome {
        
    let mut resp = reqwest::get("https://www.rust-lang.org").expect("Failed to send request?");
    assert!(resp.status().is_success());
    
    let mut content = String::new();
    resp.read_to_string(&mut content);

            
    return Tick_Outcome::DoNothing;
    }
}

