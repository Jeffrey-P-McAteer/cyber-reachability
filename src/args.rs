


use clap::Parser;

/// Reads a folder of configuration and then explores a network using
/// configured access credentials and infrastructure hints.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub config_folder: std::path::PathBuf,

    /// Outputs report both to the terminal and to this file
    #[arg(short, long, default_value = None)]
    pub report_file: Option<std::path::PathBuf>,

    /// Instead of scanning the network, fill the config file with example configuration files and exit
    #[arg(short, long, default_value_t = false)]
    pub template_config: bool,

    /// Verbosity
    #[arg(default_value_t = 0, short = 'v', action = clap::ArgAction::Count)]
    pub verbosity: u8,

}

pub fn parse_cli_args() -> Args {
  Args::parse()
}

impl Args {
  pub fn maybe_log(&self, level: u8, msg_fn: impl FnOnce() -> () ) {
    if self.verbosity >= level {
      msg_fn();
    }
  }
}

