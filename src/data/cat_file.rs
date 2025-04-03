pub fn cat_blob(content: Vec<u8>){
  let content_str = String::from_utf8_lossy(&content);
  for line in content_str.lines() {
    println!("{}", line); // Imprimir cada línea por separado
  }
}

pub fn cat_tree(content: Vec<u8>){
  for entry in content.split(|&b| b == b'\n') {
    let parts: Vec<&[u8]> = entry.splitn(3,|&b| b == b'0').collect();

    if parts.len() == 3 {
      let type_ = String::from_utf8_lossy(parts[0]).into_owned();
      let hex: String = parts[1].iter().map(|b| format!("{:02x}", b)).collect();
      let name = String::from_utf8_lossy(parts[2]).into_owned();
      println!("{} {} {}",type_,hex,name);
    }
  }
}

pub fn cat_commit(content: Vec<u8>){
  let parts = content.split(|&b| b == b'\n');

  for line in parts {
    if line.starts_with("tree".as_bytes()) || line.starts_with("parent".as_bytes()){
      let parts: Vec<&[u8]> = line.splitn(2,|&b| b == b'0').collect();

      let type_ = String::from_utf8_lossy(parts[0]).into_owned();
      let hex: String = parts[1].iter().map(|b| format!("{:02x}", b)).collect();
    
      println!("{} {}",type_,hex);
    } else {
      print!("{}",String::from_utf8_lossy(line).into_owned());
    }
  }
}
