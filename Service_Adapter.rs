use map_datatypes::User;
use map_datatypes::Source;
use map_datatypes::Destination;
use map_datatypes::Map;

pub struct Service {
    pub Send_Private_Message: fn ( message: String, desination_user_id: String )-> Activity_Outcome,
    pub Send_Public_Message: fn (message: String, desination_chatroom_id: String ) -> Activity_Outcome,
    pub Login: fn() -> Activity_Outcome,
    pub On_Tick: fn() -> Tick_Outcome
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