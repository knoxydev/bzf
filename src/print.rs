pub mod print_md
{
  use crate::state::state_md::Core;
  use crate::state::state_md;
  
  use std::fs;
  use std::io::{self, Write};


  pub fn start(id: i64)
  {
    let core = state_md::start();


  	print!("\x1B[2J");    
  	print!("\x1B[1;1H");
  	io::stdout().flush().unwrap();

			
  	for (idx, elm) in core.data.into_iter().enumerate()
  	{
      if (idx as i64 == id) { println!("> {}", elm); }
      else { println!("  {}", elm); }			
  	}
  } 
}
