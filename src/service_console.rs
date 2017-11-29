use datatypes::user;
use datatypes::source;
use datatypes::destination;
use datatypes::map;

use service::service;
use service::tick_outcome;
use service::activity_outcome;
 
use std::io;
use std::io::BufRead;

pub struct service_console;

impl service for service_console 
{
    fn send_message(&self, message: String, desination: String ) -> activity_outcome {
        println!("Sending: {0} to {1}", message, desination);
        return activity_outcome::success;
    }
    fn initialise (&self) -> activity_outcome {
        println!("Initialising console service");
        return activity_outcome::success;
    }
    fn tick(&self) -> tick_outcome {
        let stdin = io::stdin();
		
        let mut Message: String = "".to_string();
            
        for line in stdin.lock().lines() {
            Message.push_str(&line.unwrap());
            break;
        }
            
        return tick_outcome::message_received(testsource, Message);
    }
}