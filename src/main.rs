use std::{env, error::Error, process::exit};

pub mod icmp;
pub mod ipv4;
mod loadlibrary;

fn main() -> Result<(), Box<dyn Error>> {
    let arg = env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: sup DEST");
        exit(1)
    });

    use icmp::Request;
    let dest = arg.parse()?;
    let data = "Country roads, take me home";

    println!("\n");
    println!("Pinging {:?} with {} bytes of data", dest, data.len());

    use std::{thread::sleep, time::Duration};

    for _ in 0..4 {
        match Request::new(dest)
            .ttl(128)
            .timeout(4000)
            .data(data)
            .send() {
                Ok(res) => println!("Reply from {:?}: bytes={} time={:?} TTL={}", res.addr, res.data.len(), res.rtt, res.ttl),
                Err(_) => println!("Something wrent wrong!"),
             }

             sleep(Duration::from_secs(1))
    }
    println!("\n");
    Ok(())
}
