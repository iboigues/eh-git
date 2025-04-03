use super::EH_GIT_DIR;

use std::io::{self, Read, Write};
use std::fs::{self,File};
use std::path::Path;
use sha1::{Sha1, Digest};

fn calculate_sha(data: &[u8]) -> Vec<u8> {
  let mut hasher = Sha1::new();
  hasher.update(data);
  return hasher.finalize().to_vec();
}

pub fn hash_object(data: Vec<u8>, obj_type: &str) -> io::Result<Vec<u8>> {
  let mut obj = obj_type.as_bytes().to_vec();
  obj.push(0);
  obj.extend(data);

  let oid = calculate_sha(&obj);

  let mut oid_path = format!("{}/objects/",EH_GIT_DIR);
  let oid_hex = oid.iter().map(|b| format!("{:02x}",b)).collect::<String>();
  oid_path.push_str(&oid_hex);

  let path = Path::new(&oid_path);

  if let Some(parent) = path.parent() {
    fs::create_dir_all(parent)?;
  }

  let mut oid_file = File::create(oid_path)?;
  oid_file.write_all(&obj)?;

  Ok(oid)
}

pub fn get_object(oid: &str,expected: Option<&str>) -> io::Result<(String,Vec<u8>)> {
  let path = format!("{}/objects/{}",EH_GIT_DIR,oid);
  let mut file = File::open(path)?;
  let mut obj = Vec::new();
  file.read_to_end(&mut obj)?;

  if let Some(null_pos) = obj.iter().position(|&b| b == 0) {
    let (type_bytes, content) = obj.split_at(null_pos);
    let type_str = String::from_utf8_lossy(type_bytes).to_string();

    if let Some(expected_type) = expected {
      assert_eq!(type_str, expected_type, "Expected {}, got {}", expected_type, type_str);
    }

    return Ok((type_str,content[1..].to_vec())); 
  }
  
  Err(io::Error::new(io::ErrorKind::InvalidData, "No null byte found"))
}
