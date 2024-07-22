pub mod keyboard_md
{
  use std::path::PathBuf;
  use std::process::Command;
  use crate::state::state_md::{Core, Move, Info, View};
  use crate::print::print_md;
  use crate::state::state_md;
  use crate::commands::commands_md;


  use device_query::{DeviceQuery, DeviceState, Keycode};


  pub fn start()
  {
    let device_state = DeviceState::new();
  	let mut prev_keys = Vec::new();


    let get_size = || -> i64
    {
      let size: (usize, usize) = print_md::win_size();
      (size.1 as i64) - 8
    };


    let mut view = View
    {
      core: state_md::init(),
      win_size_h: get_size(),
      idx: 0,
    };


    print_md::start_view(&view);

    loop
    {
      let keys = device_state.get_keys();
      view.win_size_h = get_size();
      let core_len: i64 = view.core.data.len().try_into().unwrap_or(0);

      if keys.is_empty() {
        prev_keys = keys;
        continue;
      }
      if keys == prev_keys { continue; }


      if keys.contains(&Keycode::Up)
      {
        view.idx = if (view.idx == 0) { core_len - 1 }
        else { view.idx - 1 };
      }
      else if keys.contains(&Keycode::Down)
      {
        view.idx = if (view.idx == core_len - 1) { 0 }
        else { view.idx + 1 }
      }
      else if keys.contains(&Keycode::Right)
      {
        let next_id: &String = &view.core.data[view.idx as usize].obj;

        let mut next_path = &mut view.core.curr_path;
        next_path.push(PathBuf::from(next_id));

        let path_is: bool = state_md::check_type_path(&next_path);
        if path_is == true
        {
          view.core.data = state_md::get(next_path.to_path_buf());
          view.core.curr_path = next_path.to_path_buf();

          view.idx = 0;
        }
        else { view.core.curr_path.pop(); }
      }
      else if keys.contains(&Keycode::Left)
      {
        let mut prev_path: PathBuf = view.core.curr_path.clone();
        let temp_prev_path: PathBuf = view.core.curr_path.clone();
        prev_path.pop();

        view.core.curr_path = prev_path.to_path_buf();
        view.core.data = state_md::get(prev_path.to_path_buf());

        view.idx = state_md::prev_path_id(&temp_prev_path, &view.core.data);
      }
      else if keys.contains(&Keycode::Q)
      {
        let txt: String = format!("{}\n[EXIT]\n{}{}", print_md::colors[0], print_md::colors[0], print_md::colors[7]);
        println!("{}", txt);
        return;
      }
      else if keys.contains(&Keycode::Space)
      {
        if cfg!(target_os = "windows")
        {
          let output = Command::new("cmd")
            .args(&["/C", "cd", view.core.curr_path.to_str().expect("CAN'T CONVERT PathBuf to str")])
            .output()
            .expect("Failed to execute command");

          println!("{}", String::from_utf8_lossy(&output.stdout));
          println!("{:?}", view.core.curr_path);

          return;
        }
        else if cfg!(target_os = "linux")
        {

        }
        else if cfg!(target_os = "macos")
        {

        }
        else
        {

        }
      }
      else if keys.contains(&Keycode::O)
      {
        commands_md::open_explorer(&view.core.curr_path);
      }


      print_md::start_view(&view);
      prev_keys = keys.clone();
    }
  }
}
