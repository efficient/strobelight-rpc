
fn fib_local(var: &mut bencher::Bencher) {
    var.iter(|| funs::fib(15));

}


fn fib_remote(var: &mut bencher::Bencher) {
    let args = vec![("localhost:2000"),("0"),("15")].into_iter();
    var.iter(|| client::rpc_client(args.clone()));
}

bencher::benchmark_group![client,fib_local,fib_remote];
bencher::benchmark_main![client];
