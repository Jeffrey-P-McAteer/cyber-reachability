
// We're in R&D land, silence these errors
#![allow(unused_imports,dead_code,unused_variables)]


#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum DiscoveryTechnique {
  None,
  ThisMachine,
  ICMP_Ping,
  TCPPortScan,
  UDPPortScan,
}


#[derive(Debug)]
pub struct ScanEntity {
  pub parent: Option<Box<ScanEntity>>,
  pub neighbors: Vec<ScanEntity>,
  pub discovery_technique: DiscoveryTechnique,
  pub hardware_description: String,
  pub report_lines: Vec<String>,
}

impl ScanEntity {
  pub fn this_machine() -> Self {
    Self {
      parent: None,
      neighbors: Vec::with_capacity(16),
      discovery_technique: DiscoveryTechnique::ThisMachine,
      hardware_description: read_this_machine_hw_description(),
      report_lines: Vec::with_capacity(16),
    }
  }

  pub async fn scan(&mut self, args: &crate::args::Args, configs: &[crate::config::Config]) {
    use network_interface::NetworkInterface;
    use network_interface::NetworkInterfaceConfig;
    match network_interface::NetworkInterface::show() {
      Ok(network_ifaces) => {
        for iface in network_ifaces {
          self.scan_iface(&iface, args, configs).await;
        }
      }
      Err(e) => {
        eprintln!("{:?}", e)
      }
    }
    // TODO populate self.neighbors w/ various techniques

  }

  fn report_line(&mut self, msg_fn: impl FnOnce() -> String ) {
    self.report_lines.push(msg_fn());
  }

  async fn scan_iface(&mut self, iface: &network_interface::NetworkInterface, args: &crate::args::Args, configs: &[crate::config::Config]) {
    args.maybe_log(3, || { eprintln!("iface = {:?}", iface);});
    for addr in iface.addr.iter() {
      match addr {
        network_interface::Addr::V4(v4_addr) => {
          if v4_addr.ip.is_loopback() {
            return; // Uninteresting, do not scan self
          }
          if let Ok(net) = ipnet::Ipv4Net::with_netmask(v4_addr.ip, v4_addr.netmask.unwrap_or(std::net::Ipv4Addr::UNSPECIFIED)) { // UNSPECIFIED is 0.0.0.0 or a /32 range
            //args.maybe_log(2, || { eprintln!("v4 net = {:?}", net);});
            self.scan_iface_ipv4_net(iface, &net, args, configs).await;
          }
        }
        network_interface::Addr::V6(v6_addr) => {
          if v6_addr.ip.is_loopback() {
            return; // Uninteresting, do not scan self
          }
          if let Ok(net) = ipnet::Ipv6Net::with_netmask(v6_addr.ip, v6_addr.netmask.unwrap_or(std::net::Ipv6Addr::UNSPECIFIED)) { // UNSPECIFIED is zeroes
            args.maybe_log(2, || { eprintln!("// TODO implement ignored v6 net = {:?} ({}:{})", net, file!(), line!() );});
          }
        }
      }
    }
  }

  async fn scan_iface_ipv4_net(&mut self, iface: &network_interface::NetworkInterface, net: &ipnet::Ipv4Net, args: &crate::args::Args, configs: &[crate::config::Config]) {
      let num_hosts = hosts_from_prefix_v4(net.prefix_len());

      let mut ping_jobs = Vec::with_capacity(num_hosts as usize);
      for host_v4 in net.hosts() {
        ping_jobs.push(tokio::task::spawn(async move {
          let ping_timeout = std::time::Duration::from_secs(4);
          let data = [1,2,3,4,5,6,7,8]; // ping data
          let data_arc = std::sync::Arc::new(&data[..]);
          let options = ping_rs::PingOptions { ttl: 128, dont_fragment: false };
          ping_rs::send_ping_async(&std::net::IpAddr::V4(host_v4), ping_timeout, data_arc, Some(&options) ).await
        }));
      }

      let results = futures::future::join_all(ping_jobs).await;
      let mut online_hosts = Vec::with_capacity(32);
      for (i, result) in results.into_iter().enumerate() {
        if let Ok(Ok(result)) = result {
          online_hosts.push(result.address);
        }
      }

      self.report_line(|| { format!("{:?} with {} hosts, {} are online: {:?}", net, num_hosts, online_hosts.len(), online_hosts) });
  }

