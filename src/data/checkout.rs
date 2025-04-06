use crate::base::{commit::get_commit, tree::read_tree};

use super::ref_manager::update_ref;

use std::io;

pub fn checkout(oid: &str) -> io::Result<()>{
  let (tree_oid,_,_) = get_commit(oid);
  let tree_oid_copy = tree_oid.clone();
  read_tree(tree_oid)?;
  update_ref("HEAD",&tree_oid_copy)?;
  Ok(())
}
