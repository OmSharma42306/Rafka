use crate::{broker::Broker, log::topic::Topic};
mod log;
mod broker;


fn main(){    
    println!("Implementi Kafka In Rust!!!");

    let mut broker = Broker::new();

    // create a topic
    let mut topic1 = Topic::new(String::from("Payments"),1);
    println!("Topic Created : {:?}",topic1);
    broker.add_topic(topic1);

    broker.start("127.0.0.1:9092");

}