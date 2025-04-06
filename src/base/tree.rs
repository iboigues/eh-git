use crate::data::objects::get_object;
use crate::data::objects::hash_object;
use crate::data::EH_GIT_DIR;

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path,PathBuf};
use std::collections::BTreeMap;

fn is_ignored(path: &str) -> bool {
  //TODO .eh-gitignore
  return path.contains(EH_GIT_DIR) || path.contains("target") || path.contains("eh-git") || path.contains(".git") || path.contains("src");
}

fn empty_current_directory() -> io::Result<()> {
  let current_dir = Path::new(".");

  for entry in fs::read_dir(current_dir)? {
    let entry = entry?;
    let path = entry.path();
    let path_str = path.to_str().unwrap_or("");

    if path.is_file() {
      if is_ignored(&path_str) {
        continue;
      }

      fs::remove_file(&path)?;
    } else if path.is_dir() {

      if is_ignored(&path_str) {
        continue;
      }
      match fs::remove_dir(&path) {
        Ok(_) => (),
        Err(_) => {
          // Si el directorio no está vacío (quizás por archivos ignorados), lo ignoramos
        }
      }
    }
  }
  Ok(())
}

pub fn write_tree(directory: &str) -> io::Result<Vec<u8>> {
  let mut entries : BTreeMap<String,(Vec<u8>, String)> = BTreeMap::new();
  let dir_entries = fs::read_dir(directory)?;

  for entry in dir_entries {
    let entry = entry?;
    let full_path = entry.path();
    let name = entry.file_name().into_string().unwrap();

    if is_ignored(full_path.to_str().unwrap()) {
      continue;
    }

    if full_path.is_file() {
      let mut file = File::open(&full_path)?;
      let mut contents = Vec::new();
      file.read_to_end(&mut contents)?;
      let oid = hash_object(contents,"blob")?; 

      entries.insert(name,(oid,"blob".to_string()));
    } else if full_path.is_dir() {
      let oid = write_tree(full_path.to_str().unwrap())?; 
      entries.insert(name,(oid,"tree".to_string()));
    }
  }

  let mut tree = Vec::new();
  for (name,(oid,type_)) in &entries{
    tree.extend_from_slice(type_.as_bytes());
    tree.push(b'0');
    tree.extend_from_slice(oid);
    tree.push(b'0');
    tree.extend_from_slice(name.as_bytes());
    tree.push(b'\n');
  }

  hash_object(tree, "tree")
}

fn iter_tree_entries(oid: Vec<u8>) -> io::Result<Vec<(String, Vec<u8>, String)>> {
  if oid.is_empty() {
    return Ok(vec![])
  }

  let oid_hex = oid.iter().map(|b| format!("{:02x}",b)).collect::<String>();
  
  let (_,tree) = get_object(oid_hex.as_str(), Some("tree"))?;
  let mut entries = Vec::new();

  for entry in tree.split(|&b| b == b'\n') {
    let parts: Vec<&[u8]> = entry.splitn(3,|&b| b == b'0').collect();

    if parts.len() == 3 {
      let type_ = String::from_utf8_lossy(parts[0]).into_owned();
      let name = String::from_utf8_lossy(parts[2]).into_owned();
      entries.push((type_, parts[1].to_vec(), name));
    }
  }
  Ok(entries)
}

fn get_tree(tree_oid: Vec<u8>,base_path: &str) -> io::Result<BTreeMap<PathBuf,Vec<u8>>> {
  let mut result = BTreeMap::new();

  for(type_,oid,name) in iter_tree_entries(tree_oid)? {
    assert!(!name.contains('/'));
    assert!(name != ".." && name != ".");

    let path = PathBuf::from(base_path).join(name);

    if type_ == "blob" {
      result.insert(path,oid);
    } else if type_ == "tree" {
      result.extend(get_tree(oid, path.to_str().unwrap())?);
    } else {
      panic!("Unknonw tree entry");
    }
  }

  Ok(result)
}

pub fn read_tree(tree_oid: Vec<u8>) -> io::Result<()> {
  if let Err(e) = empty_current_directory(){
    eprintln!("Error borrando directorio {}",e);
  }

  for (path,oid) in get_tree(tree_oid,"./")? {
    if let Some(parent) = path.parent() {
      fs::create_dir_all(parent)?;
    }
    
    let oid_hex = oid.iter().map(|b| format!("{:02x}",b)).collect::<String>();

    let mut file = File::create(&path)?;
    let (_,content) = &get_object(oid_hex.as_str(),Some("blob"))?;
    file.write_all(&content)?;
  }

  Ok(())
}
