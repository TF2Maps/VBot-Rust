use std::collections::HashMap;
use storage::storable_object;
use storage::storable;


pub struct user {
    pub id: String,  
    pub application: String, 
    pub display_name: String 
}

impl storable for user {
    fn convert_to_storable(self) -> storable_object {
        let mut primarykeys: Vec<(String, String)> = Vec::new();
        primarykeys.push(("ID".to_string(), self.id));
        primarykeys.push(("Application".to_string(), self.application));
        
        let mut values: Vec<(String, String)> = Vec::new();
        values.push(("Display Name".to_string(), self.display_name));

        let converted_object: storable_object = storable_object {
            storage_location: "Users".to_string(),
            primary_keys: primarykeys,
            values: values,
            foreign_keys: Vec::new(),
        };
        return converted_object;
    }    
}

impl storable for map {
    fn convert_to_storable(self) -> storable_object {
        let mut primarykeys: Vec<(String, String)> = Vec::new();
        primarykeys.push(("Map name".to_string(), self.name));

        let mut values: Vec<(String, String)> = Vec::new();
        values.push(("Url".to_string(), self.url));
        values.push(("Notes".to_string(), self.notes));
        values.push(("Uploaded".to_string(), self.uploaded.to_string()));
        
        let mut foreign_keys: Vec<(String, storable_object)> = Vec::new();
        foreign_keys.push(("Owner".to_string(), self.owner.convert_to_storable()));
        let converted_object: storable_object = storable_object {
            storage_location: "Maps".to_string(),
            primary_keys: primarykeys,
            values: values,
            foreign_keys: foreign_keys,
        };
        return converted_object;
    }    
}

pub struct source {
    pub sender: user,
    pub chatroom: String,   
    pub elevated_perms: bool 
}

pub struct map {
    pub name: String,
    pub url: String, 
    pub notes: String, 
    pub uploaded: bool, 
    pub owner : user
}

pub struct destination {
    pub chatroom: String,
    pub application: String, 
}

enum request {
    add (map),
    delete (String),
    update (String, map)
}