
mod config;
mod args;

fn main() {
    let a = args::parse_cli_args();
    a.maybe_log(1, || { eprintln!("a = {:?}", a);});
    let c = config::read_all_config(&a.config_folder);
    a.maybe_log(1, || { eprintln!("c = {:?}", c);});

}






