pub mod print_md
{
  use crate::state::state_md::{Core, View, Type, Info, Move};
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


  // ---------------------------------------------------


  pub fn print_new(iter_elm: &String, iter_typeis: &Type, view_idx: i64, iter_idx: i64)
  {
    if (iter_idx as i64 == view_idx)
    {
      match iter_typeis {
        Type::Directory => println!("{}{}> • {}{}{}", colors[3], view_idx + 1, iter_elm, colors[3], colors[7]),
        Type::File => println!("{}{}>   {}{}{}", colors[3], view_idx, iter_elm, colors[3], colors[7]),
      }
    }
    else
    {
      match iter_typeis {
        Type::Directory => println!("   • {}", iter_elm),
        Type::File => println!("     {}", iter_elm),
      }
    }
  }


  pub fn render_last(view: &View)
  {
    let check_print_dots = || {

    };

    let core_len: i64 = view.core.data.len().try_into().unwrap_or(0);
    println!("{:?}/{:?}", view.idx + 1, core_len);


    if view.core.data.len() > view.win_size_h.try_into().unwrap()
    {
      let half_offset = (view.win_size_h - 1) / 2;

      let skip = if view.idx < half_offset {
        0
      } else {
        view.idx - half_offset
      };

      if skip > 0 { println!("..."); }

      view.core.data
        .iter()
        .enumerate()
        .skip((skip as usize).try_into().unwrap())
        .take(view.win_size_h as usize)
        .for_each(|(iter_idx, iter_elm)| {
          print_new(&iter_elm.obj, &iter_elm.typeis, view.idx, iter_idx as i64);
        });

      if &view.core.data.len() - (skip as usize) > view.win_size_h as usize { println!("..."); }
    }


    // RENDER NORMAL BLOCK
    if view.core.data.len() < view.win_size_h.try_into().unwrap()
    {
      for (idx, elm) in view.core.data.clone().into_iter().enumerate()
      {
        if (idx as i64 == view.idx) { print(true, elm.obj, elm.typeis, view.idx + 1); }
        else { print(false, elm.obj, elm.typeis, view.idx + 1); }
      }
    }
  }


  pub fn start_view(view: &View)
  {
    print!("\x1B[2J");
  	print!("\x1B[1;1H");

    print!(r"\E[2J\E[2H");

  	io::stdout().flush().unwrap();

    print_path(&view.core.curr_path);
    render_last(&view);
  }
}
