pub mod commands_md
{
  use std::path::PathBuf;


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
      "linux" => run("xdg-open".to_string(), "Linux".to_string(), path),
      "macos" => run("open".to_string(), "MacOS".to_string(), path),
      "windows" => run("explorer.exe".to_string(), "Windows".to_string(), path),
      _ => println!("Unknown operating system"),
    }
  }
}
