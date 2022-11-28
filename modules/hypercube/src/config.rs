use std::{
  fs,
  io::{Error, ErrorKind, Result as IoResult},
  path::PathBuf,
};

use dirs;

const HYPERCUBE_CONFIG_DIR: &str = ".hypercube";

/// Returns the path to the `.hypercube` configuration directory.
/// Creates the directory if it does not exist.
pub fn config_dir() -> IoResult<PathBuf> {
  let home_dir =
    dirs::home_dir().ok_or_else(|| Error::new(ErrorKind::NotFound, "The `home` directory was not found. Please set `$HOME`."))?;

  let cfg_dir = home_dir.join(HYPERCUBE_CONFIG_DIR);

  if !cfg_dir.exists() {
    fs::create_dir_all(&cfg_dir)?;
  }

  Ok(cfg_dir)
}
