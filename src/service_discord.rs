
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
pub fn get_token () -> Result<String,String> {
    let mut file = File::open("discord_token.txt");
     
    match file {
        Ok(mut x) => {
            let mut contents = String::new();
            x.read_to_string(&mut contents);
            return Ok(contents);
        },
        Err(x) => return Err("There was an IO Error".to_string())
    }
}

pub fn get_page_content(url: String , token: String) -> Result<String,String>{
    
        let binded: String = url;
        let client = reqwest::Client::new();
        let params = [("Authorization", ["Bot " , &token].join(""))];
        let mut headers = reqwest::header::Headers::new();
        
        let url = reqwest::Url::parse(&binded);
        headers.set(reqwest::header::Authorization( ["Bot ", &token].join("")));

        match url {
            Ok(x) => {
                match client.get(x).headers(headers).send(){
                    Ok(mut y) => {
                                let mut content = String::new();
                                y.read_to_string(&mut content);
                                return Ok(content);
                    },
                    Err(x) => return Err("Response was errornous".to_string()),
                }
            },
            Err(x) => return Err("URL was errornous".to_string()),
        }

}

pub fn get_roles_that_are_admin (channel_id: &str, token: String) -> Result<Vec<String>, String> {
        let binded: String = ["https://discordapp.com/api/v6/guilds/".to_string() , channel_id.to_string()].join("");
        let content = get_page_content(binded, token);
        
        match content {
            Err(x) => return Err("Error".to_string()),
            Ok(x) => {
                let outcome: Result<Value, serde_json::Error> = serde_json::from_str(&x);
                match outcome {
                    Ok(result) => 
                    {
                        match result["roles"].as_array() {
                            Some(Array) => {
                                let size = Array.len();
                                let mut admins: Vec<String> = Vec::new();
                                for i in 0..size {
                                    let perms = result["roles"][i]["permissions"].as_u64();
                    
                                    match perms {
                                        Some(x) => {
                                            //should be 8, doing 2 for testing. 2 means kicking. 8 means admin.
                                            if (x & 8 > 0) {
                                                match result["roles"][i]["id"].as_str() {
                                                    Some(id) => admins.push(id.to_string()),
                                                    None => return Err("There was an error parsing the JSON".to_string()),
                                                }
                                            }
                                        }
                                        None => return Err("There was an error parsing the JSON".to_string()),
                                    }
                                }
                                return Ok(admins);
                            },
                            None => return Err("There was an error parsing the JSON".to_string()),
                        }
                    }
                    Err(x) => return Err("There was an error processing".to_string()),
                    }
                }
            }
        }


