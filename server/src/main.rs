use funs::IngerRequest;
use log::{debug,error,info};
use std::collections::HashMap;
use std::fmt::Display;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::{Arc, Mutex, MutexGuard};
use std::string::String;
type Continuation = inger::Linger<u64, dyn FnMut(*mut Option<std::thread::Result<u64>>) + Send>;

fn handle(s: TcpStream, mut stored_ingers: MutexGuard<HashMap<String,Continuation>>)
          -> Result<(),Box<dyn Display>> {

    let mut s = std::io::BufReader::new(s);
    let mut data = String::default();
    s.read_line(&mut data).map_err(box_error)?;
    let request: IngerRequest = serde_json::from_str(&data).map_err(box_error)?;

    debug!("done parsing struct");
    debug!("{} {} {}",request.func_id,request.func_arg,request.func_timeout);

    let func_id: i64 = request.func_id.parse().map_err(box_error)?;
    let func_arg: u64 = request.func_arg.parse().map_err(box_error)?;
    let func_timeout: u64 = request.func_timeout.parse().map_err(box_error)?;
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

    //
    let f = match stored_ingers.get_mut(&data) {
        Some(inger) => {
            inger::resume(inger, func_timeout);
            inger
        },
        None => {
            let x = inger::launch(move || func(func_arg), func_timeout).map_err(box_error)?;
            stored_ingers.insert(data.clone(),x.erase());
            stored_ingers.get(&data).unwrap()
        },
    };

    if let inger::Linger::Completion(ans) = f {
        s.get_mut().write(format!("{}",ans).as_ref()).map_err(box_error)?;
        debug!("Yeilded the answer {}",ans);
    }
    else {
        info!("TIMED-OUT"); //This is where the mutex needs to be used
    }
    Ok(())
}



fn box_error<'a,T: Display + 'a>(e: T) -> Box<dyn Display + 'a> {

    Box::new(e)
}

fn handle_wrapper(s: TcpStream, stored_ingers: MutexGuard<HashMap<String,Continuation>>) {

    if let Err(e) = handle(s, stored_ingers) {
        eprintln!("ERROR: {}",e);
    }
}

fn main() -> std::io::Result<()> {

    let stored_ingers = Arc::new(Mutex::new(HashMap::new()));
    env_logger::init();
    let listener =  std::net::TcpListener::bind("0.0.0.0:2000")?;
    for s in listener.incoming() {
        let s = s?;
        let shared_cpy = Arc::clone(&stored_ingers);
        std::thread::spawn(move || {
            let locked_cpy = shared_cpy.lock().unwrap();
            handle_wrapper(s,locked_cpy);
        }).join().unwrap();
    }
    Ok(())
}
