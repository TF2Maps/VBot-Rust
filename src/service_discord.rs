use datatypes::user;
use datatypes::source;
use datatypes::destination;
use datatypes::map;

use service::service;
use service::tick_outcome;
use service::activity_outcome;

use std::io;
use std::io::BufRead;

extern crate reqwest;

use std::io::Read;

pub struct service_discord;

impl service for service_discord
{

    fn send_message(&self, message: String, desination: String ) -> activity_outcome {

		let params = [("content", &message)];
        let client = reqwest::Client::new();
        let res = client.post(&desination)
        .form(&params)
        .send().expect("Failed to Post Message");


        println!("Sending: {0} to {1}", message, desination);
        return activity_outcome::success;
    }
    fn initialise (&self) -> activity_outcome {
        println!("Beginning initialisation for Discord service");
        return activity_outcome::success;
    }

    fn tick(&self) -> tick_outcome {
		let mut resp = reqwest::get("https://www.rust-lang.org").expect("Failed to send request?");
		assert!(resp.status().is_success());

		let mut content = String::new();
		resp.read_to_string(&mut content);


		return tick_outcome::nothing;
    }
}