//Source, Msg, MsgID
pub fn get_discord_messages (channel_id: String, Parameters: String, Token: String) -> Result<Vec<(source, String, String)>, String> {
        let binded: String = ["https://discordapp.com/api/v6/channels/".to_string() , channel_id.to_string() , Parameters].join("");
        let content = get_page_content(binded, Token);
        match content {
            Err(x) => return Err("Error".to_string()),
            Ok(x) => {
                let v: Result<Value,serde_json::Error> = serde_json::from_str(&x);
                match v {
                    Err(y) => return Err("Json Error".to_string()),
                    Ok(y) => {
                        let mut messages: Vec<(source, String, String)> = Vec::new();
                        match y.as_array(){
                            None => return Err("Error".to_string()),
                            Some(z) => {
                                for i in 0..z.len() {
                                    let id = z[i]["author"]["id"].as_str();
                                    let disp_name = y[i]["author"]["username"].as_str();
                                    match id {
                                        None => return Err("Error".to_string()),
                                        Some(ID) => {
                                            match disp_name {
                                                None => return Err("Error".to_string()),
                                                Some(DISP_NAME) =>
                                                {
                                                    let sender_user = user {
                                                        id: ID.to_string(),
                                                        application: "Discord".to_string(),
                                                        display_name: DISP_NAME.to_string()
                                                    };
                                                    
                                                    let sender = source {
                                                        sender: sender_user,
                                                        chatroom: channel_id.to_string(),
                                                        elevated_perms: false
                                                    };
                                                    match y[i]["content"].as_str() {
                                                        None => {},
                                                        Some(Content) =>
                                                        {
                                                            match y[i]["id"].as_str(){
                                                                None => {},
                                                                Some(Message_ID) => {
                                                                    let x = messages.push((sender, Content.to_string(), Message_ID.to_string()));
                                                                },
                                                            } 
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        return Ok(messages);
                    }
                }
            }
        }
}


pub fn discord_get_latest_message_id (channel_id: String, Token: String) -> Result<String, String> {
        let binded: String = ["https://discordapp.com/api/v6/channels/".to_string() , channel_id.to_string(),"/messages".to_string()].join("");
        let content = get_page_content(binded, Token);
        
        match content {
            Err(x) => return Err("Error".to_string()),
            Ok(x) => {
                 let parsed: Result<Value, serde_json::Error> = serde_json::from_str(&x);
                 match parsed {
                    Err(y) => return Err("Json Error".to_string()),
                    
                    Ok(Json) => {
                        let array = Json.as_array();
                        match array {
                            None => return Err("no".to_string()),
                            Some(z) => {
                                let size = z.len();
                                if (size == 0){
                                    return Err("0 Size".to_string());
                                }
                                match z[0]["id"].as_str(){
                                    None => return Err("Error".to_string()),
                                    Some(A) => return Ok(A.to_string()),
                                }
                            }
                        }
                    }
                }
            }
        }

}

pub fn get_user_roles (guild_id: &str, user_id: &str, token: String) -> Vec<String>{
        let binded: String = ["https://discordapp.com/api/v6/guilds/".to_string() , guild_id.to_string() , "/members/".to_string(), user_id.to_string()].join("");
        match get_page_content(binded, token) {
            Err(x) => return Vec::new(),
            Ok(x) => {
                let item: Result<Value, serde_json::Error> = serde_json::from_str(&x);
                match item {
                    Err(x) => return Vec::new(),
                    Ok(Json) =>
                    {
                        
                        match Json["roles"].as_array() {
                            None => return Vec::new(), //TODO Propogate Error
                            Some(Array) => {
                                let mut roles: Vec<String> = Vec::new();
                                let size = Array.len();
            
                                for i in 0..size {
                                    match Array[i].as_str() {
                                        None =>{},
                                        Some(Entry) => roles.push(Entry.to_string()),
                                    }
                                }
                                return roles;
                            }
                        }
                    }
                }
            }
        }
         
        
        
        

}

pub fn get_all_discord_channels (storage_system: &Box<storage_utility>) -> Result<Vec<discord_chatroom>,String> {
    match Regex::new(".*")
    {
        Err(x) => return Err("Error".to_string()),
        Ok(regex) => {
            let searches: HashMap<String, Regex> = [
                    ("ChannelID".to_string() , regex)
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
            return Ok(All_discord_Channels);
        }
    }


}
pub fn get_all_channels_in_guild (Guild_id: &str, Token: String) -> Result<Vec<String>, String> {
        let binded: String = ["https://discordapp.com/api/v6/guilds/".to_string() , Guild_id.to_string() , "/channels".to_string()].join("");
        
        match get_page_content(binded, Token) {
            Ok(content) =>
            {
                match Regex::new(r##"(?:[^"], (?:"id)": ")((?:\\"|[^"])*)"##) {
                    
                    Ok(regex) =>{
                        let mut channels: Vec<String> = Vec::new();
                        match (regex.captures(&content)) {
                            Some (captures) =>
                            {
                                let text1 = captures.get(1).map_or("", |m| m.as_str()).to_string();
                                let text2 = captures.get(2).map_or("", |m| m.as_str()).to_string();
                                

                                for cap in regex.captures_iter(&content) {
                                    if (channels.contains(&cap[1].to_string()) == false){
                                        channels.push(cap[1].to_string());
                                    }
                                }
                                return Ok(channels);
                            },
                            None => return Ok(channels),
                        }
                    },
                    Err(x) => return Err("Error parsing the Regex".to_string()),
                }
            }
            Err(x) => return Err("There was an error getting the page content".to_string()),
        }
}

//Add behaviour for error handling
pub fn begin_discord_loop (
Commands: Vec<fn( msg: String, user: source, storage_system: &Box<storage_utility>) -> Vec<(destination , String)>> , 
Storage_Handler: &Box<storage_utility>)   
{
    let url: String = "https://www.google.com.au/search?hl=en&source=hp&ei=RF8iWvnfDMaB8gXKj5d4&q=trigger+site%3Adeveloper.valvesoftware.com".to_string();
    let output: String = Get_And_Splice(url.to_string(),"url?q=".to_string(),"&amp".to_string());
  
    let guild = "223211233149583361";
    let all_channels = get_all_discord_channels(Storage_Handler);
    
    match all_channels{
        Err(x) => {},
        Ok(channels) => {
            let mut all_channels_and_last_message: HashMap<discord_chatroom, String> = HashMap::new();
            match get_token(){
                    Err(x) => {},
                    Ok(Token) => {
                        for x in channels {
                            let id = x.chatroom_id.to_string();
                            
                            match discord_get_latest_message_id(id, Token.to_string()) {
                                Err(x) => {},
                                Ok(result) => {
                                    all_channels_and_last_message.insert(x,result.to_string());
                                }
                            }     
                        }
                        discord_loop(all_channels_and_last_message, Commands, Storage_Handler, Token.to_string());
                }
            }
            
        }
    }

    
}
fn send_message(message: String, desination: String ){

		let params = [("content", &message)];
        let client = reqwest::Client::new();
        let res = client.post(&desination)
            .form(&params)
            .send();

        match (res){
            Ok(x) => println!("Sending: {0} to {1}", message, desination),
            Err(x) => println!("Sending: {0} to {1}", message, desination),
        }    
}
//Add action for error
pub fn discord_loop (
mut h: HashMap<discord_chatroom, String>,
Commands: Vec<fn( msg: String, user: source, storage_system: &Box<storage_utility>) -> Vec<(destination , String)>> , 
Storage_Handler: &Box<storage_utility>, Token: String) 
{    
    let mut all_messages: Vec<(source, String, String)> = Vec::new();
    
    for (channel, LastMsgID) in &h {
        match get_discord_messages(channel.chatroom_id.clone(), ["/messages?after=", LastMsgID].join(""), Token.to_string()){
            Err(x) => {},
            Ok(mut channel_messages) => { all_messages.append(&mut channel_messages); }
        }
        
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
    discord_loop(h, Commands, Storage_Handler, Token);

}
impl services for service_discord
{
    fn send_message(message: String, desination: String ) -> Result<String,String> {

		let params = [("content", &message)];
        let client = reqwest::Client::new();
        let res = client.post(&desination)
            .form(&params)
            .send();
        match res {
            Ok(x) => return Ok("Message Sent".to_string()),
            Err(x) => return Err("There was an error sending the message".to_string()),
        }    
    }

    fn initialise () -> activity_outcome {
        println!("Beginning initialisation for Discord service");
        return activity_outcome::success;
    }

    fn check_if_admin(id: String) -> bool {
        return false;
    }

    fn tick() -> tick_outcome {
		return tick_outcome::nothing;
    }
}
