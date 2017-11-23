
pub struct User {
    pub id: String,  
    pub application: String, 
    pub display_name: String 
}

pub struct Map {
    pub name: String,
    pub url: String, 
    pub notes: String, 
    pub uploaded: bool, 
    pub owner : User
}

pub struct Source {
    pub sender: User,
    pub chatroom: String,   
    pub elevated_perms: bool 
}

pub struct Destination {
    pub chatroom: String,
    pub application: String, 
}

enum Request {
    Add (Map),
    Delete (String),
    Update (String, Map)
}