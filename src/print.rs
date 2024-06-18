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
        Type::Directory => println!("{}{}> • {}{}{}", colors[3], id, elm, colors[3], colors[7]),
        Type::File => println!("{}{}>   {}{}{}", colors[3], id, elm, colors[3], colors[7]),
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


  pub fn win_size() -> (usize, usize)
  {
    extern crate term_size;

    if let Some((w, h)) = term_size::dimensions() { return (w, h); }
    else { return (0, 0); };
  }


  fn render(id: i64, vect: Vec<Info>, last_move: Move)
  {
    let mut main_vec: Vec<Info> = vect.clone();
    let size: (usize, usize) = win_size();
    let h: i64 = (size.1 as i64);

    println!("{:?}/{:?}", id, vect.len());

    for (idx, elm) in vect.clone().into_iter().enumerate()
    {
      if (vect.len() as i64) < h - 6
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


  /* fn render_two(id: i64, vect: Vec<Info>, last_move: Move)
  {
    let mut main_vec: Vec<Info> = vect.clone();
    let mut h: i64 = 0;
    {
      let size: (usize, usize) = win_size();
      h = (size.1 as i64) - 6;
    }

    println!("{:?}/{:?}", id + 1, vect.len());

    if main_vec.len() > h.try_into().unwrap()
    {
      if main_vec.len() > h as usize {
        right = main_vec.split_off(h as usize);
      }

      /* println!("{:?}", last_move); */

      match last_move
      {
        Move::Right => {},
        Move::Left => {},
        Move::Up =>
        {
          if let Some(last_left) = left.pop() {
            main_vec.insert(0, last_left);
          }

          if let Some(last_main) = main_vec.pop() {
            right.insert(0, last_main);
          }
        },
        Move::Down =>
        {
          if let Some(elm) = main_vec.get(0).cloned() {
            main_vec.remove(0);
            left.push(elm);
          }

          if let Some(elm) = right.get(0).cloned() {
            right.remove(0);
            main_vec.push(elm);
          }
        },
        Move::None => {},
      }

      for (idx, elm) in main_vec.clone().into_iter().enumerate()
      {
        if (idx as i64 == id) { print(true, elm.obj, elm.typeis, id); }
        else { print(false, elm.obj, elm.typeis, id); }
      }

      println!("...");

      println!("left: {:?}", left.len());
      println!("main_vec: {:?}", main_vec.len());
      println!("right: {:?}", right.len());
    }
    else
    {
      for (idx, elm) in main_vec.into_iter().enumerate()
      {
        println!("{:?}", elm);
      }
    }
  } */


  /* fn render_three(id: i64, vect: Vec<Info>, last_move: Move)
  {
    let mut main_vec: Vec<Info> = vect.clone();
    let mut left: Vec<Info> = Vec::new();
    let mut right_vec: Vec<Info> = Vec::new();
    let mut index: i64 = 0;

    let mut h: i64 = 0;
    {
      let size: (usize, usize) = win_size();
      h = (size.1 as i64) - 6;
    }

    println!("{:?}/{:?}", id + 1, vect.len());

    fn print_vector(vector: &Vec<Info>, start_index: usize)
    {
      let end_index = std::cmp::min(start_index + h, vector.len());
      for (i, item) in vector.iter().enumerate().skip(start_index).take(end_index - start_index) {
        println!("{}: {}", i, item.obj);
      }
    }

    match last_move
    {
      Move::Right => {},
      Move::Left => {},
      Move::Up => {
        if index > 0 {
          index -= 1;
        }
      },
      Move::Down =>
      {
        if (index as usize) + (h as usize) < main_vec.len().try_into().unwrap() {
          index += 1;
        }
      },
      Move::None => {},
    }

    print_vector(&main_vec, index.try_into().unwrap());
  } */


  fn render_new(core: &Core, main: &Vec<Info>, id: i64, h: i64)
  {
    println!("{:?}/{:?}", id + 1, core.data.len());

    if core.data.len() > (h as usize)
    {
      for (idx, elm) in main.clone().into_iter().enumerate()
      {
        if (idx as i64 == id) { print(true, elm.obj, elm.typeis, id + 1); }
        else { print(false, elm.obj, elm.typeis, id + 1); }
      }

      println!("...");
    }
    else
    {
      for (idx, elm) in main.clone().into_iter().enumerate()
      {
        if (idx as i64 == id) { print(true, elm.obj, elm.typeis, id + 1); }
        else { print(false, elm.obj, elm.typeis, id + 1); }
      }
    }
  }


  pub fn start(core: &Core, main: &Vec<Info>, id: i64, h: i64)
  {
  	print!("\x1B[2J");
  	print!("\x1B[1;1H");
  	io::stdout().flush().unwrap();

    print_path(&core.curr_path);
    //render_new(id, core.data.clone(), last_move, h);
    render_new(core, main, id, h)

  	/* for (idx, elm) in core.data.clone().into_iter().enumerate()
  	{
      if (idx as i64 == id) { print(true, elm.obj, elm.typeis, id); }
      else { print(false, elm.obj, elm.typeis, id); }
  	} */
  }
}
