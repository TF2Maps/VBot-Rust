use map_datatypes::User;
use map_datatypes::Source;
use map_datatypes::Destination;

pub trait UserHandler {
    fn Handle_Message(&self, msg: String, sender: Source) -> (Message_Handle_Responses);
}

pub struct Message_Handle_Responses
{
    pub Public_Responses: Vec<(Destination,String)>,
    pub Private_Responses: Vec<(User,String)>
}