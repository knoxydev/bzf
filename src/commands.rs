pub mod commands_md
{
  use std::path::PathBuf;
  use crate::state::state_md::Core;


  pub fn open_explorer(path: &PathBuf)
  {
    fn run(name: String, os: String, path: &PathBuf)
    {
      match std::process::Command::new(name).arg(path).spawn()
      {
        Ok(_) => println!("{:?} Explorer launched successfully!", os),
        Err(e) => eprintln!("\n[ERROR]: Failed to launch {:?} Explorer: {}\n", os, e),
      }
    }


    match std::env::consts::OS
    {
      "linux" =>
      {
        run("xdg-open".to_string(), "Linux".to_string(), path);
      },
      "macos" =>
      {
        run("open".to_string(), "MacOS".to_string(), path);
      },
      "windows" =>
      {
        run("explorer.exe".to_string(), "Windows".to_string(), path);
      },
      _ => println!("Unknown operating system"),
    }
  }


  pub fn change_path(path: &PathBuf)
  {
    fn run_command(name: String, path: &PathBuf)
    {
      /* match std::process::Command::new(name).arg("../../../").spawn()
      {
        Ok(_) => println!("Path changed successfully!"),
        Err(e) => eprintln!("\n[ERROR]: Failed to change a path > {:?}\n", e),
      } */

      if let Err(e) = std::env::set_current_dir(path) {
        eprintln!("Failed to change directory: {}", e);
        std::process::exit(1);
      }
    }


    if cfg!(target_os = "windows")
    {
      /* let output = Command::new("cd")
        .args(&["/C", path.to_str().expect("CAN'T CONVERT PathBuf to str")])
        .output()
        .expect("Failed to execute command");

      println!("{}", String::from_utf8_lossy(&output.stdout));
      println!("{:?}", view.core.curr_path); */

      run_command("cd".to_string(), path);

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
}
