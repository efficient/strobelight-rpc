use std::io::prelude::*;
use std::net::TcpStream;
use std::str::FromStr;

const ADDRESS: &str = "127.0.0.1:2000";

fn main() -> std::io::Result<()> {

    let input_address: Option<String> = std::env::args().nth(1); //An address
    let func_arg: Option<String> = std::env::args().nth(2); //A u64 to be sent
    let mut address: String = String::from(ADDRESS);
    if let Some(address_a) = input_address {
        address = address_a;
    }


    let mut s = std::net::TcpStream::connect(&address)?;

    //If given a thrid arg, runs fib(arg) on server
    if let Some(arg) = func_arg {
       let x = u8::from_str(&arg).unwrap();
       s.write(&[x])?;
    }
    loop {
        let mut ans_buf = [0 as u8;8];
        s.read(&mut ans_buf); //read Big endian byte stream; should be
        let ans = u64::from_be_bytes(ans_buf);
        if(ans != 0){
            println!("The ans returned from the server is {}",ans);
            break; //The server returned an answer; break from loop
        }
    }
    Ok(())
}
