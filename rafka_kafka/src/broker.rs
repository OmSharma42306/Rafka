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
        // TODO : Parsing Commands...
        let response = "ACK\n";
        stream.write_all(response.as_bytes()).unwrap();
    }
}