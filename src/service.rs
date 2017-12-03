use datatypes::source;
 
pub trait services {
    
    fn send_message(message: String, desination: String ) -> activity_outcome;
    fn initialise() -> activity_outcome;
    fn tick() -> tick_outcome;
}

pub enum tick_outcome {
    nothing,
    message_received(source, String)
}

pub enum activity_outcome {
    success,
    fail
}