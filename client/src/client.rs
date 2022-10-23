use std::{net::TcpStream, io::{Write, Read}};
use bytes::{BytesMut, BufMut};
use proto::RequestCodes;

const BUF_SIZE : usize = 256;

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
		let mut list_dir_size_buf = [0;4];
		tcp_stream.read_exact(&mut list_dir_size_buf).unwrap();

		let list_dir_size = u32::from_be_bytes(list_dir_size_buf);

		let mut vec = vec![0; list_dir_size as usize];
		tcp_stream.read_exact(&mut vec).unwrap();
		println!("{}", std::str::from_utf8(&vec).unwrap());
	}
}