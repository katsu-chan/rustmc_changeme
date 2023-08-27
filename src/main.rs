use ozelot::Client;

fn main() {
    let mut client = Client::connect_unauthenticated("localhost", 25565, "dev").unwrap();
    loop {
        for packet in client.read().unwrap() {
            print!("{:?}", packet);
        }
    }
    client.close().unwrap();
    println!("Hello, world!");
}
