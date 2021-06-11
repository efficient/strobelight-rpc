use std::io::prelude::*;
use std::net::TcpStream; //I don't think I need this



//Func ID : 0
fn fib(n: u64) -> u64 {

   if n == 0 {
      return 1;
   }
   if n == 1 {
      return 1;
   }

   fib(n - 1) + fib(n - 2)
}


//Func ID : 1
fn add2(n: u64) -> u64 {
   n + 2
}



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
        0 => fib,
        1 => add2,
        _ => fib,
    };

    println!("func_id is {} func_arg is {}",func_id,func_arg);
    let f = inger::launch(|| {
        let ans = func(func_arg);
        println!("ans of input {} is {}",func_arg,ans);
        s.get_mut().write(format!("{}",ans).as_ref()).unwrap();
    }, 4000);

    if let Err (error) = f
    {
         eprintln!("ERROR: in launched function :{}",error)
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
