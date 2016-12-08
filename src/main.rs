#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate ioctl;
extern crate futures;
extern crate libc;
extern crate mio;
extern crate tokio_core as core;

mod datagram_framed;
mod tun;

use std::os::unix::io::FromRawFd;
use std::os::unix::io::IntoRawFd;

use core::io::Io;
use futures::Stream;

error_chain! {
    links {
        Tun(tun::Error, tun::ErrorKind);
    }
    foreign_links {
        Io(::std::io::Error);
    }
}

fn real_main() -> Result<()> {
    let tun = tun::Tun::new("pote")?;
    let mut core = core::reactor::Core::new()?;
    let pote = unsafe {
        mio::deprecated::unix::UnixStream::from_raw_fd(tun.file.into_raw_fd())
    };
    let file = core::reactor::PollEvented::new(pote, &core.handle())?;
    let stream = file.framed(datagram_framed::Parser).and_then(|msg| {
        println!("{}", msg.len());
        Ok(())
    }).for_each(|_| {
        Ok(())
    });
    core.run(stream)?;
    Ok(())
}

fn main() {
    if let Err(e) = real_main() {
        println!("{}", e);
        for cause in e.iter().skip(1) {
            println!("  caused by: {}", cause);
        }
        if let Some(b) = e.backtrace() {
            println!("{:?}", b);
        }
    }
    println!("done");
}
