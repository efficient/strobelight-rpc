use lazy_static::lazy_static;
use std::env;

lazy_static! {
    static ref ADDRESS: String = env::var("ADDRESS").unwrap();
    static ref FUNC_ID: String = env::var("FUNC_ID").unwrap();
    static ref FUNC_ARG: String = env::var("FUNC_ARG").unwrap();
    static ref FUNC_TIMEOUT: String = env::var("FUNC_TIMEOUT").unwrap();
}


fn fib_local(var: &mut bencher::Bencher) {

    var.iter(|| funs::fib(FUNC_ARG.parse().unwrap_or(1)));
}

fn fib_remote(var: &mut bencher::Bencher) {

    let arg1: &str = ADDRESS.as_str();
    let arg2: &str = FUNC_ID.as_str();
    let arg3: &str = FUNC_ARG.as_str();
    let arg4: &str = FUNC_TIMEOUT.as_str();
    let args = vec![arg1,arg2,arg3,arg4].into_iter();
    var.iter(|| client::rpc_client(args.clone()).unwrap());
}

fn fib_remote_with_inger(var: &mut bencher::Bencher) {

    let arg1: &str = ADDRESS.as_str();
    let arg2: &str = FUNC_ID.as_str();
    let arg3: &str = FUNC_ARG.as_str();
    let arg4: &str = FUNC_TIMEOUT.as_str();
    let args = vec![arg1,arg2,arg3,arg4].into_iter();
    var.iter(|| client::rpc_client(args.clone()).unwrap());
}

bencher::benchmark_group![client,fib_local,fib_remote,fib_remote_with_inger];
bencher::benchmark_main![client];
