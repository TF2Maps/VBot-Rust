use datatypes::user;
use datatypes::source;
use datatypes::destination;
use datatypes::map;

use service::services;
use service::tick_outcome;
use service::activity_outcome;

use std::io;
use std::io::BufRead;
use regex::Regex;


extern crate regex;
extern crate reqwest;
use std::io::Read;
use reqwest::Url;
use reqwest::header;
use reqwest::header::UserAgent;

pub struct service_discord;


pub fn get_all_channels_in_guild (Guild_id: String) -> Vec<String> {
        let binded: String = ["https://discordapp.com/api/v6/guilds/".to_string() , Guild_id , "/channels".to_string()].join("");
        


        let url: reqwest::Url = reqwest::Url::parse(&binded).expect("Failed to Post Message");;

        let client = reqwest::Client::new();
        let params = [("Authorization", "Bot ")];
        let mut headers = reqwest::header::Headers::new();

        headers.set(reqwest::header::Authorization("Bot ".to_owned()));

        let mut resp = client.get(url).headers(headers).send().expect("Failed to Post Message");
        let mut content = String::new();
		resp.read_to_string(&mut content);

        let regex_to_get_ids = Regex::new(r##"(?:[^"], (?:"id)": ")((?:\\"|[^"])*)"##).unwrap();
      
        let mut channels: Vec<String> = Vec::new();
        
        let caps = regex_to_get_ids.captures(&content).unwrap();
        
        let text1 = caps.get(1).map_or("", |m| m.as_str()).to_string();
       

        for cap in regex_to_get_ids.captures_iter(&content) {

            if (channels.contains(&cap[1].to_string()) == false){
                channels.push(cap[1].to_string());
            }
            
        }
        return channels;
}
impl services for service_discord
{


    fn send_message(message: String, desination: String ) -> activity_outcome {

		let params = [("content", &message)];
        let client = reqwest::Client::new();
        let res = client.post(&desination)
        .form(&params)
        .send().expect("Failed to Post Message");


        println!("Sending: {0} to {1}", message, desination);
        return activity_outcome::success;
    }
    fn initialise () -> activity_outcome {
        println!("Beginning initialisation for Discord service");
        return activity_outcome::success;
    }

    fn tick() -> tick_outcome {
		let mut resp = reqwest::get("https://www.rust-lang.org").expect("Failed to send request?");
		assert!(resp.status().is_success());

		let mut content = String::new();
		resp.read_to_string(&mut content);


		return tick_outcome::nothing;
    }
}

