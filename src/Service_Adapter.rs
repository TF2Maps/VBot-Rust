use map_datatypes::User;
use map_datatypes::Source;
use map_datatypes::Destination;
use map_datatypes::Map;
 
pub trait Service {
    fn Send_Public_Message(&self, message: String, desination: String ) -> Activity_Outcome;
    fn Login(&self) -> Activity_Outcome;
    fn On_Tick(&self) -> Tick_Outcome;
}

pub enum Tick_Outcome {
    DoNothing,
    Received_Public_Message(Source, String)
}

pub enum Activity_Outcome {
    success,
    fail
}