use std::borrow::Borrow;


fn fib_local(var: &mut bencher::Bencher) {
    var.iter(|| funs::fib(15));

}


fn fib_remote(var: &mut bencher::Bencher) {
    let conv = String::from;
    let args = vec!(conv("localhost:2000"),conv("0"),conv("15")).into_iter();
    var.iter(|| client::rpc_client(args.borrow().clone()));
}

bencher::benchmark_group![client,fib_local,fib_remote];
bencher::benchmark_main![client];
