use std::io::prelude::*;
use std::net::TcpStream; //I don't think I need this





fn handle(mut s : std::net::TcpStream) {

    let mut s = std::io::BufReader::new(s);
    let mut data = String::default();
    println!("The string before {}",data);
    s.read_line(&mut data); //Read all incoming args; seperated by '\n'
    data.pop();
    println!("The string afert {}",data);
    let func_id  = data.parse().unwrap();
    data.clear();
    s.read_line(&mut data); //Read all incoming args; seperated by '\n'
    data.pop();
    let func_arg = data.parse().unwrap();
    let func = match func_id {
        0 => funs::fib,
        1 => funs::add2,
        _ => funs::fib,
    };

    println!("func_id is {} func_arg is {}",func_id,func_arg);
    let f = inger::launch(|| {
        let ans = func(func_arg);
        s.get_mut().write(format!("{}",ans).as_ref()).unwrap();
    }, 4000);

    if let Err (error) = f
    {
         eprintln!("ERROR: in launched function :{}",error)
    }
    else if f.unwrap().is_continuation() {
        eprintln!("WARNING: FUNCTION TIMEDOUT!");
    }

}


fn main() -> std::io::Result<()>{
	
   let listener =  std::net::TcpListener::bind("0.0.0.0:2000")?;
    
   for s in listener.incoming()
   {
       let mut s = s?;
       std::thread::spawn(|| handle(s));
   }
   Ok(())
}
