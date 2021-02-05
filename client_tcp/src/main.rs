//Hassan Code
//Client
use std::net::TcpStream;
use std::str;
use std::io::{BufRead, BufReader, Write};
fn main()
{
	let mut stream = TcpStream::connect("127.0.0.1:8888").expect("Could not connect to server");
	println!("{:?}",stream);
	let msg= "Hello\n";
 	let mut buffer: Vec<u8> = Vec::new();
 	println!("Message Sent: {}",msg);
 	stream.write(msg.as_bytes()).expect("Failed to write to server");
 	let mut reader = BufReader::new(&stream);
 	reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
 	println!("Server Response: {}", str::from_utf8(&buffer).expect("Could not write buffer as string"));
 	println!("Connection Terminated")
}

