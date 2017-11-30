// This is a comment, and will be ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function

mod Bot;
mod datatypes;


mod service;
mod storage;


mod storage_debugger;
mod service_console;


use service::tick_outcome;

use std::rc;

mod service_discord;
mod command_map;

use command_map::parse_map;

extern crate regex;
fn main() {

    parse_map("!add mapname http://url Hello World".to_string());
    parse_map("!add     mapname   http://url    Hello World".to_string());
    parse_map("!add     mapname Hello World".to_string());
    
	let console: service_console::service_console = service_console::service_console{};
    let discord: service_discord::service_discord = service_discord::service_discord{};
    
	let console_storage: storage_debugger::storage_debugger = storage_debugger::storage_debugger{};
    
	discord.send_message("Hello World".to_string(), "".to_string());
    
    loop {
        discord.tick();
        let action: tick_outcome = console.tick();
        
        match action {
            tick_outcome::nothing => println!("Doing Nothing"),
            tick_outcome::message_received(source,msg) => println!("{0} said: {1}",source.sender.id.to_string(), msg)
        }
    }
    
    println!("Hello World!");

}
