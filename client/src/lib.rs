use funs::IngerRequest;
use std::io::prelude::*;
use std::net::TcpStream;
const ADDRESS: &str = "127.0.0.1:2000";

pub fn rpc_client<'a, T: Iterator<Item = &'a str>>(mut args: T) -> std::io::Result<String> {
    let input_address = args.next();           //An address
    let request = IngerRequest {
        func_id: args.next().unwrap_or("0"),
        func_arg: args.next().unwrap_or("0"),
        func_timeout: args.next().unwrap_or("500"),
    };

    let mut address = ADDRESS;
    if let Some(address_a) = input_address {
        address = address_a;
    }

    let mut s = TcpStream::connect(&address)?;
    let timeout_set: u64 = request.func_timeout.parse().unwrap();

    s.set_read_timeout(Some(std::time::Duration::from_micros(timeout_set))).unwrap();

    s.write(format!("{}\n",serde_json::to_string(&request).unwrap()).as_ref()).unwrap();
    let mut ans_buf = String::default();
    s.read_to_string(&mut ans_buf)?;

    //Err handling; server times-out before client
    if ans_buf == ""
    {
        Err(std::io::Error::new(std::io::ErrorKind::WouldBlock,"Resource temporarily unavailable"))?
    }

    Ok(ans_buf)
}

