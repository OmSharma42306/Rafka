use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct Message{
    pub offset : usize,
    pub key : Option<String>,
    pub value : String
}

