mod loadlibrary;

use core::fmt;
use std::{
    ffi::c_void,
    mem::{self},
};

use loadlibrary::Library;
use pretty_hex::PrettyHex;

struct IpAddr([u8; 4]);

impl fmt::Debug for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b, c, d] = self.0;

        write!(f, "{}.{}.{}.{}", a, b, c, d)
    }
}

fn main() {
    let iphlp = Library::new("IPHLPAPI.dll").unwrap();
    let IcmpSendEcho: IcmpSendEcho = iphlp.get_proc("IcmpSendEcho").unwrap();
    let IcmpCreateFile: IcmpCreateFile = iphlp.get_proc("IcmpCreateFile").expect("error");

    let handle: Handle = IcmpCreateFile();

    println!("handle: {:?}", handle);

    let data = "test";

    let reply_size = mem::size_of::<IcmpEchoReply>();
    let reply_buf_size = reply_size + 8 + data.len();

    let mut reply_buf = vec![0u8; reply_buf_size];

    let ip_options: IpOptionInformation = IpOptionInformation {
        ttl: 128,
        tos: 0,
        flags: 0,
        options_size: 0,
        options_data: 0,
    };

    let ret = unsafe {
        IcmpSendEcho(
            handle,
            IpAddr([8, 8, 8, 8]),
            data.as_ptr(),
            data.len() as u16,
            Some(&ip_options),
            reply_buf.as_mut_ptr(),
            reply_buf_size as u32,
            5000,
        )
    };

    if (ret == 0) {
        panic!("IcmpSendEcho failed! ret = {}", ret)
    }

    unsafe {
        let reply: &IcmpEchoReply = mem::transmute(&reply_buf[0]);

        let reply_data: *const u8 = mem::transmute(&reply_buf[reply_size + 8]);

        let reply_data = std::slice::from_raw_parts(reply_data, reply.data_size as usize);

        println!("{:#?}", reply_data.hex_dump())
    };
}

type Handle = *const c_void;

type IcmpSendEcho = extern "stdcall" fn(
    handle: Handle,
    dest: IpAddr,
    request_data: *const u8,
    request_size: u16,
    request_options: Option<&IpOptionInformation>,
    reply_buffer: *mut u8,
    reply_size: u32,
    timeout: u32,
) -> u32;

type IcmpCreateFile = extern "stdcall" fn() -> Handle;

#[repr(C)]
#[derive(Debug)]
struct IpOptionInformation {
    ttl: u8,
    tos: u8,
    flags: u8,
    options_size: u8,
    options_data: u32,
}

#[repr(C)]
#[derive(Debug)]
struct IcmpEchoReply {
    address: IpAddr,
    status: u32,
    rtt: u32,
    data_size: u16,
    reserved: u16,
    data: *const u8,
    options: IpOptionInformation,
}
