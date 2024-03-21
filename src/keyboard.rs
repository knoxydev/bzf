pub mod keyboard_md
{
  use crate::print::print_md;
  use device_query::{DeviceQuery, DeviceState, Keycode};


  pub fn start()
  {
  	let device_state = DeviceState::new();
  	let mut prev_keys = vec![];
  	let mut x: i32 = 0;
	

  	loop
  	{
      let keys = device_state.get_keys();
      if keys != prev_keys && !keys.is_empty()
      {
        if keys.contains(&Keycode::Up)
        {
        	// IF x == 0 -> MOVE INDICATOR TO LAST ELEMENT OF ARR
        	// EL -> MOVE INDICATOR UPPER
        	x = if (x == 0) { (print_md::get_folder_length() as i32 - 1) }
        	else { x - 1 }
        }
        if keys.contains(&Keycode::Down)
        {
        	// IF x == 0 -> MOVE INDICATOR TO FIRST ELEMENT OF ARR
        	// EL -> MOVE INDICATOR LOWER
        	x = if (x == (print_md::get_folder_length() as i32) - 1) { 0 }
        	else { x + 1 }
        }
        if keys.contains(&Keycode::Right) { }
        if keys.contains(&Keycode::Q)
        {
        	println!("\n[EXIT]\n");
        	return;
        }

        print_md::start(x);
      }
      prev_keys = keys;
    }
  }
}
