use std::io::prelude::*;
use std::net::TcpStream; //I don't think I need this

fn fib(n: u8) -> u64 {

   if n == 0 {
      return 1;
   }
   if n == 1 {
      return 1;
   }

   fib(n - 1) + fib(n - 2)
}


fn handle(mut s : std::net::TcpStream) {

    let mut arg_buf = [0 as u8;1];
    s.read(&mut arg_buf); //Read incoming arguments for fib

    let mut x = 0;  //If no args, return fib(0)
    if let Some(&y) = arg_buf.get(0)
    {
        x = y
    }

    let f = inger::launch(|| {
        let ans = fib(x);
        println!("fib of {} is {}",x,ans);
        s.write(& ans.to_be_bytes()).unwrap();
        loop {}
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
