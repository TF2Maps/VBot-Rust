use Console_Service_Adapter;
use Console_Storage_Adapter;
use Bot::Bot;
use Bot::Tick_Outcome;

use Message_Handle_Responses::Message_Handle_Responses;
use Storage_Adapter::Storage_Adapter;

use Console_Storage_Adapter::Get_Console_Storage_Adapter;

pub fn Get_Test_Bot() -> Bot {
    
    let Test_Bot_Storage_Adapter: Storage_Adapter = Get_Console_Storage_Adapter();
    
    let Test_Bot: Bot = Bot {
        Commands: Vec::new(),
        TickFunctions: Vec::new(),    
        StorageFunctions: Test_Bot_Storage_Adapter
    };
    
    return Test_Bot;
}