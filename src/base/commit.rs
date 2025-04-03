use crate::data::{self, head::{get_head, set_head}};

use super::tree::write_tree;

use std::io;

pub fn commit(message: &str) -> io::Result<Vec<u8>>{
  let mut commit:Vec<u8> = Vec::new();
  
  commit.extend_from_slice("tree".as_bytes());
  commit.push(b'0');
  commit.extend_from_slice(&write_tree(".").unwrap());
  commit.push(b'\n');
  let parent = get_head()?;
  if !parent.is_empty() {
    commit.extend_from_slice("parent".as_bytes());
    commit.push(b'0');
    commit.extend_from_slice(&parent);
  }
  commit.push(b'\n');
  commit.extend_from_slice(message.as_bytes());
  
  let oid = data::objects::hash_object(commit, "commit")?;

  set_head(&oid)?;

  Ok(oid)
}

pub fn get_commit(oid: &str) -> (Vec<u8>,Vec<u8>,String) {
  match data::objects::get_object(oid,Some("commit")) {
    Ok((_,content)) => { 
      let parts = content.split(|&b| b == b'\n');
      let mut tree_oid: Vec<u8> = Vec::new();
      let mut parent_oid: Vec<u8> = Vec::new();
      let mut message = String::new();

      for line in parts {
        if line.starts_with(b"tree") || line.starts_with(b"parent"){
          let parts: Vec<&[u8]> = line.splitn(2,|&b| b == b'0').collect();
          
          if line.starts_with(b"tree") {
            tree_oid = parts[1].to_vec();
          } else {
            parent_oid = parts[1].to_vec();
          }
        } else {
          message.push_str(&String::from_utf8_lossy(line))
        }
      }
      
      (tree_oid,parent_oid,message)
    },
    Err(e) => { 
      eprintln!("{}",e); 
      (Vec::new(),Vec::new(),String::new())
    }
  }
}
