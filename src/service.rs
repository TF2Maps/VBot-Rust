use datatypes::source;
 
pub trait services {
    
    fn send_message(message: String, desination: String ) -> Result<String,String>;
    fn check_if_admin(id: String) -> bool;
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