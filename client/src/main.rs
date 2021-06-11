

fn main() -> std::io::Result<()> {
   client::rpc_client(std::env::args().skip(1))
}
