pub mod print_md
{
  use crate::state::state_md::Core;
  use crate::state::state_md;
  
  use std::fs;
  use std::io::{self, Write};


  const colors: [&str; 8] = 
  [
    "\x1b[31m", // RED
    "\x1b[32m", // GREEN
    "\x1b[33m", // YELLOW
    "\x1b[34m", // BLUE
    "\x1b[35m", // MAGENTA
    "\x1b[36m", // CYAN
    "\x1b[37m", // WHITE
    "\x1b[0m", // RESET
  ];


  fn print(x: String) -> String
  {
    return format!("{}> {}{}{}", colors[3], x, colors[3], colors[7]);
  }


  pub fn keycode_q(x: String) -> String
  {
    return format!("{}{}{}{}", colors[0], x, colors[0], colors[7]);
  }


  pub fn start(core: &Core, id: i64)
  {
  	print!("\x1B[2J");    
  	print!("\x1B[1;1H");
  	io::stdout().flush().unwrap();
    
			
  	for (idx, elm) in core.data.clone().into_iter().enumerate()
  	{
      if (idx as i64 == id) { println!("{}", print(elm)); }
      else { println!("  {}", elm); }			
  	}
  } 
}
