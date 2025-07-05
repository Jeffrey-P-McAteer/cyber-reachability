

use std::path::PathBuf;

use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  //#[serde(default = "empty_string")]
  pub boot_iso_url: String,
  pub boot_iso: PathBuf,
}


