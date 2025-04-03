use super::EH_GIT_DIR;

use std::io::{self, Write,Read};
use std::fs::{self,File,OpenOptions};

pub fn set_head(oid: &Vec<u8>) -> io::Result<()>{
  let path = format!("{}/HEAD",EH_GIT_DIR);
  let mut file = OpenOptions::new()
    .write(true) // Permitir escritura
    .create(true) // Crear el archivo si no existe
    .open(path)?; // Abrir el archivo
  file.write_all(&oid)?;
  Ok(())
}

pub fn get_head() -> io::Result<Vec<u8>> {
  let path = format!("{}/HEAD",EH_GIT_DIR);

  if fs::metadata(&path).is_ok(){
    let mut file = File::open(path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    Ok(content)
  } else {
    Err(io::Error::new(io::ErrorKind::NotFound,"HEAD file not found"))
  }
}

