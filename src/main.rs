#![allow(warnings)]


// MODULES
mod init;
pub use crate::init::init_md;

mod keyboard;
pub use crate::keyboard::keyboard_md;

mod commands;
mod print;
mod state;


fn main()
{
  keyboard_md::start();
}
