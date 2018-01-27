
use storage::storage_event_outcome;
use storage::storage_utility;
use storage::storable_object;


use datatypes::user;
use datatypes::source;
use datatypes::map;


use std::fs::OpenOptions;
use std::io::prelude::*;
use storage::*;

use std::io::BufWriter;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
pub struct storage_debugger {}
fn get_not_matching_lines(storage_location: String, primary_keys: Vec<(String, String)>) -> Result<String,String> {
    println!("STORAGE RETRIEVAL NEEDS TO OUTPUT A RESULT");
    let filepath: String = storage_location.to_string() + ".csv";
    
    let mut file = OpenOptions::new()
        .read(true)
        .open(filepath);
    match file {
        Err(x) => return Err("An error occured".to_string()),
        Ok(x) => {
            let mut file_buffer = BufReader::new(&x);
            
            let mut first_line = String::new();
            file_buffer.read_line(&mut first_line);
            
            let all_keys: Vec<&str> = first_line.split(",").collect();
            
            let mut NewFile: String = first_line.clone();
            for line in file_buffer.lines() {
                match line {
                    Err(x) => {},
                    Ok(l) => {
                        let row: Vec<&str> = l.split(",").collect();
                        let mut i = 0;
                        let mut found_entry = true;

                        for x in &primary_keys {
                            let entry = row[i].to_string();
                            let entry2 = x.1.to_string();
                            if (entry == entry2) {
                            } else {
                                NewFile.push_str(&l);
                                NewFile.push_str(&"\n");
                                found_entry = false;
                            }
                            i = i + 1;
                        }
                        if (found_entry == false)
                        {
                        }
                    }
                }
            }
            return Ok(NewFile);
        }
        Err(x) => return Err("An error occured".to_string())
    }

    
}

//Bit tedious and should get cut down significantly
//However time is better spent getting a DB setup
impl storage_utility for storage_debugger {
    fn store_object(&self, object: &storable_object) -> &str {
        let mut csv_line: String = "".to_string();
        let mut Header_line: String = "".to_string();

        for item in &object.primary_keys {
            Header_line = Header_line + &item.0 + ",";
            csv_line = csv_line + &item.1 + ",";
        }
        for item in &object.values {
            Header_line = Header_line + &item.0 + ",";
            csv_line = csv_line + &item.1 + ",";
        }
        for item in &object.foreign_keys {
            for keyitem in &item.1.primary_keys {
                Header_line = Header_line + &keyitem.0 + ",";
                csv_line = csv_line + &keyitem.1 + ",";
            }
            self.store_object(&item.1);
        }
        Header_line.pop(); //Remove trailing ,
        csv_line.pop();

        let filepath: String = object.storage_location.to_string() + ".csv";
        
        let mut FirstWrite = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filepath);
        match (FirstWrite){
            Err(x) => {},
            Ok(mut x) => {
                x.write_all(Header_line.as_bytes());
            }
        }
           
        let filepath: String = object.storage_location.to_string() + ".csv";
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(filepath);
        match file {
            Err(x) => {},
            Ok(mut x) => {
                x.write_all(csv_line.as_bytes());
            },
        }

        println!("SANATISATION NEEDS TO BE ENABLED FOR THE CSV MODULE");

        return "Your map has been added!";
    }

    fn delete_stored_data(&self, storage_location: String, primary_keys: Vec<(String, String)>) -> Result<String,String>{
        let newstring = get_not_matching_lines(storage_location.clone(), primary_keys);

        match File::create(storage_location.to_string() + ".csv"){
            Err(x) => {},
            Ok(mut file) => {
                match newstring {
                    Err(x) => return Err("An erorr occure".to_string()),
                    Ok(x) => {
                        file.write_all(x.as_bytes());
                        return Ok("The map has been deleted".to_string())
                    }
                }
                
                
            }
        }
        return Err("An error has occured".to_string());
    }
   
    fn get_stored_data(&self, storage_location: String, primary_keys: HashMap<String, Regex>) -> Vec<HashMap<String, String>> {
        println!("STORAGE RETRIEVAL NEEDS TO OUTPUT A RESULT");
        let filepath: String = storage_location.to_string() + ".csv";
        let mut returner: Vec<HashMap<String,String>> = Vec::new(); 
        let mut file = OpenOptions::new().read(true)
            .open(filepath.clone());

        match OpenOptions::new().read(true).open(filepath){
            Err(x) => {},
        
            Ok(x) => {
                let mut file_buffer = BufReader::new(&x);
                let mut first_line = String::new();
                file_buffer.read_line(&mut first_line);
                
                let all_keys: Vec<&str> = first_line.split(",").collect();
                
                for line in file_buffer.lines() {
                    match line {
                        Err(x) => {},
                        Ok(l) => {
                            let row: Vec<&str> = l.split(",").collect();
                            let mut i = 0;
                            let mut found_entry = true;

                            for x in &primary_keys {
                                let entry = row[i].to_string();
                                let entry2 = x.1.to_string();
                                found_entry = x.1.is_match(&entry);
                                i = i + 1;
                            }

                            if (found_entry) {
                                println!("HANDLE NULL AT END");
                                let mut j = 0;
                                let mut entry : HashMap<String,String> = HashMap::new();
                                for key in &all_keys {
                                    
                                    entry.insert(key.to_string(), row[j].to_string());
                                    j = j + 1;
                                }
                                returner.push(entry);
                            }
                       },
                    }
                }
            }
        }
        return returner;
    }


    fn get_by_regex_map(&self, regex: String) -> (storage_event_outcome, map) {
        println!("Getting map by Regex");

        let test_user: user = user {
            id: "01".to_string(),
            application: "Discord".to_string(),
            display_name: "test_ user".to_string(),
        };

        let test_map: map = map {
            name: "name".to_string(),
            url: "url".to_string(),
            notes: "notes".to_string(),
            uploaded: true,
            owner: test_user,
        };

        let tuple: (storage_event_outcome, map) =
            (storage_event_outcome::completed_successfully, test_map);
        return tuple;
    }

    fn get_by_regex_source(&self, regex: String) -> (storage_event_outcome, source) {
        println!("Getting source by Regex");
        
        let test_user: user = user {
            id: "01".to_string(),
            application: "Discord".to_string(),
            display_name: "test_ user".to_string(),
        };
        let test_source: source = source {
            sender: test_user,
            chatroom: "01".to_string(),
            elevated_perms: true,
        };
        let tuple: (storage_event_outcome, source) = (storage_event_outcome::completed_successfully, test_source);
        return tuple;
    }
}
