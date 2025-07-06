

use serde::{Serialize, Deserialize};


#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Config {
  Local_Tools(ConfigLocalTools),
  Ssh(ConfigSsh)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ConfigLocalTools {
  pub linux_x86_64_bin: std::path::PathBuf,
  pub windows_x86_64_bin: std::path::PathBuf,
  pub macos_x86_64_bin: std::path::PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ConfigSsh {
  pub hostname: String,

  #[serde(default = "default_port_22")]
  pub port: u32,

  pub username: String,

  #[serde(default = "default_empty_string")]
  pub password: String,

  #[serde(default = "default_empty_string")]
  pub key_file: String,
}


pub fn read_all_config(dir: &std::path::Path) -> Vec<Config> {
  let mut c = Vec::with_capacity(32);
  let mut c_source_files = Vec::with_capacity(32);
  let mut error_occurred = false;
  match std::fs::read_dir(dir) {
    Ok(read_dir) => {
      for entry in read_dir {
        match entry {
          Ok(entry) => {
            match std::fs::read_to_string(entry.path()) {
              Ok(entry_content) => {
                match toml::from_str::<Config>(&entry_content) {
                  Ok(parsed_config) => {
                    c.push(parsed_config);
                    c_source_files.push(entry.path().clone());
                  }
                  Err(e) => {
                    eprintln!("{:?} {:?}", entry.path(), e);
                    error_occurred = true;
                  }
                }
              }
              Err(e) => {
                eprintln!("{:?}", e);
                error_occurred = true;
              }
            }
          }
          Err(e) => {
            eprintln!("{:?}", e);
            error_occurred = true;
          }
        }
      }
    }
    Err(e) => {
      eprintln!("{:?}", e);
      error_occurred = true;
    }
  }
  for (config_struct, config_source_file) in c.iter().zip(c_source_files.iter()) {
    if let Err(e) = config_struct.check_config() {
      eprintln!("From {:?} {}", config_source_file, e);
      error_occurred = true;
    }
  }
  if error_occurred {
    eprintln!("Pausing because an error occurred, press enter to continue with partial configuration...");
    let mut line = String::new();
    if let Err(e) = std::io::stdin().read_line(&mut line) {
      eprintln!("{:?}", e);
    }
  }
  c
}


fn default_port_22() -> u32 { 22 }

fn default_empty_string() -> String { "".into() }


impl Config {
  pub fn check_config(&self) -> Result<(), Box<dyn std::error::Error>> {
    match self {
      Config::Local_Tools(tools_config) => {
        for bin in &[&tools_config.linux_x86_64_bin, &tools_config.windows_x86_64_bin, &tools_config.macos_x86_64_bin] {
          if !bin.exists() {
            return Err(format!("Error: {:?} does not exist!", tools_config.linux_x86_64_bin).into());
          }
        }
      }
      Config::Ssh(ssh_config) => {
        if ssh_config.key_file.len() > 0 && !std::path::PathBuf::from(ssh_config.key_file.clone()).exists() {
          return Err(format!("Error: {:?} does not exist!", ssh_config.key_file).into());
        }
        if ssh_config.key_file.len() <= 0 && ssh_config.password.len() <= 0 {
          return Err(format!("Error: [ssh] block MUST specify either a key_file or a password, cannot leave both blank").into());
        }
      }
    }
    Ok(())
  }
}
