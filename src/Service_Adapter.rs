use map_datatypes::User;
use map_datatypes::Source;
use map_datatypes::Destination;
use map_datatypes::Map;
 
pub trait Service {
    fn Send_Private_Message(&self, message: String, desination_user_id: String ) -> Activity_Outcome;
    fn Send_Public_Message(&self, message: String, desination_chatroom_id: String ) -> Activity_Outcome;
    fn Login(&self) -> Activity_Outcome;
    fn On_Tick(&self) -> Tick_Outcome;
}

pub enum Tick_Outcome {
    DoNothing,
    Received_Public_Message(Source, String),
    Received_Private_Message(User, String)
}

pub enum Activity_Outcome {
    success,
    fail
}