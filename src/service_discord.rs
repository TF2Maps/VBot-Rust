
use serde_json;

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
use storage_debugger::storage_debugger;
use serde_json::{Value, Error};
use File;
use command_map::parse_map_commands;
pub struct service_discord;
use std::collections::HashMap;

pub fn Get_And_Splice(url: String, start: String, end: String) -> String {
    let mut resp = reqwest::get(&url);
    
    let mut resp = match resp {
        Ok(x) => x,
        Err(e) => return "There was an error during the connection process".to_string(),
    };

	assert!(resp.status().is_success());
    
    let mut content = String::new();
    resp.read_to_string(&mut content);
    let mut split = content.split(&start);
    let vec = split.collect::<Vec<&str>>();
    
    let mut secondsplit = vec[1].split(&end);
    let vec2 = secondsplit.collect::<Vec<&str>>();
    return vec2[0].to_string();
}
pub fn get_token () -> String {
    let mut file = File::open("discord_token.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    return contents;
}
pub fn get_page_content(url: String) -> String{
        let binded: String = url; //["https://discordapp.com/api/v6/guilds/".to_string() , channel_id.to_string()].join("");
        
        let url: reqwest::Url = reqwest::Url::parse(&binded).expect("Failed to Post Message");;

        let client = reqwest::Client::new();
        let params = [("Authorization", ["Bot " , &get_token()].join(""))];
        let mut headers = reqwest::header::Headers::new();

        headers.set(reqwest::header::Authorization( ["Bot ", &get_token()].join("")));

        let mut resp = client.get(url).headers(headers).send().expect("Failed to Post Message");
        let mut content = String::new();
		resp.read_to_string(&mut content);
        return content;
}

pub fn get_roles_that_are_admin (channel_id: &str) -> Vec<String> {
        let binded: String = ["https://discordapp.com/api/v6/guilds/".to_string() , channel_id.to_string()].join("");
        let mut content = get_page_content(binded);
      
        let v: Value = serde_json::from_str(&content).unwrap();
        let size = v["roles"].as_array().unwrap().len();
        let mut admins: Vec<String> = Vec::new();
        
        for i in 0..size {
            
            let perms = v["roles"][i]["permissions"].as_u64().unwrap();
            //should be 8, doing 2 for testing. 2 means kicking. 8 means admin.
            if (perms & 8 > 0){
                let id = v["roles"][i]["id"].as_str().unwrap().to_string();
                admins.push(id);
            }
        }
        return admins;
}
//Source, Msg, MsgID
pub fn get_discord_messages (channel_id: String, Parameters: String) -> Vec<(source, String, String)> {
        let binded: String = ["https://discordapp.com/api/v6/channels/".to_string() , channel_id.to_string() , Parameters].join("");
        let mut content = get_page_content(binded);
        
        let v: Value = serde_json::from_str(&content).unwrap();

        if (v.as_array().is_none()){
            return Vec::new();
        }

        let size = v.as_array().unwrap().len();

        let mut messages: Vec<(source, String, String)> = Vec::new();

        for i in 0..size {
            let sender_user = user {
                id: v[i]["author"]["id"].as_str().unwrap().to_string(),
                application: "Discord".to_string(),
                display_name: v[i]["author"]["username"].as_str().unwrap().to_string()
            };
            
            let sender = source {
                sender: sender_user,
                chatroom: channel_id.to_string(),
                elevated_perms: false
            };
            messages.push((sender,v[i]["content"].as_str().unwrap().to_string(),v[i]["id"].as_str().unwrap().to_string()));
        }
        return messages;
}

pub fn discord_get_latest_message_id (channel_id: String) -> String {
        let binded: String = ["https://discordapp.com/api/v6/channels/".to_string() , channel_id.to_string(),"/messages".to_string()].join("");
        let mut content = get_page_content(binded);
        
        let v: Value = serde_json::from_str(&content).unwrap();

        if (v.as_array().is_none()){
            return "".to_string();
        }
       
        let size = v.as_array().unwrap().len();
        if (size == 0){
            return "".to_string();
        }
        return v.as_array().unwrap()[0]["id"].as_str().unwrap().to_string();
}

pub fn get_user_roles (guild_id: &str, user_id: &str) -> Vec<String>{
        let binded: String = ["https://discordapp.com/api/v6/guilds/".to_string() , guild_id.to_string() , "/members/".to_string(), user_id.to_string()].join("");
        let mut content = get_page_content(binded);
        
        let v: Value = serde_json::from_str(&content).unwrap();
        let size = v["roles"].as_array().unwrap().len();
        
        let mut roles: Vec<String> = Vec::new();
        
        for i in 0..size {
            roles.push(v["roles"][i].as_str().unwrap().to_string());
        }
        return roles;
}
pub fn get_all_channels_in_guild (Guild_id: &str) -> Vec<String> {
        let binded: String = ["https://discordapp.com/api/v6/guilds/".to_string() , Guild_id.to_string() , "/channels".to_string()].join("");
        let mut content = get_page_content(binded);

        let regex_to_get_ids = Regex::new(r##"(?:[^"], (?:"id)": ")((?:\\"|[^"])*)"##).unwrap();
      
        let mut channels: Vec<String> = Vec::new();
        
        let caps = regex_to_get_ids.captures(&content).unwrap();
        
        let text1 = caps.get(1).map_or("", |m| m.as_str()).to_string();
        let text2 = caps.get(2).map_or("", |m| m.as_str()).to_string();

        for cap in regex_to_get_ids.captures_iter(&content) {
            if (channels.contains(&cap[1].to_string()) == false){
                channels.push(cap[1].to_string());
            }
        }
        return channels;
}
pub fn begin_discord_loop () {
    let url: String = "https://www.google.com.au/search?hl=en&source=hp&ei=RF8iWvnfDMaB8gXKj5d4&q=trigger+site%3Adeveloper.valvesoftware.com".to_string();
    let output: String = Get_And_Splice(url.to_string(),"url?q=".to_string(),"&amp".to_string());
  

    let guild = "217585440457228290";
    let all_channels: Vec<String> = get_all_channels_in_guild(guild);
    
    let mut all_channels_and_last_message: HashMap<String, String> = HashMap::new();
    
    for x in &all_channels {
        all_channels_and_last_message.insert(x.to_string(),discord_get_latest_message_id(x.to_string()));
    }
    discord_loop(all_channels_and_last_message);
}

pub fn discord_loop (mut h: HashMap<String, String>) {
    
    let mut all_messages: Vec<(source, String, String)> = Vec::new();
    
    for (channelID, LastMsgID) in &h {
        let mut channel_messages = get_discord_messages(channelID.to_string(), ["/messages?after=", LastMsgID].join(""));
        all_messages.append(&mut channel_messages);
    }
    //Goes wrong way round
    for x in &all_messages {
        h.insert(x.0.chatroom.clone(), x.2.clone());

        let mut isadmin: bool = false;

        let sender = user {
            id: x.0.sender.id.to_string(),  
            application: "Discord".to_string(), 
            display_name: x.0.sender.display_name.to_string() 
        };
        let console_storage= Box::new(storage_debugger{});
        parse_map_commands(x.1.clone(), x.0.clone() ,console_storage);
        println!("{} ({}): {}", x.0.sender.display_name, x.0.sender.id, x.1);
        
    }
    discord_loop(h);

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

    fn check_if_admin(id: String) -> bool {
        return false;
    }

    fn tick() -> tick_outcome {
		let mut resp = reqwest::get("https://www.rust-lang.org").expect("Failed to send request?");
		assert!(resp.status().is_success());

		let mut content = String::new();
		resp.read_to_string(&mut content);


		return tick_outcome::nothing;
    }
}

