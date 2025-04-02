use std::env;
use std::fs;
use std::process;

mod data;
mod base;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("Error");
    return; 
  }

  match args[1].as_str() {
    "init" => {
      if let Err(e) = data::init::init() {
        eprintln!("Error al inicializar el repositiorio : {}",e);
      } else {  
        println!("Repositiorio inicializado");
      }
    }
    "hash-object" => {
      if args.len() < 3 {
        return;
      }

      let content = fs::read_to_string(args[2].as_str()).unwrap();

      match data::objects::hash_object(content.into_bytes(), "blob") {
        Ok(oid) => {
          let hex: String = oid.iter().map(|b| format!("{:02x}", b)).collect();
          println!("{}",hex);
        }
        Err(e) => eprintln!("{}",e),
      }
    }
    "cat-file" => {
      if args.len() < 3 {
        return;
      }
      

      match data::objects::get_object(args[2].as_str(),None) {
        Ok(content) => { 
          let content_str = String::from_utf8_lossy(&content);
          for line in content_str.lines() {
            println!("{}", line); // Imprimir cada línea por separado
          }
        },
        Err(e) => eprintln!("{}",e),
      }
    }
    "write-tree" => {
      match base::tree::write_tree(".") {
        Ok(content) => {
          let hex: String = content.iter().map(|b| format!("{:02x}", b)).collect();
          println!("{}", hex);
        },
        Err(e) => eprintln!("{}",e),
      }      
    }
    "read-tree" => {
      if args.len() < 3 {
        return;
      }

      if let Err(e) = base::tree::read_tree(hex::decode(&args[2]).unwrap()) {
        eprintln!("{}",e);
      }
    }
    "commit" => {
      let mut msg = String::from("");
      
      if let Some(pos) = args.iter().position(|arg| arg == "-m"){
        if pos + 1 < args.len() {
          msg = args[pos+1].clone();
        } else {
          eprintln!("No has proporcionado un comentario");
          process::exit(1);
        }
      }

      match base::commit::commit(&msg) {
        Ok(content) => {
          let hex: String = content.iter().map(|b| format!("{:02x}", b)).collect();
          println!("{}", hex);
        },
        Err(e) => eprintln!("{}",e),
      }      
    }
    _ => {}
  }
}
