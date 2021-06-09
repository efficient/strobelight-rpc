fn h(_s : std::net::TcpStream) -> std::io::Result<()>
{
   inger::launch(|| println!("Launched ! ! !"), u64::max_value()).map(|_| ())
}


fn main() -> std::io::Result<()>{
	
   let listener =  std::net::TcpListener::bind("0.0.0.0:2000")?;
    
   for s in listener.incoming()
   {
       h(s?)?;
   }
   Ok(())
}
