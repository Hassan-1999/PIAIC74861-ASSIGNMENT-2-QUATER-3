//Hassan Server
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};
// Handles a single client
fn handle_client(mut stream: TcpStream) -> Result<(), Error> 
{
	println!("Connection Request from (IP:Port): {}",stream.peer_addr()?);
	let mut buf = [0; 512];
	loop
	{
		let bytes_read = stream.read(&mut buf)?;
		if bytes_read == 0 { return Ok(()); }
		stream.write(&buf[..bytes_read])?;
	}
}
fn main()
{
	println!("Server Started Listening...");
	let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");
	for stream in listener.incoming() 
	{		
		match stream {
				Err(e) => { eprintln!("failed: {}", e) }
 			Ok(stream) => { thread::spawn(move || { handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));});
 		}
 	}
 }
}
