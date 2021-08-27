use funs::IngerRequest;
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::SystemTime;
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
    s.write(format!("{}\n",serde_json::to_string(&request).unwrap()).as_ref()).unwrap();

    let mut ans_buf = String::default();
    let client_timeout = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() + 10000;
    while SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos()  < client_timeout {
        s.read_to_string(&mut ans_buf)?;
    }
    Ok(ans_buf)
}

