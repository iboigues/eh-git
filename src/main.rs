use std::env;
use std::fs;
use std::process;

use data::cat_file::cat_blob;
use data::cat_file::cat_commit;
use data::cat_file::cat_tree;
use data::ref_manager::get_oid;
use data::ref_manager::get_ref;

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
      
      let oid = get_oid(args[2].as_str());
    
      match data::objects::get_object(&oid,None) {
        Ok((obj_type,content)) => { 
          match obj_type.as_str() {
            "blob" => cat_blob(content), 
            "tree" => cat_tree(content),
            "commit" => cat_commit(content),
            _ => {}
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

      let oid = get_oid(args[2].as_str());

      if let Err(e) = base::tree::read_tree(oid.into_bytes()) {
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
    "log" => {
      let head:String = if args.len() == 3{
        get_oid(args[2].as_str())
      } else {
        get_ref("HEAD").unwrap()
      };

      match data::objects::get_object(&head,Some("commit")) {
        Ok((obj_type,content)) => { 
          match obj_type.as_str() {
            "commit" => cat_commit(content),
            _ => {}
          }
        },
        Err(e) => eprintln!("{}",e),
      }
    }
    "checkout" => {
      if args.len() < 3 {
        return;
      }

      if let Err(_) = data::checkout::checkout(&get_oid(args[2].as_str())){

      }
    }
    "tag" => {
      if args.len() < 3 {
        return;
      }

      let name: String = args[2].clone();

      let oid:String = if args.len() == 4{
        get_oid(args[3].as_str())
      } else {
        get_ref("HEAD").unwrap()
      };

      if let Err(_) = base::tag::create_tag(&name, &oid){

      }
    }
    _ => {}
  }
}
