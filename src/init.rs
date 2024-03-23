pub mod init_md
{
  use std::fs;
  use std::env;


  pub fn start()
  {
    let mut f_exist: bool = if fs::metadata("bzf/init.txt").is_ok() { true }
    else { false };

    println!("{:?}", f_exist);

    // match fs::create_dir("mox")
    // {
    	// Ok(_) =>
    	// {
    		// 
    	// },
    	// Err(e) =>
    	// {
    		// 
    	// },
    // }


  }
}
