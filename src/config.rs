

use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Config {
  Ssh(ConfigSsh)
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
                  }
                  Err(e) => {
                    eprintln!("{:?} {:?}", entry.path(), e);
                  }
                }
              }
              Err(e) => {
                eprintln!("{:?}", e);
              }
            }
          }
          Err(e) => {
            eprintln!("{:?}", e);
          }
        }
      }
    }
    Err(e) => {
      eprintln!("{:?}", e);
    }
  }
  c
}


fn default_port_22() -> u32 { 22 }

fn default_empty_string() -> String { "".into() }

