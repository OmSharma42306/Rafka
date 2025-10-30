use std::io::{Read,Write};
use std::net::{ TcpListener,TcpStream};
use crate::log::topic::Topic;
use std::sync::{Arc,Mutex};
use std::thread;

pub struct Broker{
    topics : Arc<Mutex<Vec<Topic>>>,
}

impl Broker{
    pub fn new() -> Self{
        Broker {
            topics : Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub fn add_topic(&mut self, topic : Topic){
        self.topics.lock().unwrap().push(topic);
    }

    pub fn start(&self , addr : &str){
        let listner = TcpListener::bind(addr).expect("Failed to Bind the Tcp Server");
        println!("Broker Listening Up : {:?}",listner);

        for stream in listner.incoming(){
            match stream{
                Ok(stream)=>{
                    let topics = Arc::clone(&self.topics);
                    thread::spawn(move || {
                        handle_client(stream,topics);
                    });
                }
                Err(e)=>eprintln!("Connection Failed!: {} ",e),
            }
        }
    }
}

fn handle_client(mut stream:TcpStream,topics : Arc<Mutex<Vec<Topic>>>){
    let mut buffer = [0 ; 512];
    loop{
        let n = match stream.read(&mut buffer){
            Ok(0) => return, // connection closed
            Ok(n) => n,
            Err(_) => return,
        };

        let input = String::from_utf8_lossy(&buffer[..n]);
        println!("Receievd : {}",input);
        
        let mut parts = input.splitn(4,' ');
        println!("Parts : {:?}",parts);
        let command  = parts.next().unwrap_or("");
        println!("Command : {:?}",command);

        match command {
            "PRODUCE" => {
                let topic_name = parts.next().unwrap_or("");
                let key = parts.next().map(|k| k.to_string());
                let value = parts.next().unwrap_or("").to_string();

                let mut topics_guard = topics.lock().unwrap();  
                if let Some(topic) = topics_guard.iter_mut().find(|t| t.name == topic_name){
                    match topic.send(key, value) {
                        Ok(_) => {
                            stream.write_all(b"OK\n").unwrap();
                        }
                        Err(e)=>{
                            let err_msg = format!("ERROR: {}\n", e);
                            stream.write_all(err_msg.as_bytes()).unwrap();
                        }    
                    }
                }else{
                    stream.write_all(b"ERROR: Unknown topic\n").unwrap();
                }
            }

            "CONSUME"=>{
                let topic_name = parts.next().unwrap_or("");
                  let partition_str = parts.next().unwrap_or("0");
                let offset_str = parts.next().unwrap_or("0");
                let offset : usize = offset_str.parse().unwrap_or(0);
                
                let partition_id: usize = partition_str.parse().unwrap_or(0);
                let  topics_guard = topics.lock().unwrap();  
                if let Some(topic) = topics_guard.iter().find(|t| t.name == topic_name){
                    match topic.read_from(offset,partition_id) {
                            Ok(messages)=>{
                                for msg in messages{
                                    let line = format!("{}:{}\n",msg.offset,msg.value);
                                    stream.write_all(line.as_bytes()).unwrap();
                                }
                            }
                            Err(e)=>{
                                let err_msg = format!("ERROR : {}\n",e);
                                stream.write_all(err_msg.as_bytes()).unwrap();
                            }
                    }
                }

            }
            _ =>{
                stream.write_all(b"ERROR: Unknown command\n").unwrap();
            }
        }
        let response = "ACK\n";
        stream.write_all(response.as_bytes()).unwrap();
    }
}