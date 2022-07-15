use std::{env, process::exit};

pub mod icmp;
pub mod ipv4;
mod loadlibrary;

fn main() {
  let arg = env::args().nth(1).unwrap_or_else(|| {
    println!("Usage: sup DEST");
    exit(1)
  });

  let dest = ipv4::Addr::parse(arg).unwrap();
  icmp::ping(ipv4::Addr([0, 0, 0, 0])).expect("Ping failed");
}
