use std::fs::{OpenOptions};

use std::fs::{OpenOptions};
use std::io::{ Write,Seek,SeekFrom };
use std::path::PathBuf;

use crate::log::message::Message;
use serde_json;

struct Partition{
    id : u32,
    path : String,
    next_offset : u64,
}

impl Partition{
    pub fn new(id : i32, path : PathBuf) -> Self{
        Self { id, path, next_offset: 0 }
    }

    pub fn append_message(&mut self, key : Option<String>, value : String)-> std::io::Result<Message>{
        let msg = Message {
            offset : self.next_offset,
            key,
            value,
        };


        // Open log file for append....

        let mut file = OpenOptions::new().create(true).append(true).open(&self.path)?;

        // Seriallize message as JSON.

        let serialized = serde_json::to_string(&msg)? + "\n";
        file.write_all(serialized.as_bytes())?;

        // Increment offset for next write...
        self.next_offset += 1;

        Ok(msg)


    }
}