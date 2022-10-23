use std::{net::{TcpListener, TcpStream}, io::{Read, Write}};
use std::fs;
use bytes::{BytesMut, BufMut};
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
			self.list_dir(tcp_stream);
		} else if buf[0] == RequestCodes::FILE_DOWNLOAD {
			//TODO: Add stuff
		}

	}

	fn list_dir(&self, tcp_stream: &mut TcpStream) {
		let dir_path = String::from("C:\\Users\\Shlomi\\Desktop\\"); //TODO: Fow now, only for testing, we use this.
		let paths = fs::read_dir(dir_path).unwrap();

		let mut paths_vec: Vec<String> = Vec::new();
		let mut total_bytes_to_write: usize = 0;
		let mut buf = BytesMut::new();

		// Determine amount of bytes to send
		for path in paths {
			let mut p2 = path.unwrap().path().into_os_string().into_string().unwrap();
			p2.push('\n');
			total_bytes_to_write += p2.len();
			paths_vec.push(p2);
		}

		// Send client amount of bytes to read
		buf.put_u32(total_bytes_to_write as u32);

		// Push to send buffer
		for x in paths_vec {
			buf.put(x.as_bytes());
		}
		
		// Send
		let bytes_written = tcp_stream.write(&buf).unwrap();
		println!("Sent {} bytes", bytes_written);
		
	}


}