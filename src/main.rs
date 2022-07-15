use std::{env, error::Error, process::exit};

pub mod icmp;
pub mod ipv4;
mod loadlibrary;

fn main() -> Result<(), Box<dyn Error>>{
    let arg = env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: sup DEST");
        exit(1)
    });

    let dest = arg.parse()?;
    icmp::Request::new(dest)
        .ttl(128)
        .timeout(4000)
        .data("Country roads, take me home")
        .send()?;
    Ok(())
}
