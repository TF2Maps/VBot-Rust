use datatypes::user;
use datatypes::map;
use datatypes::destination;
use datatypes::source;
use regex::Regex;
use storage::storable;
use storage::storage_utility;
use std::collections::HashMap;
use storage::storable_object;

fn get_map_from_message (Message: String, user: source) -> Option<storable_object> {
        let command_splitter = Regex::new(r"(?:^(!update(?:\s)(\S*))|^!delete|^!add)").unwrap();
        let command_split = command_splitter.captures(&Message).unwrap();

        let map_parser = Regex::new(r"(?:(?:\s*)(\S*)(?:\s*))((?:(?:http)\S*)|\b)(?:\s*)(.*)").unwrap();
        
        let map_parsed: Vec<&str> = Regex::new(&command_split[0]).unwrap().split(&Message).collect();
        if (map_parsed.len() < 2){
            return None;
        }

        let captures = map_parser.captures(&map_parsed[1]).unwrap();
        
        let Parsed_Map = map {
            name:     captures.get(1).map_or("", |m| m.as_str()).to_string(),
            url:      captures.get(2).map_or("", |m| m.as_str()).to_string(),
            notes:    captures.get(3).map_or("", |m| m.as_str()).to_string(),
            uploaded: false,
            owner:    user.sender
        };
        return Some(Parsed_Map.convert_to_storable());
}

pub fn add_command (Message: String, user: source, storage_system: &Box<storage_utility> ) -> Vec<(destination , String)> {
    let mut responses: Vec<(destination , String)> = Vec::new();
    
    if Message.starts_with("!add") {
            let destination = destination {
            chatroom: user.chatroom.clone(), 
            application: user.sender.application.clone()
        };
        let storable_map = get_map_from_message(Message, user);
        


        match storable_map {
            Some(expr) =>  responses.push((destination, ((*storage_system).store_object(&expr).to_string()))),
            None => return responses
        }
    }
    return responses;
}

pub fn delete_command (Message: String, user: source, storage_system: &Box<storage_utility> ) -> Vec<(destination , String)> {
    let mut responses: Vec<(destination , String)> = Vec::new();
    if Message.starts_with("!delete") {
        let destination = destination {
            chatroom: user.chatroom.clone(), 
            application: user.sender.application.clone()
        };
        let storable_map = get_map_from_message(Message, user);
        let mut map_success = String::new();

        match storable_map {
            Some(expr) =>  responses.push((destination, (*storage_system).delete_stored_data(expr.storage_location.clone() , expr.primary_keys.clone()))),
            None => return responses
        }
    }
    return responses;
}

pub fn retrieve_maps_command (Message: String, user: source, storage_system: &Box<storage_utility> ) -> Vec<(destination , String)> {
    let mut public_response: String = String::new();
    if Message.starts_with("!maps") {
        let searches: HashMap<String, Regex> = [
            ("Map name".to_string() , Regex::new(".*").unwrap())
        ].iter().cloned().collect();
      
        let allmaps = (*storage_system).get_stored_data("Maps".to_string(), searches);
        
        let mut i = 0;
        for map in &allmaps {
            let mapname: &str = &(String::from(map["Map name"].clone()));
            public_response.push_str(&i.to_string());
            public_response.push_str(") ");
            public_response.push_str(mapname);
            public_response.push_str(" | ");
            let url: &str = &(String::from(map["Url"].clone()));
            public_response.push_str(url);
            public_response.push_str(" | ");
            //let name: &str = &(String::from(map["Display Name"].clone()));
            //public_response.push_str(name);
            public_response.push_str(" (");
            let id: &str = &(String::from(map["ID"].clone()));
            public_response.push_str(id);
            public_response.push_str(")\n");
            let note: &str = &(String::from(map["Notes"].clone()));
            public_response.push_str(note);
            public_response.push_str("\n");
            i = i + 1;
        }
        let destination = destination {
            chatroom: user.chatroom, 
            application: user.sender.application
        };
        let mut responses = Vec::new();
        responses.push((destination, public_response));
        return responses;
    }
    return Vec::new();
}