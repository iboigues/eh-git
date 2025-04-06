use super::EH_GIT_DIR;

use std::io::{self, Write,Read};
use std::fs::{self,OpenOptions,File};
use std::path::Path;

pub fn update_ref(ref_name: &str,oid: &Vec<u8>) -> io::Result<()>{
  let path = format!("{}/{}",EH_GIT_DIR,ref_name);
  if let Some(parent) = Path::new(&path).parent() {
    fs::create_dir_all(parent)?;
  }

  let mut file = OpenOptions::new()
    .write(true) // Permitir escritura
    .create(true) // Crear el archivo si no existe
    .open(path)?; // Abrir el archivo
  
  file.write_all(oid)?;
  Ok(())
}

pub fn get_ref(ref_name: &str) -> io::Result<String> {
  let path = format!("{}/{}",EH_GIT_DIR,ref_name);

  if fs::metadata(&path).is_ok(){
    let mut file = File::open(path)?;
    if ref_name.contains("refs") {
      let mut content = String::new();
      file.read_to_string(&mut content)?;

      Ok(content)
    } else {
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    Ok(content.iter().map(|b| format!("{:02x}", b)).collect())
    }
  } else {
    Err(io::Error::new(io::ErrorKind::NotFound,"HEAD file not found"))
  }
}

pub fn get_oid(name: &str) -> String {
  get_ref(name).unwrap_or_else(|_| name.to_string())
}
