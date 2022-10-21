use std::{net::TcpStream, io::Write};
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


		let mut buf = BytesMut::new();
		buf.put_u8(RequestCodes::LIST_DIR); //command
		tcp_stream.write(&buf).expect("Couldn't send request");
	}
}