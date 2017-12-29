// This is a comment, and will be ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function


mod Bot;
mod datatypes;


mod service;
use service_discord::get_discord_messages;
use service::services;
mod storage;
use storage::storable;
use storage::storage_utility;

mod storage_debugger;
use service::tick_outcome;
mod service_console;
mod service_discord;
mod command_map;
use datatypes::user;
use datatypes::source;
use datatypes::map;
use std::collections::HashMap;
use service_discord::get_roles_that_are_admin;
use service_discord::get_user_roles;
use service_discord::Get_And_Splice;
use service_discord::discord_get_latest_message_id;
use service_discord::begin_discord_loop;
use std::fs::File;
use std::io::prelude::*;
extern crate serde_json;
extern crate regex;
extern crate reqwest;
#[macro_use] extern crate hyper;
use hyper::header::Headers;
header! { (Authorization, "Authorization") => [String] }

fn main() {
    let console_storage: storage_debugger::storage_debugger = storage_debugger::storage_debugger{};
    console_storage.delete_stored_data(
         
            "Maps".to_string(), 
            vec![
                ("Map name".to_string(), "test name".to_string())
            ])
        ;

    begin_discord_loop();
    
    /*
    let url: String = "https://www.google.com.au/search?hl=en&source=hp&ei=RF8iWvnfDMaB8gXKj5d4&q=trigger+site%3Adeveloper.valvesoftware.com".to_string();
    let output: String = Get_And_Splice(url.to_string(),"url?q=".to_string(),"&amp".to_string());
    let msgid: String = discord_get_latest_message_id("346831363787456513");

    let guild = "217585440457228290";
    let all_channels: Vec<String> = service_discord::get_all_channels_in_guild(guild);
    
    let mut all_messages: Vec<(source, String)> = Vec::new();
    
    for x in &all_channels {
        let mut channel_messages = get_discord_messages(x.to_string(), "/messages".to_string()/*"?after=388596943414362114".to_string()*/);
        all_messages.append(&mut channel_messages);

    }

    for x in &all_messages {
        let mut isadmin: bool = false;
        let all_user_roles = get_user_roles(guild, &x.0.sender.id);
        for x in &all_user_roles {
           if (get_roles_that_are_admin(guild).contains(x)) {
               isadmin = true;
           }
        }
        if isadmin {
            println!("{} ({}): {}", x.0.sender.display_name, x.0.sender.id, x.1);
        } else {
            println!("Admin Said: {} ({}): {}", x.0.sender.display_name, x.0.sender.id, x.1);
        }
    }
    
    
    let sender = user {
        id: "01".to_string(),  
        application: "Discord".to_string(), 
        display_name: "test_ user".to_string() 
    };

    let TestMap: map = map {
        name: "test name".to_string(),
        url: "test url".to_string(),
        notes: "test notes".to_string(),
        uploaded: false,
        owner: user {
                id: "01".to_string(),  
                application: "Discord".to_string(), 
                display_name: "test_ user".to_string() 
        },
    };
    
	let console: service_console::service_console = service_console::service_console;
    let discord: service_discord::service_discord = service_discord::service_discord;
    
	let console_storage: storage_debugger::storage_debugger = storage_debugger::storage_debugger{};
    console_storage.store_object(&TestMap.convert_to_storable());
    println!("{:?}",
        console_storage.get_stored_data( "discord".to_string(), vec!
        [
            ("configID".to_string(), "1".to_string())
        ])
    );

    let discord_configuration: HashMap<String, String> = console_storage.get_stored_data
        ("discord".to_string(), vec![("configID".to_string(), "1".to_string())]);
    
    let mut webhook = String::new();

    match discord_configuration.get(&"Chatroom".to_string()) {
        Some(hook) => {
            webhook = hook.clone()
        }
        None => println!("ERROR")
    }
    service_discord::service_discord::send_message("Hello World".to_string(), webhook);
	
    
    loop {
        
        let action: tick_outcome = service_console::service_console::tick();
        
        match action {
            tick_outcome::nothing => println!("Doing Nothing"),
            tick_outcome::message_received(source,msg) => println!("{0} said: {1}",source.sender.id.to_string(), msg)
        }
    }
    
    println!("Hello World!");
    */
}
