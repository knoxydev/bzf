pub mod keyboard_md
{
  use std::path::PathBuf;
  use crate::state::state_md::{Core, Move};
  use crate::print::print_md;
  use crate::state::state_md;
  use crate::commands::commands_md;

  use device_query::{DeviceQuery, DeviceState, Keycode};
  use crossterm::event::{read, Event, KeyCode};


  pub fn start_new() -> std::io::Result<()>
  {
    let mut core = state_md::init();
  	let mut x: i64 = 0;

    print_md::start(&core, 0 as i64, Move::None);

    loop { match read()?
    {
      Event::Key(event) =>
      {
        if event.code == KeyCode::Up
        {
          // IF x == 0 -> MOVE INDICATOR TO LAST ELEMENT OF ARR
          // EL -> MOVE INDICATOR UPPER
          x = if (x == 0) { (state_md::len(&core.data) - 1) }
          else { x - 1 }
        }
        if event.code == KeyCode::Down
        {
          // IF x == 0 -> MOVE INDICATOR TO FIRST ELEMENT OF ARR
          // EL -> MOVE INDICATOR LOWER
          x = if (x == state_md::len(&core.data) - 1) { 0 }
          else { x + 1 }
        }
        if event.code == KeyCode::Right
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
        }
        if event.code == KeyCode::Left
        {
          let prev_clone: PathBuf = core.curr_path.clone();
          let mut prev: PathBuf = core.curr_path.clone();
          prev.pop();

          core.curr_path = prev.to_path_buf();
          core.data = state_md::get(prev.to_path_buf());

          x = state_md::prev_path_id(&prev_clone, &core.data);
        }
        if event.code == KeyCode::Esc
        {
          println!("{}", print_md::keycode_q("\n[EXIT]\n".to_string()));
          break;
        }

        print_md::start(&core, x, Move::None);
      },
      _ => continue,
    }}
    Ok(())
  }












  pub fn start()
  {
  	let device_state = DeviceState::new();
    let mut last_move: Move = Move::None;
    let mut core = state_md::init();
  	let mut prev_keys = Vec::new();
  	let mut x: i64 = 0;

    print_md::start(&core, 0 as i64, last_move);

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
        print_md::start(&core, x, last_move);
      }
      prev_keys = keys;
    }
  }
}
