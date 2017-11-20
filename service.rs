use map_datatypes::User;
use map_datatypes::Source;
use map_datatypes::Destination;
use map_datatypes::Map;


pub enum Activity_Outcome {
    success,
    fail
}
pub enum Tick_Outcome {
    DoNothing,
    Received_Public_Message(Source, String),
    Received_Private_Message(User, String)
}


pub trait Service {
    fn SendPrivateMessage(&self, message: String, desination_user_id: String )-> Activity_Outcome;
    fn SendPublicMessage(&self, message: String, desination_chatroom_id: String ) -> Activity_Outcome;
    fn Login (&self) -> Activity_Outcome;
    fn OnTick(&self) -> Tick_Outcome;
}