//! Creates a TAP tunnel device

#[macro_use]
pub mod error;
mod bindgen;

use std::io::{Read, Write};
use std::fs::{File, OpenOptions};
use std::os::unix::io::AsRawFd;

use bindgen::*;
use error::DeviceResult;

#[derive(Debug)]
/// A certain device
pub struct Device {
    /// The interface name
    pub name: String,

    /// The tunnel device file descriptor
    pub fd: File,
}

impl Device {
    /// Create a new tunneling `Device`
    pub fn new(name: &str) -> DeviceResult<Self> {
        // Get a file descriptor to the operating system
        let fd = OpenOptions::new().read(true).write(true).open("/dev/net/tun")?;

        // Get the default interface options
        let mut ifr = ifreq::new();

        {
            // Set the interface name
            let ifr_name = unsafe { ifr.ifr_ifrn.ifrn_name.as_mut() };
            for (index, character) in name.as_bytes().iter().enumerate() {
                if index >= IFNAMSIZ as usize - 1 {
                    bail!("Interface name too long.");
                }
                ifr_name[index] = *character as i8;
            }

            // Set the interface flags
            let ifr_flags = unsafe { ifr.ifr_ifru.ifru_flags.as_mut() };
            *ifr_flags = (IFF_TAP | IFF_NO_PI) as i16;
        }

        // Create the tunnel device
        if unsafe { ioctl(fd.as_raw_fd(), TUNSETIFF, &ifr) < 0 } {
            bail!("Device creation failed.");
        }

        Ok(Device {
            name: name.to_owned(),
            fd: fd,
        })
    }

    /// Reads a frame from the device, returns the number of bytes read
    pub fn read(&mut self, mut buffer: &mut [u8]) -> DeviceResult<usize> {
        Ok(self.fd.read(&mut buffer)?)
    }

    /// Write a frame to the device
    pub fn write(&mut self, data: &[u8]) -> DeviceResult<()> {
        self.fd.write_all(data)?;
        Ok(self.fd.flush()?)
    }
}
