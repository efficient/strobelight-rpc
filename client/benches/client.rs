
fn fib_local(var: &mut bencher::Bencher){
    var.iter(|| funs::fib(15));

}


bencher::benchmark_group![client,fib_local];
bencher::benchmark_main![client];
