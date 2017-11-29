use datatypes::user;
use datatypes::source;
use datatypes::destination;
use datatypes::map;
 
pub trait service {
    fn send_message(&self, message: String, desination: String ) -> activity_outcome;
    fn initialise(&self) -> activity_outcome;
    fn tick(&self) -> tick_outcome;
}

pub enum tick_outcome {
    nothing,
    message_received(source, String)
}

pub enum activity_outcome {
    success,
    fail
}