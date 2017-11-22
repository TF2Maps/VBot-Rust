// This is a comment, and will be ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function

use map_datatypes::User;
use map_datatypes::Source;
use map_datatypes::Destination;
use map_datatypes::Map;

use Message_Handle_Responses::Message_Handle_Responses;
use Storage_Adapter::Storage_Adapter;

pub struct Bot {
    pub Commands: Vec<fn( msg: String, sender: Source) -> (Message_Handle_Responses)>,
    pub TickFunctions: Vec<fn() -> (Tick_Outcome)>,    
    pub StorageFunctions: Storage_Adapter
}

pub enum Tick_Outcome {
    DoNothing,
    Received_Public_Message(Source, String),
    Received_Private_Message(User, String)
}



