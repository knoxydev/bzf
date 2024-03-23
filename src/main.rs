#![allow(warnings)]


// MODULES
mod init;
pub use crate::init::init_md;

mod keyboard;
pub use crate::keyboard::keyboard_md;

mod print;
mod state;


// PACKAGES
use std::env;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};


fn main()
{
  print::print_md::start(0 as i64);
  keyboard_md::start();
}
