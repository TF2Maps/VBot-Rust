
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
#[derive(Debug, Copy, Clone)]
pub struct storage_debugger {}

impl storage_utility for storage_debugger {
    fn store_object(&self, object: &storable_object) {
        let mut csv_line: String = "\n".to_string();
        let mut Header_line: String = "".to_string();

        for item in &object.primary_keys {
            Header_line = Header_line + &item.0 + ",";
            csv_line = csv_line + &item.1 + ",";
            println!("{}: \"{}\"", &item.0, &item.1);
        }
        for item in &object.values {
            Header_line = Header_line + &item.0 + ",";
            csv_line = csv_line + &item.1 + ",";
            println!("{}: \"{}\"", &item.0, &item.1);
        }
        for item in &object.foreign_keys {
            for keyitem in &item.1.primary_keys {
                Header_line = Header_line + &keyitem.0 + ",";
                csv_line = csv_line + &keyitem.1 + ",";
            }

            self.store_object(&item.1);
        }

        let filepath: String = object.storage_location.to_string() + ".csv";
        let mut FirstWrite = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filepath)
            .expect("Write permissions are not enabled!");

        FirstWrite.write_all(Header_line.as_bytes());

        let filepath: String = object.storage_location.to_string() + ".csv";
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(filepath)
            .expect("Write permissions are not enabled!");

        println!("SANATISATION NEEDS TO BE ENABLED FOR THE CSV MODULE");

        file.write_all(csv_line.as_bytes());
    }

    fn get_stored_data(&self, storage_location: String, primary_keys: Vec<(String, String)>) -> Vec<(String, String)> {
        println!("STORAGE RETRIEVAL NEEDS TO OUTPUT A RESULT");
        let filepath: String = storage_location.to_string() + ".csv";
        
        let mut file = OpenOptions::new()
            .read(true)
            .open(filepath)
            .expect("Write permissions are not enabled!");
        
        let mut file_buffer = BufReader::new(&file);
        
        let mut first_line = String::new();
        file_buffer.read_line(&mut first_line);
        
        let all_keys: Vec<&str> = first_line.split(",").collect();

        for line in file_buffer.lines() {
            let l = line.unwrap();
           
            let row: Vec<&str> = l.split(",").collect();
            let mut i = 0;
            let mut found_entry = true;

            for x in &primary_keys {
                let entry = row[i].to_string();
                let entry2 = x.1.to_string();
                if (entry == entry2) {
                } else {
                    found_entry = false;
                }
                i = i + 1;
            }
            if (found_entry)
            {
                let mut returner: Vec<(String,String)> = Vec::new();
                
                let mut j = 0;
                for key in &all_keys {
                    returner.push((key.to_string() , row[j].to_string()));
                    j = j + 1;
                }
                return returner;
            }
               
        }
        let returner: Vec<(String, String)> = Vec::new();
        return returner;
    }



    fn get_object_by_regex(&self, file: String, regex: String) -> String {
        return "HELLO WORLD".to_string();
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
        let tuple: (storage_event_outcome, source) =
            (storage_event_outcome::completed_successfully, test_source);
        return tuple;
    }
}
