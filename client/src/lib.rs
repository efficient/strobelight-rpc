use std::io::prelude::*;
use std::net::TcpStream;
use std::str::FromStr;
const ADDRESS: &str = "127.0.0.1:2000";



pub fn rpc_client<T: Iterator<Item = String>>(mut args: T) -> std::io::Result<()> {
    let input_address: Option<String> = args.next(); //An address
    let func_id  = args.next().unwrap_or(String::from("0")); //A u8 to be sent
    let func_arg = args.next().unwrap_or(String::from("0")); //A u8 to be sent
    let mut address: String = String::from(ADDRESS);
    if let Some(address_a) = input_address {
        address = address_a;
    }

    let mut s = std::net::TcpStream::connect(&address)?;


    //write args; seperate by '\n'
    s.write((&func_id).as_bytes())?;
    s.write((&"\n").as_bytes());
    s.write((&func_arg).as_bytes())?;
    s.write((&"\n").as_bytes());
    let mut ans_buf = String::default();
    s.read_to_string(&mut ans_buf);
    Ok(())
}

