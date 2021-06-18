use std::fmt::Display;
use std::io::prelude::*;
use std::net::TcpStream;
use log::debug;

fn handle(s : TcpStream) -> Result<(),Box<dyn Display>> {

    let mut s = std::io::BufReader::new(s);
    let mut data = String::default();
    s.read_line(&mut data).map_err(box_error)?; //Read all incoming args; seperated by '\n'
    data.pop();
    let func_id  = data.parse().map_err(box_error)?;
    data.clear();
    s.read_line(&mut data).map_err(box_error)?; //Read all incoming args; seperated by '\n'
    data.pop();
    let func_arg = data.parse().map_err(box_error)?;
    data.clear();
    s.read_line(&mut data).map_err(box_error)?; //Read all incoming args; seperated by '\n'
    data.pop();
    let func_timeout: u64 = data.parse().map_err(box_error)?;

    let func = match func_id {
        0 => funs::fib,
        1 => funs::add2,
        _ => funs::fib,
    };

    debug!("function with func_id {} with args {} and specefied timeout
           {} has...", func_id,func_arg,func_timeout);
    //println!("func_id is {} func_arg is {}",func_id,func_arg);
    let f = inger::launch(|| func(func_arg), func_timeout);

    match f {
        Ok(ans) => {
            if let inger::Linger::Completion(ans) = ans {
                s.get_mut().write(format!("{}",ans).as_ref()).map_err(box_error)?;
                debug!("Yeilded the answer {}",ans);
        }
            else {
                eprintln!("TIMEDOUT");
            }
        },
        Err(error) => eprintln!("ERROR: in LIBINGER {}",error),
    }

    Ok(())
}

fn box_error<'a,T: Display + 'a>(e: T) -> Box<dyn Display + 'a> {

    Box::new(e)
}

fn handle_wrapper(s: TcpStream) {

    if let Err(e) = handle(s) {
        eprintln!("ERROR: {}",e);
    }
}

fn main() -> std::io::Result<()> {

    env_logger::init();
    let listener =  std::net::TcpListener::bind("0.0.0.0:2000")?;
    for s in listener.incoming() {
        let s = s?;
        std::thread::spawn(|| handle_wrapper(s));
    }
    Ok(())
}
