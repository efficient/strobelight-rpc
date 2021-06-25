

fn main() -> std::io::Result<()> {
   let args: Vec<_> = std::env::args().collect();
   let ans_buf = client::rpc_client(args.iter().skip(1).map(String::as_ref))?;
   println!("{}",ans_buf);
   Ok(())
}
