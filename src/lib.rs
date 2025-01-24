//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.
#![no_std]

extern crate alloc;

mod error;
use core::convert::TryInto;

pub use crate::error::*;

/// Packet size traits.
#[macro_use]
pub mod size;
pub use crate::size::Size;

mod packet;
pub use crate::packet::{Packet, PacketMut, AsPacket, AsPacketMut};

/// Buffer abstractions, dynamic buffers and static buffers.
pub mod buffer;
pub use crate::buffer::Buffer;

/// Packet builder abstractions.
pub mod builder;
pub use crate::builder::Builder;

// /// Ethernet packet parser and builder.
// pub mod ether;

/// IPv4 and IPv6 packet parser and builder.
pub mod ip;

// /// ICMP packet parser and builder.
// pub mod icmp;

// /// TCP packet parser and builder.
// pub mod tcp;

/// UDP packet parser and builder.
pub mod udp;

trait ReadU16 {
    fn read_u16(&mut self) -> Result<u16>;
    fn read_u8(&mut self) -> Result<u8>;
}

impl ReadU16 for &[u8] {
    fn read_u16(&mut self) -> Result<u16> {
        let (a, b) = self.split_at_checked(2).ok_or(Error::SmallBuffer)?;
        *self = b;
        Ok(u16::from_be_bytes(a.try_into().unwrap()))
    }
    fn read_u8(&mut self) -> Result<u8> {
        let (a, b) = self.split_at_checked(1).ok_or(Error::SmallBuffer)?;
        *self = b;
        Ok(u8::from_be_bytes(a.try_into().unwrap()))
    }
}
