use datatypes::user;
use datatypes::map;

use regex::Regex;

pub fn parse_map (Message: String ) {
    let regex = Regex::new(r"(?:(^!add|^!delete)|^!update\s*(\S*))(?:(?:\s*)(\S*)(?:\s*))((?:(?:http)\S*)|\b)(?:\s*)(.*)").unwrap();
    let captures = regex.captures(&Message).unwrap();

    println!("{} That's the Command", &captures[1]);
    println!("{} That's the mapname", &captures[2]);
    println!("{} That's the URL", &captures[3]);
    println!("{} That's the Notes", &captures[4]);

} 
