use crate::server::Server;

pub mod server;

fn  main() {
	let mut server: Server = Server::new();
	server.start();
}
