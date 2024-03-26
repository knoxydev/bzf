pub mod state_md
{
  use std::fs;
  use std::env;
  use std::path::{Path, PathBuf};
  use device_query::Keycode;
  

  pub struct Core
  {
    pub data : Vec<String>,
    pub curr_path : PathBuf,
    pub prev_path : PathBuf,
  }


  pub fn path() -> PathBuf
  {
    let mut x = PathBuf::new();
    if let Ok(curr_dir) = env::current_dir() { x = curr_dir; }
    return x;
  }


  // GIVE LENGTH OF VECTOR WHICH CONTAIN CURRENT FOLDER'S DATA
  pub fn len(x: &Vec<String>) -> i64 { return x.len() as i64; }


  // RETURN VECTOR WHICH CONTAIN CURRENT FOLDER'S DATA
  pub fn get(path: PathBuf) -> Vec<String>
  {
    let mut folder: Vec<String> = Vec::new();

    //if let Ok(entries) = fs::read_dir(path) {
    //  for entry in entries {
    //    if let Ok(entry) = entry { folder.push(entry.file_name().into_string().unwrap()); }
    //  }
    //} else { eprintln!("Failed to read directory contents"); }


    for entry in path.read_dir().expect("read_dir call failed") {
      if let Ok(entry) = entry { folder.push(entry.file_name().into_string().unwrap()); }
    }

    return folder;
  }

  
  pub fn init() -> Core
  {
    Core
    {
      curr_path : path(),
      prev_path : PathBuf::new(),
      data : get(path()),
    }
  }
  
    
  pub fn start() -> Core
  {
    return init();
  }
}