  pub fn print_tree(&self, prefix: &str) {
    println!("{} {:?} {}", prefix, self.discovery_technique, self.hardware_description);
    for report_line in self.report_lines.iter() {
      println!("{}  {}", prefix, report_line);
    }
    let child_prefix = format!("{}>", prefix);
    for neighbor in &self.neighbors {
      neighbor.print_tree(&child_prefix);
    }
  }

}

//pub fn read_thi

pub fn read_this_machine_hw_description() -> String {
  multiline_to_one_line(
    big_three_cmd_output(
      "Write-Host (Get-WmiObject Win32_ComputerSystem).Manufacturer (Get-WmiObject Win32_BIOS).Manufacturer",
      "cat /sys/devices/virtual/dmi/id/board_name ; cat /sys/devices/virtual/dmi/id/board_vendor ; cat /sys/devices/virtual/dmi/id/board_version",
      "sysctl hw.model | sed 's/.* //g'",
    )
  )
}

pub fn multiline_to_one_line(lines: String) -> String {
  let mut out = String::new();
  for line in lines.lines() {
    out += line.trim();
    out += " ";
  }
  out.trim().to_string()
}

pub fn big_three_cmd_output<S: AsRef<std::ffi::OsStr>>(win_cmd: S, linux_cmd: S, macos_cmd: S) -> String {
  if cfg!(target_os = "windows") {
      run_shell_cmd_output(win_cmd)
  }
  else if cfg!(target_os = "linux") {
    run_shell_cmd_output(linux_cmd)
  }
  else if cfg!(target_os = "macos") {
    run_shell_cmd_output(macos_cmd)
  }
  else {
    run_shell_cmd_output("Unsupported OS")
  }
}

#[cfg(target_os = "macos")]
pub fn run_shell_cmd_output<S: AsRef<std::ffi::OsStr>>(cmd: S) -> String {
  use std::process::Command;
  let r = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output();
  match r {
    Ok(output) => format!("{}\n{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr)),
    Err(e) => format!("{:?}", e)
  }
}
#[cfg(target_os = "windows")]
pub fn run_shell_cmd_output<S: AsRef<std::ffi::OsStr>>(cmd: S) -> String {
  use std::process::Command;
  let r = Command::new("powershell")
        .arg("-Command")
        .arg(cmd)
        .output();
  match r {
    Ok(output) => format!("{}\n{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr)),
    Err(e) => format!("{:?}", e)
  }
}
#[cfg(target_os = "linux")]
pub fn run_shell_cmd_output<S: AsRef<std::ffi::OsStr>>(cmd: S) -> String {
  use std::process::Command;
  let r = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output();
  match r {
    Ok(output) => format!("{}\n{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr)),
    Err(e) => format!("{:?}", e)
  }
}
#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn run_shell_cmd_output<S: AsRef<std::ffi::OsStr>>(cmd: S) -> String {
  format!("Target OS is unsupported!")
}

fn hosts_from_prefix_v4(prefix_len: u8) -> u32 {
    if prefix_len > 32 {
        panic!("Ought never occur, prefix_len > 32 for ipv4")
    }

    let mut total_addresses = 1u32;
    for _ in 0..(32 - prefix_len) {
        total_addresses *= 2;
    }

    let usable_hosts = match 32 - prefix_len {
        0 => 1,            // /32
        1 => 2,            // /31
        _ => total_addresses - 2, // subtract network + broadcast
    };

    usable_hosts
}

