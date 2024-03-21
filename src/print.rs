pub mod print_md
{
	use std::fs;
	use std::io::{self, Write};
	
	
	fn get() -> Vec<String>
	{
		let mut folder: Vec<String> = Vec::new();
	
		if let Ok(entries) = fs::read_dir(".") {
		  for entry in entries {
	      if let Ok(entry) = entry { folder.push(entry.file_name().into_string().unwrap()); }
		  }
	  } else { eprintln!("Failed to read directory contents"); }

	  return folder;
	}


	// GIVE LENGTH OF VECTOR WHICH CONTAIN CURRENT FOLDER'S DATA
	pub fn get_folder_length() -> u64
	{
		let length: u64 = (get().len() as u64);
		return length;
	}


	pub fn start(id: i32)
	{
		let folder: Vec<String> = get();
	
		//println!("{:?}", id);

		print!("\x1B[2J");    
    print!("\x1B[1;1H");
    io::stdout().flush().unwrap();

		    
		for (idx, elm) in folder.into_iter().enumerate()
		{
			if (idx as i32 == id) { println!("> {}", elm); }
			else { println!("  {}", elm); }			
		}
	} 
}
