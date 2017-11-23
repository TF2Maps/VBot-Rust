use map_datatypes::User;
use map_datatypes::Source;
use map_datatypes::Destination;

pub struct Message_Handle_Responses
{
    pub Public_Responses: Vec<(Destination,String)>,
    pub Private_Responses: Vec<(User,String)>
}