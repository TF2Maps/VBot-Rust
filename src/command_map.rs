use datatypes::user;
use datatypes::map;
use datatypes::destination;
use datatypes::source;
use regex::Regex;
use storage::storable;
use storage::storage_utility;
use std::collections::HashMap;
pub fn parse_map_commands (Message: String, user: source, storage_system: Box<storage_utility> ){
    println!("");
    println!("For message: {}", &Message);
    if (Message.starts_with("!add") | Message.starts_with("!update") | Message.starts_with("!delete")) {

        let command_splitter = Regex::new(r"(?:^(!update(?:\s)(\S*))|^!delete|^!add)").unwrap();
        let command_split = command_splitter.captures(&Message).unwrap();

        let map_parser = Regex::new(r"(?:(?:\s*)(\S*)(?:\s*))((?:(?:http)\S*)|\b)(?:\s*)(.*)").unwrap();
        
        let map_parsed: Vec<&str> = Regex::new(&command_split[0]).unwrap().split(&Message).collect();

        let captures = map_parser.captures(&map_parsed[1]).unwrap();
        println!("The command is: {}", &command_split[0]);
        println!("{} That's the Mapname", &captures[1]);
        println!("{} That's the url", &captures[2]);
        println!("{} Those are the notes", &captures[3]);
        
        let Parsed_Map = map {
            name: captures[1].to_string(),
            url: captures[2].to_string(),
            notes: captures[3].to_string(),
            uploaded: false,
            owner: user.sender
        };
        let storable_map = &Parsed_Map.convert_to_storable();
        
        if *(&command_split[0].starts_with("!add")) {
            (*storage_system).store_object(storable_map);
            //storage_system.store_object(Parsed_Map.convert_to_storable());
        }
        else if *(&command_split[0].starts_with("!update")) {
            println!("{} ~There's the old map name", &command_split[2]);
            println!("Updating not implemented yet");
            //(*storage_system).store_object(storable_map);
        }
        else if *(&command_split[0].starts_with("!delete")) {
            (*storage_system).delete_stored_data(storable_map.storage_location.clone() , storable_map.primary_keys.clone());

        }
    }
    else if (Message.starts_with("!maps")) {

        let searches: HashMap<String, Regex> = [
            ("Map name".to_string() , Regex::new(".*").unwrap())
        ].iter().cloned().collect();
        println!("{:?}",(*storage_system).get_stored_data("Maps".to_string(), searches));
        //println!("{}", (*storage_system).get_object_by_regex("Maps.csv".to_string(), "".to_string()));
    } 
} 
