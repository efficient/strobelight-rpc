use lazy_static::lazy_static;
use std::env;

lazy_static! {
    static ref ADDRESS: String = env::var("ADDRESS").unwrap_or(String::from("localhost:2000"));
    static ref FUNC_ID: String = env::var("FUNC_ID").unwrap_or(String::from("1"));
    static ref FUNC_ARG: String = env::var("FUNC_ARG").unwrap_or(String::from("20"));
    static ref FUNC_TIMEOUT: String = env::var("FUNC_TIMEOUT").unwrap_or(String::from("500"));
}

fn get_env_args(is_inger: bool) -> impl Iterator<Item = &'static str> + Clone {

    let arg1: &str = &ADDRESS;
    let mut arg2: &str = &FUNC_ID;
    if !is_inger {
        arg2 = "-1";
    }
    let arg3 = &FUNC_ARG;
    let arg4 = &FUNC_TIMEOUT;
    vec![arg1,arg2,arg3,arg4].into_iter()
}

fn fib_local(var: &mut bencher::Bencher) {

    var.iter(|| funs::fib(FUNC_ARG.parse().unwrap_or(1)))
}

fn fib_remote(var: &mut bencher::Bencher) {

    let args = get_env_args(false);
    var.iter(|| drop(client::rpc_client(args.clone())))
}

fn fib_remote_with_inger(var: &mut bencher::Bencher) {

    let args = get_env_args(true);
    var.iter(|| drop(client::rpc_client(args.clone())))
}

bencher::benchmark_group![client,fib_local,fib_remote,fib_remote_with_inger];
bencher::benchmark_main![client];
