
fn fib_local(var: &mut bencher::Bencher) {
    var.iter(|| funs::fib(15));

}

fn fib_remote(var: &mut bencher::Bencher) {
    let args = vec!["localhost:2000","-1","15","5000"].into_iter();
    var.iter(|| client::rpc_client(args.clone()).unwrap());
}

fn fib_remote_with_inger(var: &mut bencher::Bencher) {
    let args = vec!["localhost:2000","1","15","5000"].into_iter();
    var.iter(|| client::rpc_client(args.clone()).unwrap());
}

bencher::benchmark_group![client,fib_local,fib_remote,fib_remote_with_inger];
bencher::benchmark_main![client];
