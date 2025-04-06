use crate::data;

use std::io;

pub fn create_tag(name: &str, oid:&str) -> io::Result<()>{
  data::ref_manager::update_ref(&format!("refs/tags/{}",name), &oid.as_bytes().to_vec())?;
  Ok(())
}
