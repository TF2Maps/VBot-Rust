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
use command_map::delete_command;
use command_map::add_command;
use command_map::retrieve_maps_command;
use datatypes::destination;
extern crate serde_json;
extern crate regex;
extern crate reqwest;
#[macro_use] extern crate hyper;
use hyper::header::Headers;
header! { (Authorization, "Authorization") => [String] }

fn main() {
    let console_storage: storage_debugger::storage_debugger = storage_debugger::storage_debugger{};
    
    let commands: Vec<fn(String, source, &Box<storage_utility> ) -> Vec<(destination , String)>> =  vec![add_command,retrieve_maps_command, delete_command];
    let cont: Box<storage_utility> = Box::new(console_storage);
    begin_discord_loop(commands, &cont);
}
