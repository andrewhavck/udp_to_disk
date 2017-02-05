extern crate tokio_core;
extern crate futures;

use self::tokio_core::net::{UdpSocket, UdpCodec, UdpFramed};
use self::tokio_core::reactor::Handle;
use self::futures::stream::{Stream, SplitStream};

use std::io;
use std::net::SocketAddr;

pub struct LineCodec;

impl UdpCodec for LineCodec {
    type In = (SocketAddr, Vec<u8>);
    type Out = (SocketAddr, Vec<u8>);

    fn decode(&mut self, addr: &SocketAddr, buf: &[u8]) -> io::Result<Self::In> {
        Ok((*addr, buf.to_vec()))
    }

    fn encode(&mut self, (addr, buf): (SocketAddr, Vec<u8>), into: &mut Vec<u8>) -> SocketAddr {
        into.extend(buf);
        return addr;
    }
}

pub struct UdpSource {
    addr: SocketAddr,
}

impl UdpSource {
    pub fn new(addr: SocketAddr) -> Self {
        UdpSource { addr: addr }
    }

    pub fn consume(self, handle: &Handle) -> SplitStream<UdpFramed<LineCodec>> {
        let addr = self.addr;

        println!("Consuming on: {}", addr);

        let a = UdpSocket::bind(&addr, handle).unwrap();
        let (_, stream) = a.framed(LineCodec).split();
        stream
    }
}