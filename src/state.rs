pub mod state_md
{
  use std::fs;
  use device_query::Keycode;


  pub struct Core
  {
    pub data : Vec<String>,
    
  } impl Core {
    
    pub fn len(&self) -> i64 { return self.data.len() as i64; }
    pub fn start(&self) { println!("[START]"); }
  }
  

  // RETURN VECTOR WHICH CONTAIN CURRENT FOLDER'S DATA
  pub fn get(path: String) -> Vec<String>
  {
  	let mut folder: Vec<String> = Vec::new();

  	if let Ok(entries) = fs::read_dir(path) {
      for entry in entries {
      	if let Ok(entry) = entry { folder.push(entry.file_name().into_string().unwrap()); }
      }
  	} else { eprintln!("Failed to read directory contents"); }

  	return folder;
  }


  // GIVE LENGTH OF VECTOR WHICH CONTAIN CURRENT FOLDER'S DATA
  //pub fn get_folder_length() -> i64
  //{
  //  return get(".".to_string()).len() as i64;
  //}

  
  pub fn start() -> Core
  {
    return Core {
      data : get(".".to_string()),
    }
  }
}
