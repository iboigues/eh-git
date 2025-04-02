use crate::data::{head::{get_head, set_head}, objects::hash_object};

use super::tree::write_tree;

use std::io;

pub fn commit(message: &str) -> io::Result<Vec<u8>>{
  let mut commit:Vec<u8> = Vec::new();
  
  commit.extend_from_slice("tree".as_bytes());
  commit.push(b'0');
  let parent = get_head()?;
  if !parent.is_empty() {
    commit.extend_from_slice(&parent);
    commit.push(b'0');
  }
  commit.extend_from_slice(&write_tree(".").unwrap());
  commit.push(b'\n');
  commit.push(b'\n');
  commit.extend_from_slice(message.as_bytes());
  
  let oid = hash_object(commit, "commit")?;

  set_head(&oid)?;

  Ok(oid)
}
