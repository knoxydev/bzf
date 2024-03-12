pub mod keyboard_md
{
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
        if keys.contains(&Keycode::Up) { x = if (x == 0) { 0 } else { x - 1 } }
        if keys.contains(&Keycode::Down) { x += 1; }
        if keys.contains(&Keycode::Right) { }

        crate::print::print_md::start(x);
      }
    	prev_keys = keys;
		}
		
	}
}
