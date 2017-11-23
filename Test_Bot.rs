use Console_Service_Adapter;
use Console_Storage_Adapter::Console_Storage_Adapter;
use Bot::Bot;
use Bot::Tick_Outcome;

use Message_Handle_Responses::Message_Handle_Responses;



pub fn Get_Test_Bot() -> Bot {
    
    let console: Console_Storage_Adapter = Console_Storage_Adapter{};
    
    let Test_Bot: Bot = Bot {
        Commands: Vec::new(),
        TickFunctions: Vec::new(),    
        StorageFunctions: Box::new(console)
    };
    
    return Test_Bot;
}