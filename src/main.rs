use ozelot::{Client, serverbound::{self}};

fn main() {
    let mut client = Client::connect_unauthenticated("localhost", 25566, "dev").unwrap();
    //loop {
    //    for packet in client.read().unwrap() {
    //        print!("{:?}", packet);
    //    }
    //}

    let _ = client.send(serverbound::ChatMessage::new("Hiiiiiii!!".to_owned()));
    let _ = client.write();
    client.close().unwrap();
    println!("Hello, world!");
}
