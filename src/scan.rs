
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
}

impl ScanEntity {
  pub fn this_machine() -> Self {
    Self {
      parent: None,
      neighbors: Vec::with_capacity(16),
      discovery_technique: DiscoveryTechnique::ThisMachine,
      hardware_description: read_this_machine_hw_description(),
    }
  }

  pub fn scan(&mut self, args: &crate::args::Args, configs: &[crate::config::Config]) {
    use network_interface::NetworkInterface;
    use network_interface::NetworkInterfaceConfig;
    match network_interface::NetworkInterface::show() {
      Ok(network_ifaces) => {
        for iface in network_ifaces {
          self.scan_iface(&iface, args, configs);
        }
      }
      Err(e) => {
        eprintln!("{:?}", e)
      }
    }
    // TODO populate self.neighbors w/ various techniques

  }

  fn scan_iface(&mut self, iface: &network_interface::NetworkInterface, args: &crate::args::Args, configs: &[crate::config::Config]) {
    args.maybe_log(2, || { eprintln!("iface = {:?}", iface);});
    for addr in iface.addr.iter() {
      match addr {
        network_interface::Addr::V4(v4_addr) => {
          if v4_addr.ip.is_loopback() {
            return; // Uninteresting, do not scan self
          }
          let net = ipnet::Ipv4Net::with_netmask(v4_addr.ip, v4_addr.netmask.unwrap_or(std::net::Ipv4Addr::UNSPECIFIED)); // UNSPECIFIED is 0.0.0.0 or a /32 range
          args.maybe_log(2, || { eprintln!("v4 net = {:?}", net);});
        }
        network_interface::Addr::V6(v6_addr) => {
          if v6_addr.ip.is_loopback() {
            return; // Uninteresting, do not scan self
          }
          let net = ipnet::Ipv6Net::with_netmask(v6_addr.ip, v6_addr.netmask.unwrap_or(std::net::Ipv6Addr::UNSPECIFIED)); // UNSPECIFIED is zeroes
          args.maybe_log(2, || { eprintln!("v6 net = {:?}", net);});
        }
      }
    }
  }

  pub fn print_tree(&self, prefix: &str) {
    println!("{} {:?} {}", prefix, self.discovery_technique, self.hardware_description);
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

