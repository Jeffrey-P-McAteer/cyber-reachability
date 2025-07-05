
mod args;
mod config;
mod scan;
mod report;

fn main() {
    let a = args::parse_cli_args();
    a.maybe_log(1, || { eprintln!("a = {:?}", a);});
    let c = config::read_all_config(&a.config_folder);
    a.maybe_log(1, || { eprintln!("c = {:?}", c);});
    let mut e = scan::ScanEntity::this_machine();
    e.scan(&c);

    // TODO report stuff
    eprintln!("{:?}", e);
}






