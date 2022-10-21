use std::{net::TcpStream, io::{Write, Read}};
use bytes::{BytesMut, BufMut};
use proto::RequestCodes;

const BUF_SIZE : usize = 1024;

pub struct Client {
	server_addr: String,
	server_port: u32
}

impl Client {
	pub fn new(server_addr: String, server_port: u32) -> Client {
		Client {
			server_addr,
			server_port
		}
	}

	pub fn list_dir(&self) {
		let addr = format!("{}:{}", self.server_addr, self.server_port);
		let mut tcp_stream = TcpStream::connect(addr).expect("Could not connect to server");

		// Send request
		let mut buf = BytesMut::new();
		buf.put_u8(RequestCodes::LIST_DIR); //command
		tcp_stream.write(&buf).expect("Couldn't send request");

		// Get response
		let mut list_dir_buf = [0;BUF_SIZE];
		let bytes_read = tcp_stream.read(&mut list_dir_buf).unwrap();
		println!("Received {} bytes", bytes_read);
		println!("{}", std::str::from_utf8(&list_dir_buf).unwrap());
		
	}
}