pub mod keyboard_md
{
  use crate::state::state_md::Core;
  use crate::print::print_md;
  use crate::state::state_md;
  use device_query::{DeviceQuery, DeviceState, Keycode};


  pub fn start()
  {
    let core = state_md::start();
  	let device_state = DeviceState::new();
  	let mut prev_keys = Vec::new();    
  	let mut x: i64 = 0;
	

  	loop
  	{
      let keys = device_state.get_keys();
      if keys != prev_keys && !keys.is_empty()
      {
        if keys.contains(&Keycode::Up)
        {
          // IF x == 0 -> MOVE INDICATOR TO LAST ELEMENT OF ARR
          // EL -> MOVE INDICATOR UPPER
          x = if (x == 0) { (core.len() - 1) }
          else { x - 1 }
        }
        if keys.contains(&Keycode::Down)
        {
          // IF x == 0 -> MOVE INDICATOR TO FIRST ELEMENT OF ARR
          // EL -> MOVE INDICATOR LOWER
          x = if (x == core.len() - 1) { 0 }
          else { x + 1 }
        }
        if keys.contains(&Keycode::Right)
        {
          //println!("{:?}", x);

          state_md::start();
          return;
        }
        if keys.contains(&Keycode::Q)
        {
          core.start();
          println!("\n[EXIT]\n");
          return;
        }

        print_md::start(x);
      }
      prev_keys = keys;
    }
  }
}
