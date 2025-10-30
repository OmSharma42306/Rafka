use std::{fmt::format, fs::File, io::{self, BufReader}, path::PathBuf};
use std::io::{BufRead};
use crate::log::{message::Message, partition::Partition};
use std::fs;
#[derive(Debug)]
pub struct Topic{
    pub name : String,
    pub partitions : Vec<Partition>,
}


impl Topic {
    pub fn new( name : String, num_partitions : u32 )-> Self {
        
        let mut partitions = Vec::new();

        for i in 0..num_partitions {
            let dir = format!("data/{}",name);
            fs::create_dir_all(&dir);
            let path: PathBuf = PathBuf::from(format!("{}/partition-{}.log",dir,i));
            partitions.push(Partition::new(i, path));
        }

        Self { name,partitions }

    }

    pub fn send(&mut self, key: Option<String>, value : String)-> std::io::Result<()>{
        
        // simple partition selection * ( Round Robbin )
        let partition_id = 0;
        let msg = self.partitions[partition_id].append_message(key,value)?;
        println!("Wrote message to {}[partition={},offset={}]",self.name,partition_id,msg.offset);
        Ok(())
    }

    pub fn read_from(&self, offset : usize,partition_id:usize)-> io::Result<Vec<Message>>{
        let partition = self.partitions.get(partition_id).ok_or_else(||io::Error::new(io::ErrorKind::NotFound,"Invalid Partition ID..."))?;

        
        let file = File::open(&partition.path)?;
        let reader = BufReader::new(file);

        let mut messages = Vec::new();

        for(i, line) in reader.lines().enumerate().skip(offset){
            let value = line?;
            messages.push(Message{
                offset : i,
                key : None,
                value
            });

            
        }
        println!("{:?} reading from partitions ",messages);
        Ok(messages)
    }
}
