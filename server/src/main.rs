use log::{debug,error,info};
use serde::{Serialize, Deserialize};
use std::fmt::Display;
use std::io::prelude::*;
use std::net::TcpStream;


#[derive(Serialize, Deserialize,Debug)]
struct IngerRequest<'a> {
    func_id: &'a str,
    func_arg: &'a str,
    func_timeout: &'a str,
}

fn handle(s : TcpStream) -> Result<(),Box<dyn Display>> {

    let mut s = std::io::BufReader::new(s);
    let mut data = String::default();
    s.read_line(&mut data).map_err(box_error)?;
    let request: IngerRequest = serde_json::from_str(&data).map_err(box_error)?;

    debug!("done parsing struct");
    debug!("{} {} {}",request.func_id,request.func_arg,request.func_timeout);

    let func_id: i64 = request.func_id.parse().map_err(box_error)?;
    let func_arg: u64 = request.func_arg.parse().map_err(box_error)?;
    let func_timeout: u64 = request.func_timeout.parse().map_err(box_error)?;
    /* Dead code; should probably throw out
    let mut data = String::default();
    s.read_line(&mut data).map_err(box_error)?;
    data.pop();
    let func_id: i64 = data.parse().map_err(box_error)?;
    data.clear();
    s.read_line(&mut data).map_err(box_error)?;
    data.pop();
    let func_arg = data.parse().map_err(box_error)?;
    data.clear();
    s.read_line(&mut data).map_err(box_error)?;
    data.pop();
    let func_timeout: u64 = data.parse().map_err(box_error)?; //NOTE: The typechecker needed explcit type anno. for parse()
    */
    let func = match func_id.abs() {
        1 => funs::fib,
        2 => funs::add2,
        _ => funs::fib,
    };

    debug!("function with func_id {} with args {} and func timeout {} has..."
           ,func_id,func_arg,func_timeout);

    if func_id < 0 {    //for all negative func_ids, the server runs them outside inger
        debug!("function with ID {} ran without inger!",func_id.abs());
        let ans = func(func_arg);
        s.get_mut().write(format!("{}",ans).as_ref()).map_err(box_error)?;
        return Ok(())
    }

    let f = inger::launch(|| func(func_arg), func_timeout);

    match f {
        Ok(ans) => {
            if let inger::Linger::Completion(ans) = ans {
                s.get_mut().write(format!("{}",ans).as_ref()).map_err(box_error)?;
                debug!("Yeilded the answer {}",ans);
            }
            else {
                info!("TIMED-OUT");
            }
        },
        Err(error) => error!("ERROR: caused by LIBINGER? {}",error),
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
