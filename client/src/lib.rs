use std::io::prelude::*;
use serde::Serialize;
use std::net::TcpStream;
const ADDRESS: &str = "127.0.0.1:2000";


pub fn rpc_client<'a, T: Iterator<Item = &'a str>>(mut args: T) -> std::io::Result<()> {
    let input_address = args.next();           //An address
    let func_id  = args.next().unwrap_or("0"); //A u64 to be sent
    let func_arg = args.next().unwrap_or("0"); //A u64 to be sent
    let func_timeout = args.next().unwrap_or("10000"); //A u64 to be sent
    let mut address = ADDRESS;
    if let Some(address_a) = input_address {
        address = address_a;
    }

    let mut s = TcpStream::connect(&address)?;

    //write args; seperate by '\n'
    s.write(format!("{}",func_id).as_ref())?;
    s.write(format!("{}","\n").as_ref())?;
    s.write(format!("{}",func_arg).as_ref())?;
    s.write(format!("{}","\n").as_ref())?;
    s.write(format!("{}",func_timeout).as_ref())?;
    s.write(format!("{}","\n").as_ref())?;
    let mut ans_buf = String::default();
    s.read_to_string(&mut ans_buf)?;
    println!("done! {}",ans_buf);
    Ok(())
}

