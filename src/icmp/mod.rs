use std::mem;

use crate::icmp::icmp_sys::IcmpEchoReply;
use crate::ipv4;

use self::icmp_sys::IpOptionInformation;

mod icmp_sys;

pub struct Request {
    // TODO
}

pub struct Reply {
    // TODO
}

pub fn ping(dest: ipv4::Addr) -> Result<(), String> {
    let handle = icmp_sys::IcmpCreateFile();

    let data = "test";
    let reply_size = mem::size_of::<IcmpEchoReply>();
    let reply_buf_size = reply_size + 8 + data.len();
    let mut reply_buf = vec![0u8; reply_buf_size];
    let timeout = 4000_u32;

    match icmp_sys::IcmpSendEcho(
        handle,
        dest,
        data.as_ptr(),
        data.len() as u16,
        None,
        reply_buf.as_mut_ptr(),
        reply_buf_size as u32,
        timeout,
    ) {
        0 => Err("IcmpSendEcho failed!".to_string()),
        _ => Ok(()),
    }
}
