use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use std::net::TcpStream;
const ADDRESS: &str = "127.0.0.1:2000";

#[derive(Serialize, Deserialize,Debug)]
pub struct  IngerRequest<'a> {
    func_id: &'a str,
    func_arg: &'a str,     //In the future, might need to change this to a generic queue
    func_timeout: &'a str,
}

pub fn rpc_client<'a, T: Iterator<Item = &'a str>>(mut args: T) -> std::io::Result<()> {
    let input_address = args.next();           //An address
    let request = IngerRequest {
        func_id: args.next().unwrap_or("0"), //A u64 to be sent
        func_arg: args.next().unwrap_or("0"), //A u64 to be sent
        func_timeout: args.next().unwrap_or("10000"), //A u64 to be sent
    };
    let mut address = ADDRESS;
    if let Some(address_a) = input_address {
        address = address_a;
    }
    let mut s = TcpStream::connect(&address)?;
    s.write(format!(r#"{}"#,serde_json::to_string(&request).unwrap()).as_ref()).unwrap();

    let mut ans_buf = String::default();
    //s.read_to_string(&mut ans_buf)?;
    Ok(())
}

