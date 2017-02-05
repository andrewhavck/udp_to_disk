extern crate tokio_core;
extern crate udp_to_disk;
extern crate futures;

use tokio_core::reactor::Core;
use udp_to_disk::source::udp::UdpSource;
use futures::stream::Stream;
use std::fs::File;
use std::io::Write;

pub fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let source = UdpSource::new("127.0.0.1:6666".parse().unwrap());
    let stream = source.consume(&handle);
    let mut f = File::create("something.log").unwrap();

    let parse = stream.and_then(|(_, msg)| Ok(format!("{}\n", String::from_utf8_lossy(&msg))));
    let run = parse.for_each(|result| {
        try!(f.write_all(&result.into_bytes()));
        Ok(())
    });

    core.run(run).unwrap();
}