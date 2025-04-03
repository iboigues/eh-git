use super::EH_GIT_DIR;

use std::io;
use std::fs;

pub fn init() -> io::Result<()> {
  fs::create_dir(EH_GIT_DIR)?;
  fs::create_dir(format!("{}/objects",EH_GIT_DIR))?;
  let _ = fs::File::create(format!("{}/HEAD",EH_GIT_DIR))?;
  Ok(())
}
