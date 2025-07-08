
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

    // Setup a tokio runtime for the scan
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(12)
        .build()
        .expect("Failed to build Tokio runtime");

    // Run the scan async
    rt.block_on(e.scan(&a, &c));

    // TODO report stuff
    //eprintln!("{:?}", e);
    e.print_tree(">");
}






