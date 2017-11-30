use datatypes::user;
use datatypes::map;

use regex::Regex;

pub fn parse_map (Message: String ) {
    let initial_split = Regex::new(r"(?:^(!update(?:\s)(\S*))|^!delete|^!add)").unwrap();
    let firstsplit: Vec<&str> = initial_split.split(&Message).collect();

    let map_parser = Regex::new(r"(?:(?:\s*)(\S*)(?:\s*))((?:(?:http)\S*)|\b)(?:\s*)(.)").unwrap();
    if (firstsplit[0] == "!add")
    {

    }
    if (firstsplit[0] == "!update")
    {

    }
    if (firstsplit[0] == "!delete")
    {

    }
    let captures = map_parser.captures(firstsplit[1]).unwrap();

    println!("{} That's the Command", &captures[1]);
    println!("{} That's the mapname", &captures[2]);
    println!("{} That's the URL", &captures[3]);
    println!("{} That's the Notes", &captures[4]);

} 
