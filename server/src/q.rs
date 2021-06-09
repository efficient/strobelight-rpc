fn add(x : u64) -> u64
{
    if x == 0 {return 1;}
    x + add(x-1)
}

fn main() {
	
   inger::launch(|| add(13123123), 300).unwrap();

   println!("Hello word!");
}
