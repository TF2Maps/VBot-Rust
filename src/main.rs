// This is a comment, and will be ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function

mod Bot;
mod datatypes;


mod service;
use service::services;
mod storage;
use storage::storable;
use storage::storage_utility;

mod storage_debugger;
use service::tick_outcome;
mod service_console;
mod service_discord;
mod command_map;
use command_map::parse_map;
use datatypes::user;
use datatypes::source;
use datatypes::map;
extern crate regex;
fn main() {
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
        console_storage.get_stored_data( "config".to_string(), vec!
        [
            ("config".to_string(),"Data".to_string())
        ])
    );

    //parse_map("!add oldmap http://url    Hello World".to_string(), sender);
    service_discord::service_discord::send_message("Hello World".to_string(), "".to_string());
	
    
    loop {
        
        let action: tick_outcome = service_console::service_console::tick();
        
        match action {
            tick_outcome::nothing => println!("Doing Nothing"),
            tick_outcome::message_received(source,msg) => println!("{0} said: {1}",source.sender.id.to_string(), msg)
        }
    }
    
    println!("Hello World!");

}
