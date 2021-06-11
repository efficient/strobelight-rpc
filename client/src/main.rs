
const ADDRESS: &str = "127.0.0.1:2000";

fn main() -> std::io::Result<()> {

    let input_address: Option<String> = std::env::args().nth(1); //Fix: .next_back() Always returned
    let mut address: String = String::from(ADDRESS);             //A value, since if no args given,
    if let Some(address_a) = input_address {                     //It returns Some(/target/../client)
       address = address_a;
    }

    let _s = std::net::TcpStream::connect(&address)?;



    Ok(())
}
