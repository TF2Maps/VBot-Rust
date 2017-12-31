use datatypes::user;
use datatypes::map;
use datatypes::destination;
use datatypes::source;
use regex::Regex;
use storage::storable;
use storage::storage_utility;
use std::collections::HashMap;
pub fn parse_map_commands (Message: String, user: source, storage_system: Box<storage_utility> ) -> String {
    println!("");
    println!("For message: {}", &Message);
    if (Message.starts_with("!add") | Message.starts_with("!update") | Message.starts_with("!delete")) {

        let command_splitter = Regex::new(r"(?:^(!update(?:\s)(\S*))|^!delete|^!add)").unwrap();
        let command_split = command_splitter.captures(&Message).unwrap();

        let map_parser = Regex::new(r"(?:(?:\s*)(\S*)(?:\s*))((?:(?:http)\S*)|\b)(?:\s*)(.*)").unwrap();
        
        let map_parsed: Vec<&str> = Regex::new(&command_split[0]).unwrap().split(&Message).collect();
        if (map_parsed.len() < 2){
            return "No captures found :(".to_string();
        }

        let captures = map_parser.captures(&map_parsed[1]).unwrap();
        
        let Parsed_Map = map {
            name:     captures.get(1).map_or("", |m| m.as_str()).to_string(),
            url:      captures.get(2).map_or("", |m| m.as_str()).to_string(),
            notes:    captures.get(3).map_or("", |m| m.as_str()).to_string(),
            uploaded: false,
            owner:    user.sender
        };

        let storable_map = &Parsed_Map.convert_to_storable();
        
        if *(&command_split[0].starts_with("!add")) {
            return (*storage_system).store_object(storable_map).to_string();
        }
        else if *(&command_split[0].starts_with("!update")) {
        }
        else if *(&command_split[0].starts_with("!delete")) {
            (*storage_system).delete_stored_data(storable_map.storage_location.clone() , storable_map.primary_keys.clone());
        }
    }
    else if (Message.starts_with("!maps")) {

        let searches: HashMap<String, Regex> = [
            ("Map name".to_string() , Regex::new(".*").unwrap())
        ].iter().cloned().collect();
      
        let allmaps = (*storage_system).get_stored_data("Maps".to_string(), searches);
        
        let mut public_response: String = String::new();
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
        println!("{}", public_response);
    } 
    
    return "done it".to_string();
} 
