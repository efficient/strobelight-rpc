//Func ID : 0
pub fn fib(n: u64) -> u64 {

   if n == 0 {
      return 1;
   }
   if n == 1 {
      return 1;
   }

   fib(n - 1) + fib(n - 2)
}


//Func ID : 1
pub fn add2(n: u64) -> u64 {
   n + 2
}
