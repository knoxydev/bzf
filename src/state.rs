pub mod state_md
{
  use std::fs;
  use std::env;
  use std::path::{Path, PathBuf};
  use device_query::Keycode;


  #[derive(Clone)]
  pub enum Type { Directory, File, }


  #[derive(Clone)]
  pub struct Info
  {
    pub obj : String,
    pub typeis : Type,
    pub size : i64,
  }


  pub struct Core
  {
    pub data : Vec<Info>,
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
  pub fn len(x: &Vec<Info>) -> i64 { return x.len() as i64; }
 

  // RETURN VECTOR WHICH CONTAIN CURRENT FOLDER'S DATA
  pub fn get(path: PathBuf) -> Vec<Info>
  {
    let mut folder: Vec<Info> = Vec::new();

    match fs::read_dir(path)
    {
      Ok(x) => {
        for entry in x {        
          if let Ok(elm) = entry {
            let file_type = elm.file_type();

            if let Ok(ft) = file_type
            {
              if ft.is_dir() { folder.push(
                Info {
                  obj: elm.file_name().into_string().unwrap(),
                  typeis: Type::Directory, size: 0,
                });
              }
              else { folder.push(
                Info {
                  obj: elm.file_name().into_string().unwrap(),
                  typeis: Type::File, size: 0,
                });
              }
            }
            else
            { println!("[ERROR]: Failed to get file type for {}", elm.path().display()); }
          }
          else
          { println!("[ERROR]: Failed to get file type for {}", entry.expect("").path().display()); }
        }
      },
      Err(e) => println!("[ERROR]: Failed to read directory - {:?}", e),
    }

    return folder;
  }

  
  pub fn check_type_path(next: &PathBuf) -> bool
  {
    // TRUE = DIRECTORY, FALSE = FILE
    if let Ok(metadata) = fs::metadata(next) { return metadata.is_dir(); }
    else { return false; }
  }


  pub fn prev_path_id(prev: &PathBuf, data: &Vec<Info>) -> i64
  {
    let mut path_x: String = String::new();
    
    if let Some(file_name) = prev.file_name() {
      if let Some(file_name_str) = file_name.to_str()
        { path_x = file_name_str.to_string(); }
      else { println!("[ERROR]: Failed to convert to string"); }
    }
    else { println!("[ERROR]: PathBuf is empty"); }
  
    println!("{:?}\n", path_x);

    for (idx, elm) in data.iter().enumerate()
      { if path_x == data[idx].obj { return idx as i64; } }
    
    return 0 as i64;
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
}
