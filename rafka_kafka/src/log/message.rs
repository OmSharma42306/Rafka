use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct Message{
    pub offset : u64,
    pub key : Option<String>,
    pub value : String
}

