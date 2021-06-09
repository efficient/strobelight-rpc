const ADDRESS: &str = "127.0.0.1:2000";
fn main() -> std::io::Result<()>
{
   let _s = std::net::TcpStream::connect(ADDRESS)?;

   Ok(())
}
