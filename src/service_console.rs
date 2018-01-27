use datatypes::user;
use datatypes::source;
use datatypes::destination;
use datatypes::map;

use service::services;
use service::tick_outcome;
use service::activity_outcome;
 
use std::io;
use std::io::BufRead;

pub struct service_console;

impl services for service_console 
{
    fn send_message(message: String, desination: String ) -> Result<String,String> {
        println!("Sending: {0} to {1}", message, desination);
        return Ok("Sent".to_string());
    }
    fn initialise () -> activity_outcome {
        println!("Initialising console service");
        return activity_outcome::success;
    }
    fn check_if_admin(id: String) -> bool {
        return false;
    }
    fn tick() -> tick_outcome {
        let stdin = io::stdin();
		
        let mut Message: String = "".to_string();
            
        for line in stdin.lock().lines() {
            match line {
                Err(x) => {},
                Ok(l) => {
                    Message.push_str(&l);
                }
            }
            break;
        }

        let test_source: source = source {
            sender: user {
                    id: "01".to_string(),  
                    application: "Discord".to_string(), 
                    display_name: "test_ user".to_string() 
                },
            chatroom: ("Test room").to_string(),
            elevated_perms: false
        };
      
        return tick_outcome::message_received(test_source, Message);
    }
}