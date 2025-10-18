use std::path::PathBuf;
use crate::log::partition::Partition;

pub struct Topic{
    pub name : String,
    pub partitions : Vec<Partition>,
}


impl Topic {
    pub fn new( name : String, num_partitions : u32 )-> Self {
        
        let mut partitions = Vec::new();

        for i in 0..num_partitions {
            let path = PathBuf::from(format!("data/{}/partition-{}.log",name,i));
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
}
