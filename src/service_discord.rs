
use serde_json;
use datatypes::discord_chatroom;
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
pub struct service_discord;
use std::collections::HashMap;
use storage_utility;

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
        let binded: String = url;
        
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

pub fn get_all_discord_channels (storage_system: &Box<storage_utility>) -> Vec<discord_chatroom> {
    let searches: HashMap<String, Regex> = [
            ("ChannelID".to_string() , Regex::new(".*").unwrap())
        ].iter().cloned().collect();
    let All_Channels = (*storage_system).get_stored_data("Discord_Config".to_string(),searches);
    let mut All_discord_Channels: Vec<discord_chatroom> = Vec::new();
    
    for channel in All_Channels {
        let mut admins: Vec<&str> = Vec::new();
        if (channel.contains_key("Admins")) {
            admins = channel["Admins"].split("-").collect();
        }
        
        let mut all_admins = Vec::new();
        for admin in admins {
            all_admins.push(admin.to_string());
        }

        let chatroom = discord_chatroom {
            chatroom_id: channel["ChannelID"].clone(),
            webhook: channel["Webhook"].clone(),
            admin_groups: all_admins,
        };

        All_discord_Channels.push(chatroom);
    }
    return All_discord_Channels;

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
pub fn begin_discord_loop (
Commands: Vec<fn( msg: String, user: source, storage_system: &Box<storage_utility>) -> Vec<(destination , String)>> , 
Storage_Handler: &Box<storage_utility> )   
{
    let url: String = "https://www.google.com.au/search?hl=en&source=hp&ei=RF8iWvnfDMaB8gXKj5d4&q=trigger+site%3Adeveloper.valvesoftware.com".to_string();
    let output: String = Get_And_Splice(url.to_string(),"url?q=".to_string(),"&amp".to_string());
  

    //let guild = "217585440457228290";
    let guild = "223211233149583361";
    let all_channels: Vec<discord_chatroom> = get_all_discord_channels(Storage_Handler);
    
    let mut all_channels_and_last_message: HashMap<discord_chatroom, String> = HashMap::new();
    
    for x in all_channels {
        let id = x.chatroom_id.to_string();
        all_channels_and_last_message.insert(x, discord_get_latest_message_id(id));
    }

    discord_loop(all_channels_and_last_message, Commands, Storage_Handler);
}
fn send_message(message: String, desination: String ){

		let params = [("content", &message)];
        let client = reqwest::Client::new();
        let res = client.post(&desination)
            .form(&params)
            .send().expect("Failed to Post Message");
        println!("Sending: {0} to {1}", message, desination);
    }

pub fn discord_loop (
mut h: HashMap<discord_chatroom, String>,
Commands: Vec<fn( msg: String, user: source, storage_system: &Box<storage_utility>) -> Vec<(destination , String)>> , 
Storage_Handler: &Box<storage_utility> ) 
{    
    let mut all_messages: Vec<(source, String, String)> = Vec::new();
    
    for (channel, LastMsgID) in &h {
        let mut channel_messages = get_discord_messages(channel.chatroom_id.clone(), ["/messages?after=", LastMsgID].join(""));
        all_messages.append(&mut channel_messages);
        
    }
    //Goes wrong way round
    for x in &all_messages {

        let mut chatroom: discord_chatroom;
        let mut new_pairs: HashMap<discord_chatroom, String> = h.clone();
        for (channel, lastmessage) in new_pairs {
            if channel.chatroom_id == x.0.chatroom.to_string(){
                chatroom = channel;
                h.insert(chatroom,x.2.to_string());
            }
        }

        let mut isadmin: bool = false;

        let sender = user {
            id: x.0.sender.id.to_string(),  
            application: "Discord".to_string(), 
            display_name: x.0.sender.display_name.to_string() 
        };

        for command in &Commands {
            let responses: Vec<(destination , String)> = command(x.1.clone(), x.0.clone() , Storage_Handler);
            for msg in &responses {
                let mut new_pairs1: HashMap<discord_chatroom, String> = h.clone();
                for (channel, lastmessage) in new_pairs1 {
                    if channel.chatroom_id == x.0.chatroom.to_string(){
                        send_message(msg.1.to_string(),channel.webhook.to_string());
                    }
                }
                println!("{}", msg.1);
            }
        }
    }
    discord_loop(h, Commands, Storage_Handler);

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

