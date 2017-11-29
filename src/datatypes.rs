
pub struct user {
    pub id: String,  
    pub application: String, 
    pub display_name: String 
}

pub struct source {
    pub sender: user,
    pub chatroom: String,   
    pub elevated_perms: bool 
}

pub struct map {
    pub name: String,
    pub url: String, 
    pub notes: String, 
    pub uploaded: bool, 
    pub owner : user
}

pub struct destination {
    pub chatroom: String,
    pub application: String, 
}

enum request {
    add (map),
    delete (String),
    update (String, map)
}