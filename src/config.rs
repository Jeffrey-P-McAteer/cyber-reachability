

use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  //#[serde(default = "empty_string")]
  pub boot_iso_url: String,
  pub boot_iso: std::path::PathBuf,
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
