pub mod keyboard_md
{
  use std::path::PathBuf;
  use crate::state::state_md::Core;
  use crate::print::print_md;
  use crate::state::state_md;
  use device_query::{DeviceQuery, DeviceState, Keycode};


  pub fn start()
  {
  	let device_state = DeviceState::new();
    let mut core = state_md::start();
  	let mut prev_keys = Vec::new();    
  	let mut x: i64 = 0;
    
    print_md::start(&core, 0 as i64);    


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
          let next_id: &String = &core.data[x as usize];
          let mut next_path = &mut core.curr_path;
          next_path.push(PathBuf::from(next_id));

          core.data = state_md::get(next_path.to_path_buf());
          core.curr_path = next_path.to_path_buf();

          x = 0;                
        }
        if keys.contains(&Keycode::Left)
        {
          let mut prev = core.curr_path.clone();
          prev.pop();

          core.curr_path = prev.to_path_buf();
          core.data = state_md::get(prev.to_path_buf());
          
          x = 0;
        }
        if keys.contains(&Keycode::Q)
        {  
          println!("{}", print_md::keycode_q("\n[EXIT]\n".to_string()));
          return;
        }
        print_md::start(&core, x);
      }
      prev_keys = keys;
    }
  }
}
