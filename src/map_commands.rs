use map_datatypes::User;
use map_datatypes::Map;

use regex::Regex;

pub fn ParseMap (Message: String ) {
    let MapRegex = Regex::new(r"(^.\S*)(?:(?:\s*)(\S*)(?:\s*))((?:(?:http)\S*)|\b)(?:\s*)(.*)").unwrap();
    
    let cap = MapRegex.captures(&Message).unwrap();

    println!("{} That's the Command", &cap[1]);
    println!("{} That's the Mapname", &cap[2]);
    println!("{} That's the URL", &cap[3]);
    println!("{} That's the Notes", &cap[4]);

} 