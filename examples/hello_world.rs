//! A simple client that opens a TCP stream, writes "hello world\n", and closes
//! the connection.
//!
//! To start a server that this client can talk to on port 6142, you can use this command:
//!
//!     ncat -l 6142
//!
//! And then in another terminal run:
//!
//!     cargo run --example hello_world

#![warn(rust_2018_idioms)]
// This is a compiler directive that warns if code doesn't follow Rust 2018 idioms.
// It helps ensure modern Rust coding practices are used, like:
// - Using async/await syntax instead of older futures 
// - Using ? operator for error handling
// - Using dyn for trait objects
// - And other 2018 edition best practices
//
// Q: No, this program still follows modern Rust idioms. The 2018 idioms are a subset
// of good practices that carried forward into 2021 edition. The 2021 edition added new
// features but didn't invalidate 2018 idioms. This directive helps catch common
// anti-patterns regardless of edition.

// 📦 Core Libraries Used:
//
// tokio::io::AsyncWriteExt  
//   |      |     |
//   |      |     Trait for async write operations
//   |      I/O module
//   Async runtime
//
// Q: What is a trait and why is AsyncWriteExt imported here?
// A: A trait is like an interface in Java - it defines a set of methods that a type must implement.
// AsyncWriteExt is a trait that adds async write methods like write_all() to any type that can be
// written to (like our TcpStream). When we import it, we get access to these write methods.
// Without importing this trait, we wouldn't be able to call write_all() on our stream.
// Think of it like plugging in a power strip - the trait adds new capabilities to existing types.
//



// tokio::net::TcpStream
//   |      |    |
//   |      |    Async TCP connection type
//   |      Networking module
//   Async runtime
//
// std::error::Error
//   |    |     |
//   |    |     Base error trait
//   |    Error handling module
//   Standard library
//
// Memory Layout of TcpStream:
// ┌─────────────────┐
// │   TcpStream     │ 
// ├─────────────────┤
// │ Socket Handle   │ ──► Operating System Socket
// │ Buffer          │ ──► Internal Read/Write Buffer
// └─────────────────┘

use tokio::io::AsyncWriteExt;  // For write_all() method
use tokio::net::TcpStream;     // For TCP connections
use std::error::Error;         // For error handling

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.
    let mut stream = TcpStream::connect("127.0.0.1:6142").await?;
    println!("created stream");

    let result = stream.write_all(b"hello world\n").await;
    println!("wrote to stream; success={:?}", result.is_ok());

    Ok(())
}
