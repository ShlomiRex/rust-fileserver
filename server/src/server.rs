use std::{net::{TcpListener, TcpStream}, io::{Read}};
use std::fs;
use proto::RequestCodes;

const BUF_SIZE : usize = 1024;

pub struct Server {
	running: bool
}

impl Server {
	pub fn new() -> Server {
		Server {
			running: false
		}
	}

	pub fn start(&mut self) {
		let addr = "127.0.0.1";
		let port = 8080;
		let bind_addr = format!("{}:{}", addr, port);

		println!("Server is starting");
		let listener = TcpListener::bind(bind_addr).expect("Could not bind server");
		self.running = true;
		println!("Server is running at address: {}, port: {}", addr, port);

		while self.running {
			let stream = listener.accept().expect("Could not accept client");
			let mut tcp_stream = stream.0;
			let socket_addr = stream.1;
			println!("Accepted client, address: {}", socket_addr);
			self.handle_client(&mut tcp_stream);
		}
	}

	fn handle_client(&self, tcp_stream: &mut TcpStream) {
		let mut buf = [0;BUF_SIZE];

		let bytes_read = tcp_stream.read(&mut buf).expect("Error reading message");

		let req_str: String = match buf[0] {
			RequestCodes::LIST_DIR => String::from("List directory"), 
			RequestCodes::FILE_DOWNLOAD => String::from("File download"),
			_ => {
				return
			}
		};

		println!("Read {} bytes, client requests: {}", bytes_read, req_str);

		if buf[0] == RequestCodes::LIST_DIR {
			self.list_dir();
		} else if buf[0] == RequestCodes::FILE_DOWNLOAD {
			//TODO: Add stuff
		}

	}

	fn list_dir(&self) {
		let dir_path = String::from("C:\\Users\\Shlomi\\Desktop\\");
		let paths = fs::read_dir(dir_path).unwrap();
		for path in paths {
			println!("{}", path.unwrap().path().display());
		}
	}
}