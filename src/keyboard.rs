pub mod keyboard_md
{
  use std::path::PathBuf;
  use crate::state::state_md::{Core, Move, Info};
  use crate::print::print_md;
  use crate::state::state_md;
  use crate::commands::commands_md;

  use device_query::{DeviceQuery, DeviceState, Keycode};


  /* pub fn start_classic()
  {
  	let device_state = DeviceState::new();
    let mut prev_keys = Vec::new();

    let mut left: Vec<Info> = Vec::new();
    let mut right: Vec<Info> = Vec::new();
    let mut core = state_md::init();
    let mut main = core.data.clone();
    let mut h: i64 = 0;
  	let mut x: i64 = 0;

    print_md::start(&core, &main, x, h);

  	loop
  	{
      let keys = device_state.get_keys();
      if keys != prev_keys && !keys.is_empty()
      {
        if keys.contains(&Keycode::Up)
        {
          // IF x == 0 -> MOVE INDICATOR TO LAST ELEMENT OF ARR
          // EL -> MOVE INDICATOR UPPER
          x = if (x == 0) { (state_md::len(&core.data) - 1) }
          else { x - 1 }
        }
        if keys.contains(&Keycode::Down)
        {
          // IF x == 0 -> MOVE INDICATOR TO FIRST ELEMENT OF ARR
          // EL -> MOVE INDICATOR LOWER
          x = if (x == state_md::len(&core.data) - 1) { 0 }
          else { x + 1 }
        }
        if keys.contains(&Keycode::Right)
        {
          let next_id: &String = &core.data[x as usize].obj;
          let mut next_path = &mut core.curr_path;
          next_path.push(PathBuf::from(next_id));

          let path_is: bool = state_md::check_type_path(&next_path);
          if path_is == true
          {
            core.data = state_md::get(next_path.to_path_buf());
            core.curr_path = next_path.to_path_buf();

            x = 0;
          }
          else { core.curr_path.pop(); }
        }
        if keys.contains(&Keycode::Left)
        {
          let prev_clone: PathBuf = core.curr_path.clone();
          let mut prev: PathBuf = core.curr_path.clone();
          prev.pop();

          core.curr_path = prev.to_path_buf();
          core.data = state_md::get(prev.to_path_buf());

          x = state_md::prev_path_id(&prev_clone, &core.data);
        }
        if keys.contains(&Keycode::Q)
        {
          println!("{}", print_md::keycode_q("\n[EXIT]\n".to_string()));
          return;
        }
        if keys.contains(&Keycode::O)
        {
          commands_md::open_explorer(&core.curr_path);
        }

        //if core.data.len() as i64 == x { x = core.data.len() as i64; }

        print_md::start(&core, &core.data, x, h);
        println!("main -> {:?}", core.data.len());
      }
      prev_keys = keys;
    }
  } */


/*   fn move_up(last_move: &mut Move,
            left: &mut Vec<Info>,
            main: &mut Vec<Info>,
            right: &mut Vec<Info>,
            core: &mut Core,
            h: &mut i64,
            x: &mut i64,
            id: &mut i64)
  {
    // WHEN CORE.VECTOR LESS THAN WINDOWS' HEIGHT
    if core.data.len() < h as usize
    {
      // NORMAL BLOCK
      x = if (x == 0) { (state_md::len(&core.data) - 1) }
      else { x - 1 };

      last_move = Move::None;
    }
    else
    {
      // WHEN CORE.VECTOR MORE THAN WINDOWS' HEIGHT
      if x == 0 {
        if left.len() != 0 {

          // MOVE LEFT.END > MAIN.FIRST
          if let Some(last_left) = left.pop() { &main.insert(0, last_left.clone()); }
          // MOVE MAIN.END > RIGHT.FIRST
          if let Some(last_main) = &main.pop() { right.insert(0, last_main.clone()); }

          last_move = Move::Up;
        }
        else
        {
          left.clear();
          main.clear();
          right.clear();

          let mut temp_vec: Vec<Info> = core.data.clone();
          temp_vec.reverse();

          // original first main, second left
          left = temp_vec[(h as usize)..].iter().rev().cloned().collect();
          main = temp_vec[..(h as usize)].iter().rev().cloned().collect();

          last_move = Move::Up;

          x = main.len() as i64 - 1;
        }
      } else { x -= 1; }
    }
  } */

/* move_up(
            &mut last_move,
            &mut left,
            &mut main,
            &mut right,
            &mut core,
            &mut h,
            &mut x,
            &mut id
          ); */





  pub fn start()
  {
  	let device_state = DeviceState::new();
  	let mut prev_keys = Vec::new();

    let mut last_move: Move = Move::None;
    let mut left: Vec<Info> = Vec::new();
    let mut right: Vec<Info> = Vec::new();
    let mut core = state_md::init();
    let mut main = core.data.clone();
    let mut h: i64 = 0;
  	let mut x: i64 = 0;
    let mut id: i64 = 0;


    let get_size = || -> i64
    {
      let size: (usize, usize) = print_md::win_size();
      (size.1 as i64) - 8
    };

    print_md::start(&core, &main, x, get_size(), last_move);

  	loop
  	{
      h = get_size();

      let keys = device_state.get_keys();
      if keys != prev_keys && !keys.is_empty()
      {
        if keys.contains(&Keycode::Up)
        {
          // WHEN CORE.VECTOR LESS THAN WINDOWS' HEIGHT
          if core.data.len() < h as usize
          {
            // NORMAL BLOCK
            x = if (x == 0) { (state_md::len(&core.data) - 1) }
            else { x - 1 };

            id = if (id == 0) { (state_md::len(&core.data) - 1) }
            else { id - 1 };

            last_move = Move::None;
          }
          else
          {
            // WHEN CORE.VECTOR MORE THAN WINDOWS' HEIGHT
            if x == 0 {
              if left.len() != 0 {

                // MOVE LEFT.END > MAIN.FIRST
                if let Some(last_left) = left.pop() { &main.insert(0, last_left.clone()); }
                // MOVE MAIN.END > RIGHT.FIRST
                if let Some(last_main) = &main.pop() { right.insert(0, last_main.clone()); }

                last_move = Move::Up;
              }
              else
              {
                left.clear();
                main.clear();
                right.clear();

                let mut temp_vec: Vec<Info> = core.data.clone();
                temp_vec.reverse();

                // original first main, second left
                left = temp_vec[(h as usize)..].iter().rev().cloned().collect();
                main = temp_vec[..(h as usize)].iter().rev().cloned().collect();

                last_move = Move::Up;

                x = main.len() as i64 - 1;
              }
            } else { x -= 1; }
          }

          /* id = if (id == 0) { (state_md::len(&core.data) - 1) }
          else { id - 1 };
          x = id; */
        }
        if keys.contains(&Keycode::Down)
        {
          if core.data.len() < h as usize {
            // NORMAL BLOCK
            x = if (x == state_md::len(&core.data) - 1) { 0 }
            else { x + 1 }
          }
          else {
            if main.len() - 1 == (x as usize) {
              if right.len() != 0 {
                if let Some(elm) = &main.get(0).cloned() {
                  &main.remove(0);
                  left.push(elm.clone());
                }

                if let Some(elm) = right.get(0).cloned() {
                  right.remove(0);
                  &main.push(elm);
                }
              }
              else
              {
                left.clear();
                main.clear();
                right.clear();

                right = core.data[(h as usize)..].to_vec();
                main = core.data[..(h as usize)].to_vec();

                last_move = Move::Down;

                x = 0;
              }
            } else {
              x += 1;
            }

          }

          /* id = if (id == state_md::len(&core.data) - 1) { 0 }
          else { id + 1 };
          x = id; */
        }
        if keys.contains(&Keycode::Right)
        {
          let next_id: &String = &core.data[x as usize].obj;
          let mut next_path = &mut core.curr_path;
          next_path.push(PathBuf::from(next_id));

          let path_is: bool = state_md::check_type_path(&next_path);
          if path_is == true
          {
            core.data = state_md::get(next_path.to_path_buf());
            core.curr_path = next_path.to_path_buf();

            x = 0;

            if core.data.len() > h as usize
            {
              left.clear();
              main.clear();
              right.clear();

              right = core.data[(h as usize)..].to_vec();
              main = core.data[..(h as usize)].to_vec();
            }
            else
            {
              left.clear();
              right.clear();
              main = core.data.clone();
            }
          }
          else { core.curr_path.pop(); }

          last_move = Move::Right;
        }
        if keys.contains(&Keycode::Left)
        {
          let prev_clone: PathBuf = core.curr_path.clone();
          let mut prev: PathBuf = core.curr_path.clone();
          prev.pop();

          core.curr_path = prev.to_path_buf();
          core.data = state_md::get(prev.to_path_buf());

          x = state_md::prev_path_id(&prev_clone, &core.data);

          if core.data.len() > h as usize
          {
            left.clear();
            main.clear();
            right.clear();

            right = core.data[(h as usize)..].to_vec();
            main = core.data[..(h as usize)].to_vec();
          }
          else
          {
            left.clear();
            right.clear();
            main = core.data.clone();
          }

          last_move = Move::Left;
        }
        if keys.contains(&Keycode::Q)
        {
          println!("{}", print_md::keycode_q("\n[EXIT]\n".to_string()));
          return;
        }
        if keys.contains(&Keycode::O)
        {
          commands_md::open_explorer(&core.curr_path);
        }

        print_md::start(&core, &main, x, h, last_move);
        println!("l > {:?} | m > {:?} | r > {:?} | core > {:?} | h > {:?} | x > {:?} | id > {:?}", left.len(), main.len(), right.len(), core.data.len(), h, x, id);
      }
      prev_keys = keys;
    }
  }


}
