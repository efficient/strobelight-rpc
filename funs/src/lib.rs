extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct  IngerRequest<'a> {
    pub func_id: &'a str,
    pub func_arg: &'a str,     //In the future, might need to change this to a generic queue
    pub func_timeout: &'a str,
}

//Func ID : 1
pub fn fib(n: u64) -> u64 {

   if n == 0 {
      return 1;
   }
   if n == 1 {
      return 1;
   }

   fib(n - 1) + fib(n - 2)
}

//Func ID : 2
pub fn add2(n: u64) -> u64 {
   n + 2
}

//Func ID : 3
pub fn lambda<T>(x: T) -> T {
   x
}
