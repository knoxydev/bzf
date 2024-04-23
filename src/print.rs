pub mod print_md
{
  use crate::state::state_md::{Core, Type};
  use crate::state::state_md;
  
  use std::fs;
  use std::io::{self, Write};
  use std::path::PathBuf;


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


  fn print_path(path: &PathBuf)
  {
    println!("\n{}\n", path.display());
  }


  pub fn keycode_q(x: String) -> String
  {
    return format!("{}{}{}{}", colors[0], x, colors[0], colors[7]);
  }


  fn print(curr: bool, elm: String, typeis: Type)
  {
    if curr == true
    {
      match typeis {
        Type::Directory => println!("{}> • {}{}{}", colors[3], elm, colors[3], colors[7]),
        Type::File => println!("{}>   {}{}{}", colors[3], elm, colors[3], colors[7]),
      }
    }
    else
    {
      match typeis {
        Type::Directory => println!("  • {}", elm),
        Type::File => println!("    {}", elm),
      }
    }
  }


  pub fn start(core: &Core, id: i64)
  {
  	print!("\x1B[2J");    
  	print!("\x1B[1;1H");
  	io::stdout().flush().unwrap();


    print_path(&core.curr_path);
    
			
  	for (idx, elm) in core.data.clone().into_iter().enumerate()
  	{
      if (idx as i64 == id) { print(true, elm.obj, elm.typeis); }
      else { print(false, elm.obj, elm.typeis); }
  	}
  } 
}
