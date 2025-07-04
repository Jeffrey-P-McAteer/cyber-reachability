

fn main() {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();
    println!("System name:             {:?}", sysinfo::System::name());
    println!("System host name:        {:?}", sysinfo::System::host_name());

    let family = hw_crossplatform::get_system_family().expect("No known system family");
    println!("family:             {:?}", &family);
    let hws = hw_crossplatform::strategy::get_hw_strategy(family);
    println!("os:             {:?}", hws.get_os());
    println!("bios:             {:?}", hws.get_bios());
    println!("mobo:             {:?}", hws.get_motherboard());
    println!("cpu:             {:?}", hws.get_cpu());
    println!("hw:             {:?}", hws.get_hw());

}




