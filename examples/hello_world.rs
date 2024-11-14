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

// "Hey, remember when you had to manually set up your gaming console before playing?
// This is like having a magical setup button that does it all for you!"

#[tokio::main]  // ┌─ Attribute macro that transforms our async main function
                // │ into a regular main function that sets up the runtime
                // └─ Think of it as the "plug and play" feature for async code!

// Why convert async main to regular main?
// 1. Operating systems don't understand async/await - they need a regular main() function
// 2. The runtime needs to be initialized before any async code can run
// 3. The macro handles this boilerplate for us by:
//    - Creating a tokio runtime
//    - Converting our async main into a regular main
//    - Setting up the runtime to execute our async code
//    - Handling cleanup when the program ends
// Without this macro, we'd need to manually write all this setup code ourselves

// 💡 What's happening under the hood:
// 1. Takes our async main()
// 2. Creates a runtime
// 3. Runs our async code
// 4. Cleans up when done
//
// Without this, we'd need:
// ```rust
// fn main() {
//     let runtime = tokio::runtime::Runtime::new().unwrap();
//     runtime.block_on(async_main())
// }
// ```


pub async fn main() -> Result<(), Box<dyn Error>> {
    // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.
    // "Hey, imagine texting your friend. First you need their phone number (IP:port), 
    // then you can send them messages!"

    // Connect to server (like dialing a phone number)
    let mut stream = TcpStream::connect("127.0.0.1:6142").await?;
    //  |   |    |         |           |               |     |
    //  |   |    |         |           |               |     Handle errors with ?
    //  |   |    |         |           |               Wait for connection
    //  |   |    |         |           IP:Port to connect to
    //  |   |    |         Start connection
    //  |   |    Our connection object
    //  |   Mutable (we'll write to it)
    //  Create variable

    // Q: Hey, I'm 15 and curious - what's happening in this line of code?
    // A: Great question! Think of it like making a phone call:
    //    - First, you need a phone (that's our 'stream' variable)
    //    - The phone number is "127.0.0.1:6142" (IP address:port)
    //    - 'mut' means we can use this phone to both send and receive
    //    - 'await' is like waiting for someone to pick up
    //    - The '?' at the end is like checking if the call connected properly
    //
    // Q: What's special about 127.0.0.1?
    // A: 127.0.0.1 is like calling yourself! It's known as "localhost" - 
    //    it always means "this computer". It's super useful when you're
    //    testing programs that need to talk to each other on the same machine.
    //
    // Q: Why do we need to 'await'?
    // A: Imagine texting while playing a video game. You don't want to freeze
    //    the game while waiting for replies! 'await' lets our program do other
    //    things while waiting for the connection, just like you can play your
    //    game while waiting for text responses.
    //
    // Similar examples from other codebases:
    // 1. Basic TCP Echo Server:
    //    ```rust
    //    let listener = TcpListener::bind("127.0.0.1:8080")?;
    //    for stream in listener.incoming() {
    //        handle_client(stream?);
    //    }
    //    ```
    //
    // 2. Simple TCP Client:
    //    ```rust 
    //    let stream = TcpStream::connect("127.0.0.1:8080")?;
    //    stream.write(&[1])?;
    //    ```
    println!("created stream");

    // Send message through connection (like typing and sending a text)
    let result = stream.write_all(b"hello world\n").await;
    //  |        |      |         |              |
    //  |        |      |         |              Wait for write to complete
    //  |        |      |         Message as bytes
    //  |        |      Write entire message
    //  |        Our connection
    //  Store success/failure
    println!("wrote to stream; success={:?}", result.is_ok());

    // Everything worked! (Like getting "delivered" confirmation)
    Ok(())
}
