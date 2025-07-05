
mod config;
mod args;

fn main() {
    let a = args::parse_cli_args();
    a.maybe_log(1, || { eprintln!("a = {:?}", a);})

    //let c = config::();
}






