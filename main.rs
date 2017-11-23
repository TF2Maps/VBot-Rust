// This is a comment, and will be ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function

mod Bot;
mod map_datatypes;
mod Message_Handle_Responses;


mod Service_Adapter;
mod Storage_Adapter;


mod Test_Bot;
mod Console_Storage_Adapter;
mod Console_Service_Adapter;

use Console_Service_Adapter::Console_Service;
use Service_Adapter::Tick_Outcome;
use Service_Adapter::Service;

fn main() {

    let Console: Console_Service = Console_Service{};
    
    loop {
        let action: Tick_Outcome = Console.On_Tick();
        
        match action {
            Tick_Outcome::DoNothing => println!("Doing Nothing"),
            Tick_Outcome::Received_Public_Message(source,msg) => println!("{0} said: {1}",source.sender.id.to_string(), msg),
            Tick_Outcome::Received_Private_Message(user,msg) => println!("{0} said: {1}",user.id.to_string(), msg)
        }
    }
    
    println!("Hello World!");

}
