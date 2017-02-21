//! Basic tunnel device handling in Rust

#[macro_use]
extern crate tokio_core;
extern crate futures;

#[macro_use]
pub mod error;
pub mod device;
mod bindgen;

use device::Device;
use error::TunnelResult;

use std::io;
use std::net::SocketAddr;

use futures::{Future, Poll};
use tokio_core::net::UdpSocket;
use tokio_core::reactor::Handle;

/// The main tunnel structure
pub struct Tunnel {
    /// A tunneling TAP device
    device: Device,

    /// The VPN server socket
    server: UdpSocket,

    /// An internal packet buffer
    buffer: Vec<u8>,

    /// Things to send
    to_send: Option<(usize, SocketAddr)>,
}

impl Tunnel {
    /// Creates a new `Tunnel`
    pub fn new(handle: &Handle) -> TunnelResult<Self> {
        // Create a tunneling device
        let device = Device::dummy("tun.rs")?;

        // Create a server for the tunnel
        let addr = "127.0.0.1:8080".to_owned().parse()?;
        let server = UdpSocket::bind(&addr, handle)?;

        Ok(Tunnel {
            device: device,
            server: server,
            buffer: vec![0; 1500],
            to_send: None,
        })
    }
}

impl Future for Tunnel {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        loop {
            // Check first if a message needs to be processed
            if let Some((size, peer)) = self.to_send {
                // Write the message to the tunnel device
                let send_bytes = try_nb!(self.device.write(&self.buffer[..size]));

                // Set `to_send` to `None` if done
                self.to_send = None;

                println!("Wrote {}/{} bytes from {} to tunnel device", send_bytes, size, peer);
            }

            // If `to_send` is `None`, we can receive the next message from the client
            self.to_send = Some(try_nb!(self.server.recv_from(&mut self.buffer)));

            // Flush the device file descriptor
            try_nb!(self.device.flush());

            // Read from the tunnel device and write to the client
            let read_bytes = try_nb!(self.device.read(&mut self.buffer));
            let amt = try_nb!(self.server.send_to(&self.buffer[..read_bytes], &peer));
            println!("Read {} bytes from tunnel device", read_bytes);
        }
    }
}

#[test]
fn tunnel() {
    use tokio_core::reactor::Core;

    // Setup tokio
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    // Run the core with the tunnel
    let tunnel = Tunnel::new(&handle).unwrap();
    core.run(tunnel).unwrap();
}
