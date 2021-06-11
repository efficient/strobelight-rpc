

fn handle(_s : std::net::TcpStream) {

    let f = inger::launch(|| {
        println!("Launched ! ! !");
        while true {}
    }, u64::max_value());

    if let Err (error) = f
    {
         eprintln!("ERROR: in launched function :{}",error)
    }

}


fn main() -> std::io::Result<()>{
	
   let listener =  std::net::TcpListener::bind("0.0.0.0:2000")?;
    
   for s in listener.incoming()
   {
       let s = s?;
       std::thread::spawn(|| handle(s));
   }
   Ok(())
}
