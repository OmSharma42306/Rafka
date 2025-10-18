use crate::log::topic::Topic;
mod log;


fn main()-> std::io::Result<()> {    
    println!("Implementi Kafka In Rust!!!");

    let mut orders = Topic::new("orders".to_string(),2);
    orders.send(Some("user1".into()),"OM Sharma".into())?;
    orders.send(Some("user2".into()), "Sagar Sharma".into())?;
    Ok(())
}


