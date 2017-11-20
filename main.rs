// This is a comment, and will be ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function

mod map_datatypes;
mod StorageAdapter;
mod ConsoleImplementations;
mod bot;
mod UserHandler;
mod service;


use map_datatypes::User;
use map_datatypes::Source;
use map_datatypes::Destination;
use map_datatypes::Map;

use StorageAdapter::Storage_Adapter;
use service::Service;
use service::Tick_Outcome;

use ConsoleImplementations::Console_Bot;

fn main() {
    // The statements here will be executed when the compiled binary is called

    let TestBot: Console_Bot = Console_Bot;

    loop {
        let action: Tick_Outcome = TestBot.OnTick();
        match action {
            Tick_Outcome::DoNothing => println!("Doing Nothing"),
            Tick_Outcome::Received_Public_Message(source,msg) => println!("{0} said: {1}",source.sender.id.to_string(), msg),
            Tick_Outcome::Received_Private_Message(user,msg) => println!("{0} said: {1}",user.id.to_string(), msg)
        }
    }
    // Print text to the console
    println!("Hello World!");
}
