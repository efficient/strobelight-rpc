

fn main() -> std::io::Result<()> {
   let args: Vec<_> = std::env::args().collect();

   client::rpc_client(args.iter().skip(1).map(String::as_ref))
}
