use std::io::prelude::*;
use std::net::TcpStream;
use std::str::FromStr;

const ADDRESS: &str = "127.0.0.1:2000";

fn main() -> std::io::Result<()> {

    let input_address: Option<String> = std::env::args().nth(1); //An address
    let func_id  = std::env::args().nth(2).unwrap_or(String::from("0")); //A u8 to be sent
    let func_arg = std::env::args().nth(3).unwrap_or(String::from("0")); //A u8 to be sent
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
    println!("{}",ans_buf);
    Ok(())
}
