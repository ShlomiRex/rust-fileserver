use crate::client::Client;

pub mod client;

fn main() {
	let server_addr = String::from("127.0.0.1");
	let server_port = 8080;
    let client: Client = Client::new(server_addr, server_port);
	client.list_dir();
}
