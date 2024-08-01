pub mod keyboard_md
{
  use std::path::PathBuf;
  use crate::state::state_md::{Core, Info, View};
  use crate::print::print_md;
  use crate::state::state_md;
  use crate::commands::commands_md;

  use crossterm::event::{read, Event, KeyEvent, KeyEventKind, KeyCode};


  pub fn start() -> std::io::Result<()>
  {
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


    let mut check_key_press = |view: &mut View, event: KeyEvent, core_len: i64|
    {
      if event.kind == KeyEventKind::Release {
        match event.code
        {
          KeyCode::Up =>
          {
            view.idx = if (view.idx == 0) { core_len - 1 }
            else { view.idx - 1 };
          },

          KeyCode::Down =>
          {
            view.idx = if (view.idx == core_len - 1) { 0 }
            else { view.idx + 1 }
          },

          KeyCode::Right =>
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
          },

          KeyCode::Left =>
          {
            let mut prev_path: PathBuf = view.core.curr_path.clone();
            let temp_prev_path: PathBuf = view.core.curr_path.clone();
            prev_path.pop();

            view.core.curr_path = prev_path.to_path_buf();
            view.core.data = state_md::get(prev_path.to_path_buf());

            view.idx = state_md::prev_path_id(&temp_prev_path, &view.core.data);
          },

          KeyCode::Char('q') =>
          {
            let txt: String = format!("{}\n[EXIT]\n{}{}", print_md::colors[0], print_md::colors[0], print_md::colors[7]);
            println!("{}", txt);
            std::process::exit(0);
          },

          KeyCode::Char('o') => { commands_md::open_explorer(&view.core.curr_path); },
          _ => {},
        }
      }
    };


    loop
    {
      view.win_size_h = get_size();
      let core_len: i64 = view.core.data.len().try_into().unwrap_or(0);

      match read()?
      {
        Event::Key(event) => { check_key_press(&mut view, event, core_len); },
        _ => todo!(),
      }

      print_md::start_view(&view);
    }
    Ok(())
  }
}
