pub mod print_md
{
  use crate::state::state_md::{Core, Type, Info, Move};
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


  fn print(curr: bool, elm: String, typeis: Type, id: i64)
  {
    if curr == true
    {
      match typeis {
        Type::Directory => println!("{}{}> • {}{}{}", colors[3], id + 1, elm, colors[3], colors[7]),
        Type::File => println!("{}{}>   {}{}{}", colors[3], id + 1, elm, colors[3], colors[7]),
      }
    }
    else
    {
      match typeis {
        Type::Directory => println!("   • {}", elm),
        Type::File => println!("     {}", elm),
      }
    }
  }


  fn win_size() -> (usize, usize)
  {
    extern crate term_size;

    if let Some((w, h)) = term_size::dimensions() { return (w, h); }
    else { return (0, 0); };
  }


  fn render_one(id: i64, vect: Vec<Info>, last_move: Move)
  {
    let mut left_vec: Vec<Info> = Vec::new();
    let mut right_vec: Vec<Info> = Vec::new();
    let mut main_vec: Vec<Info> = vect.clone();

    let size: (usize, usize) = win_size();
    let h: i64 = size.1 as i64;

    println!("{:?}/{:?}", id + 1, vect.len());

    for (idx, elm) in vect.clone().into_iter().enumerate()
    {
      if (idx + 8) == h.try_into().unwrap() { right_vec.push(elm); }
      else { main_vec.push(elm); }
    }

    /* print!("{:#?}", main_vec); */
    print!("- {:#?}\n", right_vec);

    for (idx, elm) in vect.clone().into_iter().enumerate()
    {
      if (vect.len() as i64) < h
      {
        if (idx as i64 == id) { print(true, elm.obj, elm.typeis, id); }
        else { print(false, elm.obj, elm.typeis, id); }
      }
      else
      {
        if (idx + 10) == h.try_into().unwrap() { println!("..."); break; }
        else
        {
          if (idx as i64 == id) { print(true, elm.obj, elm.typeis, id); }
          else { print(false, elm.obj, elm.typeis, id); }
        }
      }
    }
  }


  fn render(id: i64, vect: Vec<Info>, last_move: Move)
  {
    let mut main_vec: Vec<Info> = vect.clone();
    let mut left_vec: Vec<Info> = Vec::new();
    let mut right_vec: Vec<Info> = Vec::new();

    let size: (usize, usize) = win_size();
    let h: i64 = size.1 as i64;

    println!("{:?}/{:?}", id + 1, vect.len());

    if (main_vec.len()) > h.try_into().unwrap()
    {
      {
        let start_idx: usize = (h + 4) as usize;
        let end_idx: usize = (main_vec.len()) as usize;

        for elm in &main_vec[start_idx..end_idx]
          { right_vec.push(elm.clone()); }

        main_vec.drain(start_idx..end_idx);
      }

      //println!("{:?}", main_vec);


      for (idx, elm) in main_vec.into_iter().enumerate()
      {
        println!("{:?}", elm.obj);

        /* if (idx as i64 == id) { print(true, elm.obj, elm.typeis, id); }
        else { print(false, elm.obj, elm.typeis, id); }

        match last_move
        {
          Move::Left => {
            println!("-LEFT");
          },
          Move::Right => {
            println!("-RIGHT");
          },
          Move::None => {
            println!("-NONE");
          },
        } */
      }

      println!("...");

    }
  }


  pub fn start(core: &Core, id: i64, last_move: Move)
  {
  	print!("\x1B[2J");
  	print!("\x1B[1;1H");
  	io::stdout().flush().unwrap();

    print_path(&core.curr_path);
    render(id, core.data.clone(), last_move);

  	/* for (idx, elm) in core.data.clone().into_iter().enumerate()
  	{
      if (idx as i64 == id) { print(true, elm.obj, elm.typeis, id); }
      else { print(false, elm.obj, elm.typeis, id); }
  	} */
  }
}
