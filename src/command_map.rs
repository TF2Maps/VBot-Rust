use datatypes::user;
use datatypes::map;

use regex::Regex;

pub fn parse_map (Message: String, owner: user ) {
    println!("");
    println!("For message: {}", &Message);
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
        owner: owner
    };

    if *(&command_split[0].starts_with("!add")) {
    }
    if *(&command_split[0].starts_with("!update")) {
        println!("{} ~There's the old map name", &command_split[2]);
    }
    if *(&command_split[0].starts_with("!delete")) {
    }

    


} 
